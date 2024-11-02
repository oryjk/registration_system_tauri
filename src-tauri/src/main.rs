// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_directory_monitoring() {
    thread::spawn(|| {
        let path_to_watch_wechat = Path::new("F:\\抢票\\软件\\3.9.10.19(20240410003158)\\WeChat");
        let path_to_watch_plugins = Path::new("C:\\Users\\oryjk\\AppData\\Roaming\\Tencent\\WeChat\\XPlugin\\Plugins\\RadiumWMPF");

        // 每秒循环检查
        loop {
            check_and_delete_folder(path_to_watch_wechat, &["_tmp"]);
            check_and_delete_folder(path_to_watch_plugins, &["11279", "11275"]);

            // 休眠 1 秒
            thread::sleep(Duration::from_secs(1));
        }
    });
}

// 检查事件并处理特定条件
fn check_and_delete_folder(path_to_watch: &Path, folder_names: &[&str]) {
    if let Ok(entries) = fs::read_dir(path_to_watch) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if path.is_dir() && folder_names.contains(&file_name_str.as_ref()) {
                        // 尝试删除符合条件的目录
                        if let Err(e) = fs::remove_dir_all(&path) {
                            eprintln!("Failed to delete folder: {}", e);
                        } else {
                            println!("Deleted folder: {:?}", path);
                        }
                    }

                    for end_with in folder_names{
                        if file_name_str.ends_with(end_with) {
                            if let Err(e) = fs::remove_dir_all(&path) {
                                eprintln!("Failed to delete folder: {}", e);
                            } else {
                                println!("Deleted folder: {:?}", path);
                            }
                        }
                    }

                }
            }
        }
    }
}

fn main() {
    start_directory_monitoring();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
