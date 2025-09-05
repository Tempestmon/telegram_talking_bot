use std::collections::HashMap;
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

struct ChatStateManager {
    timers: Mutex<HashMap<String, JoinHandle<()>>>,
    last_times: Mutex<HashMap<String, OffsetDateTime>>,
}

impl ChatStateManager {
    pub fn new() -> Arc<Self> {
        Arc::new(ChatStateManager {
            timers: Mutex::new(HashMap::new()),
            last_times: Mutex::new(HashMap::new()),
        })
    }

    pub async fn update_last_time(&self, chat_id: &str, time: OffsetDateTime) {
        let mut last_times = self.last_times.lock().await;
        last_times.insert(chat_id.to_string(), time);
    }

    pub async fn get_last_time(&self, chat_id: &str) -> Option<OffsetDateTime> {
        let last_times = self.last_times.lock().await;
        last_times.get(chat_id).cloned()
    }

    pub async fn schedule_timer<F>(&self, chat_id: &str, delay_secs: u64, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(timer) = self.timers.lock().await.remove(chat_id) {
            timer.abort();
        }

        let handle = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(delay_secs)).await;
        });

        self.timers.lock().await.insert(chat_id.to_string(), handle);
    }
}
