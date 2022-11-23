use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::{thread, time::Duration};
use tauri::{Manager, Window};

#[derive(Clone, serde::Serialize)]
struct PathReceived {
    path: String,
}

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn walk(path: &str, window: Window) -> Result<(), ()> {
    println!("Walking {}", path);
    let resource_dir = PathBuf::from(path);
    let walker_rx = mediawalker::start_walking(&resource_dir);
    for received in walker_rx {
        match received.result {
            Ok(result) => {
                if result == true {
                    println!("Received path {}", received.path);
                    window
                        .emit(
                            "path-received",
                            PathReceived {
                                path: received.path,
                            },
                        )
                        .unwrap();
                    thread::sleep(Duration::from_millis(1000));
                }
            }
            Err(err) => eprintln!("Error"),
        }
    }
    println!("All received");
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let id = main_window.listen("walk-started", |event| {
                println!("{:?}", event.payload().unwrap())
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![walk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
