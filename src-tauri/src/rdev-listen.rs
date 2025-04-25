use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use tauri::{AppHandle, Runtime};
use tauri::Emitter;


#[tauri::command]
fn start_listener<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    let running = Arc::new(AtomicBool::new(true));
    let _running_clone = running.clone();
    
    thread::spawn(move || {
        // This callback will be called for each event
        let callback = move |event: Event| {
            
            // Convert the event to a serializable format
            let event_data = match event.event_type {
                EventType::KeyPress(key) => {
                    format!("KeyPress: {:?}", key)
                }
                EventType::KeyRelease(key) => {
                    format!("KeyRelease: {:?}", key)
                }
                EventType::ButtonPress(button) => {
                    format!("ButtonPress: {:?}", button)
                }
                EventType::ButtonRelease(button) => {
                    format!("ButtonRelease: {:?}", button)
                }
                EventType::MouseMove { x, y } => {
                    format!("MouseMove: x={}, y={}", x, y)
                }
                EventType::Wheel { delta_x, delta_y } => {
                    format!("Wheel: dx={}, dy={}", delta_x, delta_y)
                }
            };
            
            // Emit an event to the frontend
            let _ = app_handle.emit("input-event", event_data);
            
        };
        
        // Start listening
        if let Err(_error) = listen(callback) {
            // let error_msg = format!("{:?}", error);
            // let _ = app_handle.emit("input-listener-error", error_msg);
        }
    });
    
    Ok(())
}

#[tauri::command]
fn stop_listener(running: tauri::State<Arc<AtomicBool>>) {
    running.store(false, Ordering::SeqCst);
}
