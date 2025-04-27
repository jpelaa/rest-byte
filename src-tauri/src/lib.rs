// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Emitter;
use tokio::time::{sleep, Duration};
use tauri::Manager;
#[derive(Clone)]
struct IdleState {
    last_event_time: Arc<AtomicU64>,
    is_idle: Arc<AtomicBool>,
}

impl IdleState {
    fn new() -> Self {
        Self {
            last_event_time: Arc::new(AtomicU64::new(0)),
            is_idle: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start_listener(&self) {
        let last_event_time = Arc::clone(&self.last_event_time);
        tauri::async_runtime::spawn_blocking(move || {
            rdev::listen(move |_event: rdev::Event| {
                last_event_time.store(SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs()), Ordering::SeqCst);
            }).unwrap();
        });
    }

    fn start_checker(&self, app: tauri::AppHandle) {
        let is_idle = Arc::clone(&self.is_idle);
        let last_event_time = Arc::clone(&self.last_event_time);
        tauri::async_runtime::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
                let last = last_event_time.load(Ordering::SeqCst);
                let current_idle = now - last > 60;  // 5-minute idle threshold
                if current_idle != is_idle.load(Ordering::SeqCst) {
                    is_idle.store(current_idle, Ordering::SeqCst);
                    app.emit("idle_state_changed", current_idle).unwrap();
                }
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .manage(IdleState::new())
    .setup(|app: &mut tauri::App| {
        let idle_state = app.state::<IdleState>();
        idle_state.start_listener();
        idle_state.start_checker(app.handle().clone());
        Ok(())
    })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
