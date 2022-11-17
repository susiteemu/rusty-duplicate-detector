use std::path::PathBuf;
use std::sync::mpsc::channel;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn walk(path: &str) -> Result<Vec<String>, String> {
    println!("Walking {}", path);
    let resource_dir = PathBuf::from(path);
    let mut results = Vec::new();
    let walker_rx = mediawalker::start_walking(&resource_dir);
    for received in walker_rx {
        match received.result {
            Ok(result) => {
                if result == true {
                    println!("Received path {}", received.path);
                    results.push(received.path)
                }
            }
            Err(err) => eprintln!("Error"),
        }
    }
    println!("All received");
    Ok(results)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![walk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
