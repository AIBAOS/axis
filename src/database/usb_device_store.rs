// USB 外部设备存储模块 — SQLite 持久化

use crate::database::pool::DbConnectionType;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsbDeviceRow {
    pub id: i64,
    pub name: String,
    pub device_type: String,
    pub capacity_bytes: i64,
    pub used_bytes: i64,
    pub mount_point: String,
    pub status: String,
    pub vendor: String,
    pub serial_number: String,
    pub connected_at: i64,
}

pub struct SqliteUsbDeviceRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteUsbDeviceRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    fn get_connection(&self) -> Result<Connection, String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        match &*guard {
            DbConnectionType::Sqlite(pool) => {
                Connection::open(&pool.path).map_err(|e| format!("Open failed: {}", e))
            }
            #[cfg(feature = "postgres")]
            DbConnectionType::Postgres(_) => Err("PostgreSQL not implemented".to_string()),
        }
    }

    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS usb_devices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                device_type TEXT NOT NULL DEFAULT 'flash',
                capacity_bytes INTEGER NOT NULL DEFAULT 0,
                used_bytes INTEGER NOT NULL DEFAULT 0,
                mount_point TEXT NOT NULL DEFAULT '',
                status TEXT NOT NULL DEFAULT 'connected',
                vendor TEXT NOT NULL DEFAULT '',
                serial_number TEXT NOT NULL DEFAULT '',
                connected_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_usb_devices_status ON usb_devices(status);
            CREATE INDEX IF NOT EXISTS idx_usb_devices_type ON usb_devices(device_type);
        "#).map_err(|e| format!("Init usb_devices table failed: {}", e))
    }

    const SELECT_COLS: &'static str = "id, name, device_type, capacity_bytes, used_bytes, \
        mount_point, status, vendor, serial_number, connected_at";

    fn row_to_usb_device(row: &rusqlite::Row<'_>) -> Result<UsbDeviceRow, rusqlite::Error> {
        Ok(UsbDeviceRow {
            id: row.get(0)?,
            name: row.get(1)?,
            device_type: row.get(2)?,
            capacity_bytes: row.get(3)?,
            used_bytes: row.get(4)?,
            mount_point: row.get(5)?,
            status: row.get(6)?,
            vendor: row.get(7)?,
            serial_number: row.get(8)?,
            connected_at: row.get(9)?,
        })
    }

    pub fn get_usb_devices(
        &self,
        status: Option<&str>,
        device_type: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<UsbDeviceRow>, u64), String> {
        let conn = self.get_connection()?;
        let offset = (page - 1) * per_page;

        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(st) = status {
            if st != "all" {
                conditions.push(format!("status = ?{}", param_values.len() + 1));
                param_values.push(Box::new(st.to_string()));
            }
        }
        if let Some(dt) = device_type {
            if dt != "all" {
                conditions.push(format!("device_type = ?{}", param_values.len() + 1));
                param_values.push(Box::new(dt.to_string()));
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM usb_devices {}", where_clause);
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();
        let total: u64 = conn.query_row(&count_sql, params_ref.as_slice(), |row| row.get(0))
            .map_err(|e| format!("Count failed: {}", e))?;

        let data_sql = format!(
            "SELECT {} FROM usb_devices {} ORDER BY connected_at DESC LIMIT ?{} OFFSET ?{}",
            Self::SELECT_COLS, where_clause, param_values.len() + 1, param_values.len() + 2,
        );
        param_values.push(Box::new(per_page as i64));
        param_values.push(Box::new(offset as i64));
        let params_ref2: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn.prepare(&data_sql).map_err(|e| format!("Prepare failed: {}", e))?;
        let rows = stmt.query_map(params_ref2.as_slice(), |row| Self::row_to_usb_device(row))
            .map_err(|e| format!("Query failed: {}", e))?;

        let devices: Vec<UsbDeviceRow> = rows.filter_map(|r| r.ok()).collect();
        Ok((devices, total))
    }

    pub fn get_usb_device_by_id(&self, id: i64) -> Result<Option<UsbDeviceRow>, String> {
        let conn = self.get_connection()?;
        let sql = format!("SELECT {} FROM usb_devices WHERE id = ?1", Self::SELECT_COLS);
        let mut stmt = conn.prepare(&sql).map_err(|e| format!("Prepare failed: {}", e))?;

        match stmt.query_row(params![id], |row| Self::row_to_usb_device(row)) {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    pub fn eject_device(&self, id: i64) -> Result<bool, String> {
        let conn = self.get_connection()?;
        let affected = conn.execute(
            "UPDATE usb_devices SET status = 'ejected' WHERE id = ?1 AND status = 'connected'",
            params![id],
        ).map_err(|e| format!("Eject failed: {}", e))?;
        Ok(affected > 0)
    }
}
