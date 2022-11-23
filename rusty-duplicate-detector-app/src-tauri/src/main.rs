use serde_json;
use std::path::PathBuf;
use std::{thread, time::Duration};
use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct PathFoundPayload {
    path: String,
}

#[derive(Clone, serde::Deserialize)]
struct StartWalkPayload {
    path: String,
}

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn do_walk<R: tauri::Runtime>(path: &str, manager: &impl Manager<R>) {
    println!("Walking {}", path);
    let resource_dir = PathBuf::from(path);
    let walker_rx = mediawalker::start_walking(&resource_dir);
    for received in walker_rx {
        match received.result {
            Ok(result) => {
                if result == true {
                    println!("Received path {}", received.path);
                    manager
                        .emit_all(
                            "path-found",
                            PathFoundPayload {
                                path: received.path,
                            },
                        )
                        .unwrap();
                    thread::sleep(Duration::from_millis(1000));
                }
            }
            Err(err) => eprintln!("Error {}", err),
        }
    }
    println!("All received");
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // let copy = app.manager.clone();
            let app_handle = app.handle();
            let _id = app.listen_global("walk-started", move |event| {
                let start_walk: StartWalkPayload =
                    serde_json::from_str(event.payload().unwrap()).unwrap();
                let path = start_walk.path;
                do_walk(&path, &app_handle);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
