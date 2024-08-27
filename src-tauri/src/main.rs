// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use dirs::home_dir;
use open::that;
use regex::Regex;
use ripgrepa_gui::myutils::*;
use std::env::consts::OS;
use std::io::{self, BufRead};
//use std::os::linux::raw::stat;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
//use tauri::api::file;
// use tauri::api::dialog;
//use that_open;

use rfd::FileDialog;
// use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }
// #[tauri::command]
// fn search(searchKey: &str, searchPath: &str) -> String {
//     println!("search_key:{} search_path:{}", searchKey, searchPath);
//     let rga_str = "rga -l -g '*.docx' ".to_string() + searchKey + " " + searchPath + " 2>/dev/null";
//     let args = ["-c", &rga_str];
//     let output = Command::new("sh")
//         .args(&args)
//         .output()
//         .expect("Failed to execute rga command");
//     let output_str = String::from_utf8_lossy(&output.stdout);
//     return output_str.to_string();
// }

#[tauri::command]
fn goto_folder(folderPath: &str) {
    println!("folder_path:{}", folderPath);

    let dir_path = Path::new(folderPath);

    if !dir_path.is_dir() {
        if let Some(dir_path) = dir_path.parent() {
            match that(dir_path) {
                Ok(_) => println!("Directory opened successfully."),
                Err(err) => eprintln!("Failed to open directory: {}", err),
            }
        }
        return;
    }

    match that(dir_path) {
        Ok(_) => println!("Directory opened successfully."),
        Err(err) => eprintln!("Failed to open directory: {}", err),
    }
}

#[tauri::command]
fn open_folder_dialog() -> String {
    if let Some(path) = FileDialog::new().pick_folder() {
        return path.to_string_lossy().to_string();
    } else {
        return "No folder selected".to_string();
    }
}

// #[tauri::command]
// fn open_folder_dialog2() -> Option<String> {
//     let mut result = "No folder selected".to_string();
//     dialog::FileDialogBuilder::new()
//         .set_title("Select a folder")
//         .pick_folder(|path_buf| {
//             if let Some(path) = path_buf {
//                 return Some(path.to_string_lossy().to_string());
//             } else {
//                 return ();
//             }
//         });
//     Some(result)
// }

#[tauri::command]
fn run_rg_command(
    window: tauri::Window,
    searchPattern: &str,
    searchPath: &str,
    filenamePattern: &str,
    regexMode: bool,
    dispHitCount: bool,
    searchFilename: bool,
    maxCount: u32,
) {
    let mut ptrn_str = String::new();
    let file_patrn_str = generate_filename_pattern(filenamePattern);
    let mut disp_hitcount_str = " ";
    let mut max_count_str = " ".to_string();
    let mut re = Regex::new(".*").unwrap();

    if regexMode {
        ptrn_str = " -e '".to_string() + searchPattern + "' ";
    } else {
        ptrn_str = generate_patterns(searchPattern);
    }

    if dispHitCount {
        disp_hitcount_str = " --count-matches ";
    }
    if maxCount > 0 {
        max_count_str = " -m ".to_string() + maxCount.to_string().as_str();
    }
    let mut rga_str = "rga ".to_string()
        + max_count_str.as_str()
        + disp_hitcount_str
        + file_patrn_str.as_str()
        + ptrn_str.as_str()
        + searchPath;
    // + " 2>/dev/null";
    if OS == "windows" {
        rga_str += " 2>nul";
    } else {
        rga_str += " 2>/dev/null";
    }

    if searchFilename {
        rga_str = "rga ".to_string() + file_patrn_str.as_str() + " --files " + searchPath;
    }

    println!("Running command:{}", rga_str);
    // re用于文件名搜索的模式匹配
    if searchFilename && searchPattern.trim().len() > 0 {
        re = match Regex::new(searchPattern) {
            Ok(r) => r,
            Err(e) => {
                println!("Error compiling regex: {}", e);
                window
                    .emit(
                        "completed",
                        Some("执行结束：正则表达式解析错误!".to_string()),
                    )
                    .unwrap();
                return;
            }
        };
    }
    println!("regex:{}", re);

    let shell = if OS == "windows" {
        ("cmd", ["/C"])
    } else {
        ("sh", ["-c"])
    };
    std::thread::spawn(move || {
        let args = [shell.1[0], &rga_str];
        let mut child = Command::new(shell.0)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");
        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        //println!("line:{}", line);
                        if searchFilename {
                            //let dir_path = Path::new(line.as_str());
                            //let filename = dir_path.file_name().unwrap().to_str().unwrap();
                            //    println!("filename:{}", filename);
                            if let Some(filename) = Path::new(&line)
                                .file_name()
                                .and_then(|os_str| os_str.to_str())
                            {
                                if re.is_match(filename) {
                                    window.emit("rg-output", line).unwrap();
                                }
                                continue;
                            }
                        }
                        window.emit("rg-output", line).unwrap();
                    }
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
        }

        /*
        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            for chunk in reader.split(b'\n') {
                match chunk {
                    Ok(chunk) => {
                        let line = String::from_utf8_lossy(&chunk).to_string();
                        if searchFilename {
                            let dir_path = Path::new(&line);
                            if let Some(filename) = dir_path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    if re.is_match(filename_str) {
                                        window.emit("rg-output", line).unwrap();
                                    }
                                }
                            }
                            continue;
                        }
                        window.emit("rg-output", line).unwrap();
                    }
                    Err(err) => eprintln!("Error reading chunk: {}", err),
                }
            }
        }

         */
        let status = child.wait().expect("Command wasn't running");
        // 发送结束信息
        println!("Command finished with status: {}", status.code().unwrap());
        match status.code().unwrap() {
            0 => {
                window
                    .emit("completed", Some("搜索完成!".to_string()))
                    .expect("Failed to send completed message");
            }
            1 => {
                window
                    .emit("completed", Some("搜索完成,无匹配结果!".to_string()))
                    .expect("Failed to send completed message");
            }
            2 => {
                window
                    .emit(
                        "completed",
                        Some("搜索完成，已忽略无法打开的文件!".to_string()),
                    )
                    .expect("Failed to send completed message");
            }
            127 => {
                window
                    .emit("completed", Some("搜索命令rga不存在!".to_string()))
                    .expect("Failed to send completed message");
            }
            _ => {
                window
                    .emit("completed", Some("搜索失败!".to_string()))
                    .expect("Failed to send completed message");
            }
        }
        // window
        //     .emit("completed", Some(status.to_string()))
        //     .expect("Failed to send completed message");
    });
}
#[tauri::command]
fn open_file(filePath: &str) {
    println!("file_path:{}", filePath);
    if let Err(e) = that(filePath) {
        println!("Error opening file: {}", e);
    }
}

#[tauri::command]
fn get_home_dir() -> String {
    let home_dir = dirs::home_dir().unwrap();
    return home_dir.to_str().unwrap().to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            run_rg_command,
            open_file,
            open_folder_dialog,
            goto_folder,
            get_home_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
