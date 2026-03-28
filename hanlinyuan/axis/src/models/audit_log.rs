use diesel::prelude::*;
use crate::schema::audit_logs;

#[derive(Queryable, Insertable)]
#[diesel(table_name = audit_logs)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: i32,
    pub action: String,
    pub resource: String,
    pub details: Option<String>,
    pub ip_address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog {
    pub user_id: i32,
    pub action: String,
    pub resource: String,
    pub details: Option<String>,
    pub ip_address: String,
}

pub fn create_log(
    conn: &mut SqliteConnection,
    new_log: NewAuditLog,
) -> Result<i32, diesel::result::Error> {
    diesel::insert_into(audit_logs::table)
        .values(&new_log)
        .execute(conn)?;
    Ok(0)
}

pub fn get_logs_by_user(
    conn: &mut SqliteConnection,
    user_id: i32,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    let offset = page.unwrap_or(1) * page_size.unwrap_or(20);
    audit_logs::table
        .filter(audit_logs::user_id.eq(user_id))
        .order(audit_logs::created_at.desc())
        .skip(offset as usize)
        .limit(page_size.unwrap_or(20) as i64)
        .load::<AuditLog>(conn)
}

pub fn get_all_logs(
    conn: &mut SqliteConnection,
    page: Option<i64>,
    page_size: Option<i64>,
) -> Result<Vec<AuditLog>, diesel::result::Error> {
    let offset = page.unwrap_or(1) * page_size.unwrap_or(20);
    audit_logs::table
        .order(audit_logs::created_at.desc())
        .skip(offset as usize)
        .limit(page_size.unwrap_or(20) as i64)
        .load::<AuditLog>(conn)
}
