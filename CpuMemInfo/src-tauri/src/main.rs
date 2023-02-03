#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt, CpuExt};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Serialize, Deserialize)]
struct SystemInfo {
    memory_total: f32,
    memory_used: f32,
    cpu_used: f32,
	hostname: String
}

#[tauri::command]
fn system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    SystemInfo {
        memory_total: sys.total_memory() as f32 / (1024 * 1024 * 1024) as f32,
        memory_used: sys.used_memory() as f32 / (1024 * 1024 * 1024) as f32,
        cpu_used: sys.global_cpu_info().cpu_usage(),
		hostname: sys.host_name().unwrap(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
