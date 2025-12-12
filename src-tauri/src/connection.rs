use tauri::State;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected(String), // Active scheme
    Failed(String),
}

pub struct ConnectionManager {
    pub status: Mutex<ConnectionStatus>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(ConnectionStatus::Disconnected),
        }
    }
}

#[tauri::command]
pub async fn start_connection(
    token: String,
    state: State<'_, ConnectionManager>,
) -> Result<String, String> {
    {
        let mut status = state.status.lock().map_err(|_| "Lock error")?;
        *status = ConnectionStatus::Connecting;
    }

    // Level 1: P2P Hole Punching
    // Simulate check
    sleep(Duration::from_millis(800)).await;
    // Assume P2P failed for demo purposes

    // Level 2: Dynamic IP
    sleep(Duration::from_millis(500)).await;
    // Assume Success
    let success = true;

    if success {
        let mut status = state.status.lock().map_err(|_| "Lock error")?;
        *status = ConnectionStatus::Connected("DynamicIP".to_string());
        return Ok("Connected via Dynamic IP".to_string());
    }

    // Level 3: Relay (Fallback)
    
    {
        let mut status = state.status.lock().map_err(|_| "Lock error")?;
        *status = ConnectionStatus::Failed("All schemes failed".to_string());
    }
    
    Err("Failed to connect".to_string())
}

#[tauri::command]
pub fn get_connection_status(state: State<'_, ConnectionManager>) -> String {
    let status = state.status.lock().unwrap();
    match &*status {
        ConnectionStatus::Disconnected => "Disconnected".to_string(),
        ConnectionStatus::Connecting => "Connecting...".to_string(),
        ConnectionStatus::Connected(scheme) => format!("Connected ({})", scheme),
        ConnectionStatus::Failed(msg) => format!("Failed: {}", msg),
    }
}
