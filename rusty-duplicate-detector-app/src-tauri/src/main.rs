use std::path::PathBuf;
use std::sync::mpsc::channel;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn walk(path: &str) -> Vec<String> {
    println!("Walking {}", path);
    let resource_dir = PathBuf::from(path);
    let (tx, rx) = channel();
    let walker_rx = mediawalker::start_walking(&resource_dir);
    for received in walker_rx {
        let tx = tx.clone();
        match received.result {
            Ok(result) => {
                if result == true {
                    println!("Received path {}", received.path);
                    tx.send(received.path).unwrap();
                }
            }
            Err(err) => println!("{}", err),
        }
    }
    println!("All received");
    return rx.into_iter().collect();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![walk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
