#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "value")]
enum FileType {
    Dir,
    File,
    Symlink,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileEntry {
    file_name: String,
    file_path: String,
    file_type: FileType,
    file_size: u64,
}

fn _list_dir(path: &str) -> io::Result<Vec<FileEntry>> {
    let mut p = PathBuf::from(path);
    if p.is_relative() {
        p = env::current_dir()?;
    }
    let mut entries = vec![FileEntry {
        file_name: "..".to_string(),
        file_path: p
            .parent()
            .unwrap_or(Path::new("/"))
            .to_str()
            .unwrap()
            .to_string(),
        file_type: FileType::Dir,
        file_size: 0,
    }];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_type;
        if metadata.is_dir() {
            file_type = FileType::Dir;
        } else if metadata.is_file() {
            file_type = FileType::File;
        } else if metadata.is_symlink() {
            file_type = FileType::Symlink;
        } else {
            continue;
        };
        entries.push(FileEntry {
            file_name: entry.file_name().into_string().unwrap(),
            file_path: p.join(entry.file_name()).to_str().unwrap().to_string(),
            file_type,
            file_size: metadata.len(),
        });
    }
    Ok(entries)
}

#[tauri::command]
fn list_dir(path: &str) -> Vec<FileEntry> {
    match _list_dir(path) {
        Ok(r) => r,
        Err(e) => vec![FileEntry {
            file_name: format!("error: {}", e.to_string()),
            file_path: path.to_string(),
            file_type: FileType::Dir,
            file_size: 0,
        }],
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
