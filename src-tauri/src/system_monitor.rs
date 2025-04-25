// // Create a new Tauri plugin in your src-tauri/src directory

// use tauri::{plugin::{Builder, TauriPlugin}, Runtime, State, AppHandle, Manager};
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::{Duration, Instant};

// // State to track system status
// #[derive(Default)]
// struct SystemState {
//     is_locked: Arc<Mutex<bool>>,
//     last_activity: Arc<Mutex<Instant>>,
// }

// // Function to check if Windows is locked (using Windows API)
// #[cfg(target_os = "windows")]
// fn is_windows_locked() -> bool {
//     false
// }

// #[cfg(not(target_os = "windows"))]
// fn is_windows_locked() -> bool {
//     false // Non-Windows platforms return false
// }

// // Function to get system idle time (Windows specific)
// #[cfg(target_os = "windows")]
// fn get_idle_time_ms() -> u64 {
//     use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
//     use windows::Win32::System::SystemServices::GetTickCount;
    
//     unsafe {
//         let mut last_input = LASTINPUTINFO {
//             cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
//             dwTime: 0,
//         };
        
//         if GetLastInputInfo(&mut last_input).as_bool() {
//             let current_tick = GetTickCount();
//             return (current_tick - last_input.dwTime) as u64;
//         }
//     }
    
//     0
// }

// #[cfg(not(target_os = "windows"))]
// fn get_idle_time_ms() -> u64 {
//     0 // Non-Windows platforms
// }

// // Tauri commands to expose to frontend
// #[tauri::command]
// fn is_system_locked(state: State<'_, SystemState>) -> bool {
//     *state.is_locked.lock().unwrap()
// }

// #[tauri::command]
// fn get_idle_time_seconds(state: State<'_, SystemState>) -> u64 {
//     let now = Instant::now();
//     let last = *state.last_activity.lock().unwrap();
//     now.duration_since(last).as_secs()
// }

// #[tauri::command]
// fn reset_idle_timer(state: State<'_, SystemState>) {
//     *state.last_activity.lock().unwrap() = Instant::now();
// }

// // Create the plugin
// pub fn init<R: Runtime>() -> TauriPlugin<R> {
//     Builder::new("system_monitor")
//         .setup(|app| {
//             // Initialize system state
//             let system_state = SystemState {
//                 is_locked: Arc::new(Mutex::new(false)),
//                 last_activity: Arc::new(Mutex::new(Instant::now())),
//             };
            
//             app.manage(system_state.clone());
            
//             // Start monitoring thread
//             let app_handle = app.app_handle();
//             let is_locked_clone = system_state.is_locked.clone();
            
//             thread::spawn(move || {
//                 let mut previous_locked_state = false;
                
//                 loop {
//                     // Check locked state
//                     let current_locked_state = is_windows_locked();
                    
//                     // If state changed
//                     if current_locked_state != previous_locked_state {
//                         *is_locked_clone.lock().unwrap() = current_locked_state;
//                         previous_locked_state = current_locked_state;
                        
//                         // Emit event to frontend
//                         app_handle.emit_all("system-lock-changed", current_locked_state).ok();
//                     }
                    
//                     // Check idle time
//                     let idle_time_ms = get_idle_time_ms();
//                     if idle_time_ms > 60000 { // 1 minute for example
//                         app_handle.emit_all("system-idle", idle_time_ms / 1000).ok();
//                     }
                    
//                     thread::sleep(Duration::from_secs(2));
//                 }
//             });
            
//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![
//             is_system_locked,
//             get_idle_time_seconds,
//             reset_idle_timer
//         ])
//         .build()
// }