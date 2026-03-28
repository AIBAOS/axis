// 网络共享存储实现（基于 SQLite）
use crate::database::pool::{DbConnectionType};
use rusqlite::{params, OptionalExtension};
use std::sync::{Arc, Mutex};

/// 网络共享模型
#[derive(Debug, Clone)]
pub struct Share {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub protocol: String, // "smb" or "nfs"
    pub status: String,   // "active" or "inactive"
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    // SMB 专用字段
    pub allowed_users: Option<String>,   // 逗号分隔的用户名
    pub allowed_groups: Option<String>,  // 逗号分隔的组名
    pub guest_ok: bool,                  // 是否允许访客访问
    pub read_only: bool,                 // 是否只读
    // NFS 专用字段
    pub comment: Option<String>,
    pub no_subtree_check: bool,
    pub sync: bool,
    pub clients: Option<String>,         // JSON 格式的客户端配置
    pub enabled: bool,
}

/// SQLite 网络共享存储实现
pub struct SqliteShareRepository {
    db: Arc<Mutex<DbConnectionType>>,
}

impl SqliteShareRepository {
    pub fn new(db: Arc<Mutex<DbConnectionType>>) -> Self {
        Self { db }
    }

    /// 从数据库连接获取连接
    fn get_connection(&self) -> Result<rusqlite::Connection, String> {
        let guard = self.db.lock().map_err(|e| format!("Lock failed: {}", e))?;
        match &*guard {
            DbConnectionType::Sqlite(pool) => {
                rusqlite::Connection::open(&pool.path)
                    .map_err(|e| format!("Open failed: {}", e))
            }
            #[cfg(feature = "postgres")]
            DbConnectionType::Postgres(_) => Err("PostgreSQL not implemented".to_string()),
        }
    }

