// 预置系统角色
// 使用：在数据库初始化后调用 init_system_roles()

use rusqlite::Connection;

/// 预置系统角色数据
/// admin - 管理员（全部权限）
/// user - 普通用户（基础读写权限）  
/// guest - 访客（只读权限）
pub fn init_system_roles(conn: &Connection) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs()) as i64;

    // 预置角色
    if let Err(e) = conn.execute_batch(&format!(r#"
        INSERT OR IGNORE INTO roles (id, name, description, created_at, updated_at)
        VALUES 
            (1, 'admin', '系统管理员 - 拥有全部权限', {now}, {now}),
            (2, 'user', '普通用户 - 拥有基本文件读写权限', {now}, {now}),
            (3, 'guest', '访客 - 仅拥有文件只读权限', {now}, {now})
    "#)) {
        eprintln!("Failed to init roles: {}", e);
    }

    // 预置权限
    if let Err(e) = conn.execute_batch(&format!(r#"
        INSERT OR IGNORE INTO permissions (id, name, description, resource, action, created_at, updated_at)
        VALUES 
            (1, 'file:read', '读取文件', 'file', 'read', {now}, {now}),
            (2, 'file:write', '写入文件', 'file', 'write', {now}, {now}),
            (3, 'file:delete', '删除文件', 'file', 'delete', {now}, {now}),
            (4, 'role:manage', '管理角色', 'role', 'manage', {now}, {now}),
            (5, 'permission:manage', '管理权限', 'permission', 'manage', {now}, {now})
    "#)) {
        eprintln!("Failed to init permissions: {}", e);
    }

    // role-id 到 permission-id 的映射
    let role_perms = vec![
        (1, vec![1, 2, 3, 4, 5]),  // admin: 全部权限
        (2, vec![1, 2, 3]),         // user: 文件读写删
        (3, vec![1]),               // guest: 只读
    ];

    for (role_id, perm_ids) in role_perms {
        for perm_id in perm_ids {
            let sql = format!(
                "INSERT OR IGNORE INTO roles_permissions (role_id, permission_id, assigned_at) VALUES ({}, {}, {})",
                role_id, perm_id, now
            );
            conn.execute_batch(&sql).ok();
        }
    }
}
