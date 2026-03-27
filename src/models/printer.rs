use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrinterStatusType {
    Idle,
    Printing,
    Paused,
    Error,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrinterStatus {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub queue_size: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Printer {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub location: String,
    pub model: String,
    pub ip_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintJob {
    pub id: u64,
    pub printer_id: u64,
    pub filename: String,
    pub status: String,
    pub created_at: String,
    pub pages: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PrinterListQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