    /// 初始化共享表
    pub fn init_tables(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS shares (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                protocol TEXT NOT NULL CHECK(protocol IN ('smb', 'nfs')),
                status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'inactive')),
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                allowed_users TEXT,
                allowed_groups TEXT,
                guest_ok INTEGER NOT NULL DEFAULT 0,
                read_only INTEGER NOT NULL DEFAULT 0,
                comment TEXT,
                no_subtree_check INTEGER NOT NULL DEFAULT 0,
                sync INTEGER NOT NULL DEFAULT 1,
                clients TEXT,
                enabled INTEGER NOT NULL DEFAULT 1
            )
            "#,
        ).map_err(|e| format!("Create table failed: {}", e))?;
        Ok(())
    }

    /// 获取共享详情（包含 SMB/NFS 字段）
    pub fn get_share_by_id(&self, id: u64) -> Result<Option<Share>, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, name, path, protocol, status, description, created_at, updated_at, allowed_users, allowed_groups, guest_ok, read_only, comment, no_subtree_check, sync, clients, enabled
             FROM shares WHERE id = ?1"
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let result = stmt.query_row(params![id], |row| {
            Ok(Share {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                protocol: row.get(3)?,
                status: row.get(4)?,
                description: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                allowed_users: row.get(8)?,
                allowed_groups: row.get(9)?,
                guest_ok: row.get::<_, i32>(10)? != 0,
                read_only: row.get::<_, i32>(11)? != 0,
                comment: row.get(12)?,
                no_subtree_check: row.get::<_, i32>(13)? != 0,
                sync: row.get::<_, i32>(14)? != 0,
                clients: row.get(15)?,
                enabled: row.get::<_, i32>(16)? != 0,
            })
        });

        match result {
            Ok(share) => Ok(Some(share)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Query failed: {}", e)),
        }
    }

    /// 获取所有共享（分页 + 筛选）
    pub fn get_shares(&self, page: u32, per_page: u32, protocol: Option<String>, status: Option<String>) -> Result<Vec<Share>, String> {
        let conn = self.get_connection()?;
        
        let offset = (page - 1) * per_page;
        let mut query = String::from(
            r#"
            SELECT id, name, path, protocol, status, description, created_at, updated_at, allowed_users, allowed_groups, guest_ok, read_only, comment, no_subtree_check, sync, clients, enabled
            FROM shares WHERE 1=1
            "#
        );
        
        if let Some(proto) = protocol {
            query.push_str(&format!(" AND protocol = '{}'", proto));
        }
        
        if let Some(st) = status {
            query.push_str(&format!(" AND status = '{}'", st));
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT ?1 OFFSET ?2");
        
        let mut stmt = conn.prepare(&query)
            .map_err(|e| format!("Prepare failed: {}", e))?;
        
        let shares = stmt
            .query_map(params![per_page, offset], |row| {
                Ok(Share {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    protocol: row.get(3)?,
                    status: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    allowed_users: row.get(8)?,
                    allowed_groups: row.get(9)?,
                    guest_ok: row.get::<_, i32>(10)? != 0,
                    read_only: row.get::<_, i32>(11)? != 0,
                    comment: row.get(12)?,
                    no_subtree_check: row.get::<_, i32>(13)? != 0,
                    sync: row.get::<_, i32>(14)? != 0,
                    clients: row.get(15)?,
                    enabled: row.get::<_, i32>(16)? != 0,
                })
            })
            .map_err(|e| format!("Query failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(shares)
    }

    /// 获取 SMB 共享列表（分页 + 筛选 + path 模糊搜索）
    pub fn get_smb_shares(&self, page: u32, per_page: u32, status: Option<String>, path: Option<String>) -> Result<Vec<Share>, String> {
        let conn = self.get_connection()?;
        
        let offset = (page - 1) * per_page;
        let mut query = String::from(
            r#"
            SELECT id, name, path, protocol, status, description, created_at, updated_at, allowed_users, allowed_groups, guest_ok, read_only, comment, no_subtree_check, sync, clients, enabled
            FROM shares WHERE protocol = 'smb'
            "#
        );
        
        let mut param_index = 1;
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        
        if let Some(st) = status {
            query.push_str(&format!(" AND status = ?{}", param_index));
            param_index += 1;
            params.push(Box::new(st));
        }
        
        if let Some(p) = path {
            query.push_str(&format!(" AND path LIKE ?{}", param_index));
            param_index += 1;
            params.push(Box::new(format!("%{}%", p)));
        }
        
        query.push_str(" ORDER BY created_at DESC LIMIT ?1 OFFSET ?2");
        params.push(Box::new(per_page as i64));
        params.push(Box::new(offset as i64));
        
        let params_ref: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        
        let mut stmt = conn.prepare(&query)
            .map_err(|e| format!("Prepare failed: {}", e))?;
        
        let shares = stmt
            .query_map(params_ref.as_slice(), |row| {
                Ok(Share {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    protocol: row.get(3)?,
                    status: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                    allowed_users: row.get(8)?,
                    allowed_groups: row.get(9)?,
                    guest_ok: row.get::<_, i32>(10)? != 0,
                    read_only: row.get::<_, i32>(11)? != 0,
                    comment: row.get(12)?,
                    no_subtree_check: row.get::<_, i32>(13)? != 0,
                    sync: row.get::<_, i32>(14)? != 0,
                    clients: row.get(15)?,
                    enabled: row.get::<_, i32>(16)? != 0,
                })
            })
            .map_err(|e| format!("Query failed: {}", e))?
            .filter_map(|r| r.ok())
            .collect();
        
        Ok(shares)
    }

    /// 统计共享数量（带筛选）
    pub fn count_shares(&self, protocol: Option<String>, status: Option<String>) -> Result<u64, String> {
        let conn = self.get_connection()?;
        
        let mut query = String::from("SELECT COUNT(*) FROM shares WHERE 1=1");
        
        if let Some(proto) = protocol {
            query.push_str(&format!(" AND protocol = '{}'", proto));
        }
        
        if let Some(st) = status {
            query.push_str(&format!(" AND status = '{}'", st));
        }
        
        let count: i64 = conn.query_row(&query, [], |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;
        
        Ok(count as u64)
    }

    /// 统计 SMB 共享数量（带筛选）
    pub fn count_smb_shares(&self, status: Option<String>, path: Option<String>) -> Result<u64, String> {
        let conn = self.get_connection()?;
        
        let mut query = String::from("SELECT COUNT(*) FROM shares WHERE protocol = 'smb'");
        
        if let Some(st) = status {
            query.push_str(&format!(" AND status = '{}'", st));
        }
        
        if let Some(p) = path {
            query.push_str(&format!(" AND path LIKE '%{}%'", p));
        }
        
        let count: i64 = conn.query_row(&query, [], |row| row.get(0))
            .map_err(|e| format!("Count query failed: {}", e))?;
        
        Ok(count as u64)
    }

    /// 创建共享（支持 SMB/NFS 字段）
    pub fn create_share(
        &self,
        name: &str,
        path: &str,
        protocol: &str,
        description: Option<&str>,
        allowed_users: Option<&str>,
        allowed_groups: Option<&str>,
        guest_ok: bool,
        read_only: bool,
        comment: Option<&str>,
        no_subtree_check: bool,
        sync: bool,
        clients: Option<&str>,
        enabled: bool,
    ) -> Result<Share, String> {
        let conn = self.get_connection()?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Invalid time")?
            .as_secs() as i64;

        let mut stmt = conn.prepare(
            r#"
            INSERT INTO shares (name, path, protocol, status, description, created_at, updated_at, allowed_users, allowed_groups, guest_ok, read_only, comment, no_subtree_check, sync, clients, enabled)
            VALUES (?1, ?2, ?3, 'active', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
            "#
        ).map_err(|e| format!("Prepare failed: {}", e))?;

        let id = stmt.execute(params![
            name,
            path,
            protocol,
            description,
            now,
            now,
            allowed_users,
            allowed_groups,
            if guest_ok { 1 } else { 0 },
            if read_only { 1 } else { 0 },
            comment,
            if no_subtree_check { 1 } else { 0 },
            if sync { 1 } else { 0 },
            clients,
            if enabled { 1 } else { 0 },
        ]).map_err(|e| format!("Insert failed: {}", e))?;

        Ok(Share {
            id: id as u64,
            name: name.to_string(),
            path: path.to_string(),
            protocol: protocol.to_string(),
            status: "active".to_string(),
            description: description.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
            allowed_users: allowed_users.map(|s| s.to_string()),
            allowed_groups: allowed_groups.map(|s| s.to_string()),
            guest_ok,
            read_only,
            comment: comment.map(|s| s.to_string()),
            no_subtree_check,
            sync,
            clients: clients.map(|s| s.to_string()),
            enabled,
        })
    }

    /// 更新共享
    pub fn update_share(&self, id: u64, name: Option<String>, path: Option<String>, protocol: Option<String>, status: Option<String>) -> Result<Share, String> {
        let share = self.get_share_by_id(id)?
            .ok_or_else(|| format!("Share {} not found", id))?;
        
        if share.status == "active" && status == Some("inactive".to_string()) {
            // 使用中禁删/停用（可选业务逻辑）
            // 暂时不阻塞，仅记录
        }
        
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Invalid time")?
            .as_secs() as i64;
        
        // 动态构建 UPDATE 语句
        let mut sets = vec![];
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];
        
        if let Some(ref n) = name {
            sets.push("name = ?1");
            params.push(Box::new(n));
        }
        if let Some(ref p) = path {
            sets.push("path = ?2");
            params.push(Box::new(p));
        }
        if let Some(ref pr) = protocol {
            sets.push("protocol = ?3");
            params.push(Box::new(pr));
        }
        if let Some(ref s) = status {
            sets.push("status = ?4");
            params.push(Box::new(s));
        }
        
        if sets.is_empty() {
            return Ok(share); // 没有更新内容
        }
        
        sets.push("updated_at = ?5");
        params.push(Box::new(now));
        
        let set_clause = sets.join(", ");
        let query = format!("UPDATE shares SET {} WHERE id = ?{}", set_clause, sets.len());
        
        let mut stmt = conn.prepare(&query)
            .map_err(|e| format!("Prepare failed: {}", e))?;
        
        stmt.execute(rusqlite::params_from_iter(params.iter()))
            .map_err(|e| format!("Update failed: {}", e))?;
        
        Ok(Share {
            id,
            name: name.clone().unwrap_or(share.name),
            path: path.clone().unwrap_or(share.path),
            protocol: protocol.clone().unwrap_or(share.protocol),
            status: status.clone().unwrap_or(share.status),
            description: share.description,
            created_at: share.created_at,
            updated_at: now,
            allowed_users: share.allowed_users,
            allowed_groups: share.allowed_groups,
            guest_ok: share.guest_ok,
            read_only: share.read_only,
            comment: share.comment,
            no_subtree_check: share.no_subtree_check,
            sync: share.sync,
            clients: share.clients,
            enabled: share.enabled,
        })
    }

    /// 更新 SMB 共享（支持 SMB 字段）
    pub fn update_share_smb(
        &self,
        id: u64,
        name: Option<&str>,
        path: Option<&str>,
        description: Option<&str>,
        allowed_users: Option<&str>,
        allowed_groups: Option<&str>,
        guest_ok: Option<bool>,
        read_only: Option<bool>,
    ) -> Result<Share, String> {
        let share = self.get_share_by_id(id)?
            .ok_or_else(|| format!("Share {} not found", id))?;

        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Invalid time")?
            .as_secs() as i64;

        // 动态构建 UPDATE 语句
        let mut sets = vec![];
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(n) = name {
            sets.push("name = ?1");
            params.push(Box::new(n));
        }
        if let Some(p) = path {
            sets.push("path = ?2");
            params.push(Box::new(p));
        }
        if let Some(d) = description {
            sets.push("description = ?3");
            params.push(Box::new(d));
        }
        if let Some(u) = allowed_users {
            sets.push("allowed_users = ?4");
            params.push(Box::new(u));
        }
        if let Some(g) = allowed_groups {
            sets.push("allowed_groups = ?5");
            params.push(Box::new(g));
        }
        if let Some(go) = guest_ok {
            sets.push("guest_ok = ?6");
            params.push(Box::new(if go { 1 } else { 0 }));
        }
        if let Some(ro) = read_only {
            sets.push("read_only = ?7");
            params.push(Box::new(if ro { 1 } else { 0 }));
        }

        sets.push("updated_at = ?8");
        params.push(Box::new(now));

        let set_clause = sets.join(", ");
        let query = format!("UPDATE shares SET {} WHERE id = ?{}", set_clause, sets.len());

        let mut stmt = conn.prepare(&query)
            .map_err(|e| format!("Prepare failed: {}", e))?;

        stmt.execute(rusqlite::params_from_iter(params.iter()))
            .map_err(|e| format!("Update failed: {}", e))?;

        let updated = self.get_share_by_id(id)?;

        Ok(updated.ok_or_else(|| "Update failed, share not found".to_string())?)
    }

    /// 删除共享
    pub fn delete_share(&self, id: u64) -> Result<bool, String> {
        let share = self.get_share_by_id(id)?
            .ok_or_else(|| format!("Share {} not found", id))?;
        
        if share.status == "active" {
            return Err("Share is active, cannot delete".to_string()); // 使用中禁删 409
        }
        
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM shares WHERE id = ?1", params![id])
            .map_err(|e| format!("Delete failed: {}", e))?;
        
        Ok(true)
    }

    /// 切换共享启用/禁用状态
    pub fn toggle_share(&self, id: u64) -> Result<Share, String> {
        let share = self.get_share_by_id(id)?
            .ok_or_else(|| format!("Share {} not found", id))?;
        
        let new_status = if share.status == "active" { "inactive" } else { "active" };
        let conn = self.get_connection()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Invalid time")?
            .as_secs() as i64;
        
        let mut stmt = conn.prepare(
            "UPDATE shares SET status = ?1, updated_at = ?2 WHERE id = ?3"
        ).map_err(|e| format!("Prepare failed: {}", e))?;
        
        stmt.execute(params![new_status, now, id])
            .map_err(|e| format!("Update failed: {}", e))?;
        
        Ok(Share {
            id,
            name: share.name,
            path: share.path,
            protocol: share.protocol,
            status: new_status.to_string(),
            description: share.description,
            created_at: share.created_at,
            updated_at: now,
            allowed_users: share.allowed_users,
            allowed_groups: share.allowed_groups,
            guest_ok: share.guest_ok,
            read_only: share.read_only,
            comment: share.comment,
            no_subtree_check: share.no_subtree_check,
            sync: share.sync,
            clients: share.clients,
            enabled: share.enabled,
        })
    }
}

impl Default for SqliteShareRepository {
    fn default() -> Self {
        panic!("SqliteShareRepository requires database connection");
    }
}
