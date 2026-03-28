// Backups handlers module
// 组织备份相关的 handler 函数

// 引用 handlers 模块下的文件
pub use crate::handlers::backups_list::list_backup_tasks;
pub use crate::handlers::backups_detail::get_backup_task_detail;
pub use crate::handlers::backups_create::create_backup;
pub use crate::handlers::backups_execute::run_backup;
pub use crate::handlers::backups_execute::execute_backup_task;
pub use crate::handlers::backups_update::update_backup;
pub use crate::handlers::backups_delete::delete_backup;
pub use crate::handlers::backups_archive::archive_backup;
pub use crate::handlers::backups_restore::restore_backup;
pub use crate::handlers::backups_execution_history::get_backup_execution_history;
pub use crate::handlers::backups_stats::get_backup_stats;
