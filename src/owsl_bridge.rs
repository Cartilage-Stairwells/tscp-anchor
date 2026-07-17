use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const OWSL_STATUS_PATH: &str = "~/.tscp/owsl_status.json";
const STATUS_MAX_AGE_SECONDS: u64 = 60;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct OWSLStatus {
    pub timestamp: f64,
    pub status: String,
    pub action: String,
    pub round: u64,
    pub bits_consumed: u64,
    pub bits_remaining: u64,
    pub anomalies: Vec<String>,
    pub frame_count: u64,
    pub window_start: f64,
    pub window_end: f64,
    pub checksum_valid: bool,
}

impl OWSLStatus {
    pub fn from_json(data: &str) -> Result<Self, String> {
        serde_json::from_str(data).map_err(|e| format!("JSON parse error: {}", e))
    }

    pub fn permits_verification(&self) -> bool {
        self.status != "CRITICAL" && self.action != "HALT"
    }

    pub fn is_fresh(&self, max_age_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        (now - self.timestamp) < max_age_seconds as f64
    }

    pub fn is_healthy(&self) -> bool {
        self.permits_verification()
            && self.checksum_valid
            && self.bits_remaining > 0
            && self.frame_count > 0
    }
}

pub struct OWSLBridgeReader {
    status_path: PathBuf,
    max_age_seconds: u64,
    last_read_status: Option<OWSLStatus>,
}

impl OWSLBridgeReader {
    pub fn new() -> Self {
        let path = shellexpand::tilde(OWSL_STATUS_PATH).into_owned();
        Self {
            status_path: PathBuf::from(path),
            max_age_seconds: STATUS_MAX_AGE_SECONDS,
            last_read_status: None,
        }
    }

    pub fn with_path(path: &str) -> Self {
        Self {
            status_path: PathBuf::from(path),
            max_age_seconds: STATUS_MAX_AGE_SECONDS,
            last_read_status: None,
        }
    }

    pub fn read_status(&mut self) -> Result<OWSLStatus, String> {
        if !self.status_path.exists() {
            return Err(format!(
                "OWSL status file not found: {}. Is the Python OWSL running?",
                self.status_path.display()
            ));
        }

        let data = fs::read_to_string(&self.status_path)
            .map_err(|e| format!("Failed to read OWSL status: {}", e))?;

        let status = OWSLStatus::from_json(&data)?;

        if !status.is_fresh(self.max_age_seconds) {
            return Err(format!(
                "OWSL status is stale (age > {}s). Last update: {}",
                self.max_age_seconds,
                status.timestamp
            ));
        }

        self.last_read_status = Some(status.clone());
        Ok(status)
    }

    pub fn permits_verification(&mut self) -> bool {
        match self.read_status() {
            Ok(status) => {
                let permits = status.permits_verification()
                    && status.checksum_valid
                    && status.bits_remaining > 0;

                if !permits {
                    eprintln!("[OWSL-Bridge] Verification BLOCKED: status={}, action={}, remaining={}",
                        status.status, status.action, status.bits_remaining);
                }
                permits
            }
            Err(e) => {
                eprintln!("[OWSL-Bridge] Verification BLOCKED (safe default): {}", e);
                false
            }
        }
    }

    pub fn last_status(&self) -> Option<&OWSLStatus> {
        self.last_read_status.as_ref()
    }

    pub fn get_anomalies(&mut self) -> Vec<String> {
        match self.read_status() {
            Ok(status) => status.anomalies,
            Err(_) => vec!["OWSL status unreadable".to_string()],
        }
    }
}

impl Default for OWSLBridgeReader {
    fn default() -> Self {
        Self::new()
    }
}

use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref OWSL_BRIDGE: Mutex<OWSLBridgeReader> = Mutex::new(OWSLBridgeReader::new());
}

pub fn owsl_permits_verification() -> bool {
    match OWSL_BRIDGE.lock() {
        Ok(mut bridge) => bridge.permits_verification(),
        Err(_) => {
            eprintln!("[OWSL-Bridge] Mutex poisoned — blocking verification");
            false
        }
    }
}

pub fn get_owsl_anomalies() -> Vec<String> {
    match OWSL_BRIDGE.lock() {
        Ok(mut bridge) => bridge.get_anomalies(),
        Err(_) => vec!["OWSL bridge mutex poisoned".to_string()],
    }
}

pub fn get_owsl_status() -> Result<OWSLStatus, String> {
    match OWSL_BRIDGE.lock() {
        Ok(mut bridge) => bridge.read_status(),
        Err(_) => Err("OWSL bridge mutex poisoned".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_test_status_json(status: &str, action: &str) -> String {
        format!(
            r#"{{"timestamp": {},"status": "{}","action": "{}","round": 42,"bits_consumed": 32,"bits_remaining": 96,"anomalies": [],"frame_count": 10,"window_start": 0.0,"window_end": 1000.0,"checksum_valid": true}}"#,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            status,
            action
        )
    }

    #[test]
    fn test_owsl_status_parse() {
        let json = create_test_status_json("SAFE", "COMMIT");
        let status = OWSLStatus::from_json(&json).unwrap();
        assert_eq!(status.status, "SAFE");
        assert!(status.permits_verification());
    }

    #[test]
    fn test_owsl_status_critical_blocks() {
        let json = create_test_status_json("CRITICAL", "HALT");
        let status = OWSLStatus::from_json(&json).unwrap();
        assert!(!status.permits_verification());
    }

    #[test]
    fn test_bridge_reader_permits_safe() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status_path = temp_dir.path().join("owsl_status.json");
        let json = create_test_status_json("SAFE", "COMMIT");
        let mut file = std::fs::File::create(&status_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut bridge = OWSLBridgeReader::with_path(status_path.to_str().unwrap());
        assert!(bridge.permits_verification());
    }

    #[test]
    fn test_bridge_reader_blocks_critical() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status_path = temp_dir.path().join("owsl_status.json");
        let json = create_test_status_json("CRITICAL", "HALT");
        let mut file = std::fs::File::create(&status_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.flush().unwrap();

        let mut bridge = OWSLBridgeReader::with_path(status_path.to_str().unwrap());
        assert!(!bridge.permits_verification());
    }

    #[test]
    fn test_bridge_reader_missing_file_blocks() {
        let mut bridge = OWSLBridgeReader::with_path("/nonexistent/owsl_status.json");
        assert!(!bridge.permits_verification());
    }
}
