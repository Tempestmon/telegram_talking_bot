use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::task::JoinHandle;

struct ChatStateManager {
    timers: Mutex<HashMap<String, JoinHandle<()>>>,
    last_times: Mutex<HashMap<String, DateTime<Utc>>>,
}
