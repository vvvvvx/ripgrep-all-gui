// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use dirs::home_dir;
use open::that;
use regex::Regex;
use ripgrepa_gui::myutils::*;
use std::env::consts::OS;
use std::io::{self, BufRead};
use tauri::window;
//use std::os;

//use std::os::linux::raw::stat;
use std::path::Path;
use std::process::Stdio;
use std::process::{Command, ExitStatus};
//use std::sync::{Arc, Mutex};
//use tauri::api::file;
// use tauri::api::dialog;
//use that_open;
//use onig::Regex;
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
    searchPattern: &str,    // 全文搜索模式
    searchPath: &str,       // 搜索路径
    filenamePattern: &str,  // 文件名模式特征
    regexMode: bool,        // 是否使用正则模式
    dispHitCount: bool,     // 是否显示匹配行数
    searchFilename: bool,   // 是否仅搜索文件名
    maxCount: u32,          // 最大匹配行数
    searchHidden: bool,     // 是否搜索隐藏文件
    maxDepth: u32,          // 最大搜索深度
    searchBinary: bool,     // 是否搜索二进制文件
    excludeNotCommon: bool, // 是否排除常见压缩文件
) {
    //let pattern = searchPattern.clone();
    let mut ptrn_str = String::new();
    let file_patrn_str = generate_filename_pattern(filenamePattern);
    let mut disp_hitcount_str = " ";
    let mut max_count_str = " ".to_string();
    let mut re = Regex::new(".*").unwrap();
    let mut search_hidden_str = " ";
    let max_depth_str = " -d ".to_string() + maxDepth.to_string().as_str() + " ";
    let mut search_binary_str = " ";
    let common_args = " -M 1000 --max-columns-preview ";
    let keywords: Vec<String> = searchPattern
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    //let mut exclude_not_common_str=" -g '!*.[zZ][iI][pP]' -g '!*.[rR][aA][rR]' -g '!*.gz' -g '!*.tgz' -g '!*.arj' -g '!*.7z' -g '!*.tar' -g '!*.bz2' -g '!*.tbz2' -g '!*.Z' -g '!*.lzh' -g '!*.ace' -g '!*.jar' -g '!*.zst' -g '!*.db' -g '!*.[mM][pP]4' -g '!*.avi' -g '!*.mkv' -g '!*.[mM][pP]3' -g '!*.[jJ][pP][gG]' -g '!*.[jJ][pP][eE][gG]' -g '!*.[bB][mM][pP]' -g '!*.[pP][nN][gG]' -g '!*.[gG][iI][fF]'  -g '!*.tiff' -g '!*.raw' -g '!*.svg' -g '!*.psd' -g '!*.eps' -g '!*.sqlite' ";
    let mut exclude_not_common_str=" -g !*.[zZ][iI][pP] -g !*.[rR][aA][rR] -g !*.gz -g !*.tgz -g !*.arj -g !*.7z -g !*.tar -g !*.bz2 -g !*.tbz2 -g !*.Z -g !*.lzh -g !*.ace -g !*.jar -g !*.zst -g !*.db -g !*.[mM][pP]4 -g !*.avi -g !*.mkv -g !*.[mM][pP]3 -g !*.[jJ][pP][gG] -g !*.[jJ][pP][eE][gG] -g !*.[bB][mM][pP] -g !*.[pP][nN][gG] -g !*.[gG][iI][fF]  -g !*.tiff -g !*.raw -g !*.svg -g !*.psd -g !*.eps -g !*.sqlite ";

    // 告知前端OS情况
    window.emit("get-os", OS.to_string()).unwrap();
    println!("OS:{}", OS);

    if regexMode {
        ptrn_str = " --engine=auto -e ".to_string() + searchPattern + " ";
    } else {
        if keywords.len() > 0 {
            ptrn_str = " -F ".to_string() + keywords[0].as_str() + " ";
        }
        // if keywords.len() >= 1 && keywords.len() <= 2 {
        //     ptrn_str = generate_patterns(searchPattern);
        // } else if keywords.len() > 2 {
        //     ptrn_str = generate_patterns(keywords[0].as_str());
        // }
    }

    if dispHitCount {
        disp_hitcount_str = " --count-matches ";
    }
    if maxCount > 0 {
        max_count_str = " -m ".to_string() + maxCount.to_string().as_str() + " ";
    }
    if searchHidden {
        search_hidden_str = " --hidden ";
    }
    if searchBinary {
        search_binary_str = " -a ";
    }
    if !excludeNotCommon || filenamePattern.trim().len() > 0 {
        exclude_not_common_str = " ";
    }
    let mut rga_str = " ".to_string()
        + common_args
        + max_count_str.as_str()
        + disp_hitcount_str
        + search_binary_str
        + search_hidden_str
        + max_depth_str.as_str()
        + ptrn_str.as_str()
        + searchPath.replace(" ", "\\ ").as_str() // 路径中可能有空格，需要转义
        + exclude_not_common_str
        + file_patrn_str.as_str();

    // if OS == "windows" {
    //     rga_str += " 2>nul";
    // } else {
    //     rga_str += " 2>/dev/null";
    // }

    if searchFilename {
        rga_str = " ".to_string()
            + file_patrn_str.as_str()
            + " --files "
            + search_hidden_str
            + max_depth_str.as_str()
            + searchPath.replace(" ", "\\ ").as_str();
    }
    rga_str += " --no-messages ";

    //替换单引号，windows不需要单引号，linux需要
    // if OS == "windows" {
    //     rga_str = rga_str.replace('\'', "");
    // }

    //rga_str = rga_str.replace('\'', "");
    println!("rga_str:{:?}", split_args(&rga_str));
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
        println!("regex:{}", re);
    }

    // let shell = if OS == "windows" {
    //     ("cmd", ["/C"])
    // } else {
    //     ("sh", ["-c"])
    // };

    // let file_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    std::thread::spawn(move || {
        // 如果是空格分隔的多关键字，则启用pip_search ，更新进度
        if keywords.len() > 1 && !regexMode {
            //let file_list = file_list.lock().unwrap();
            window
                .emit(
                    "progress",
                    Some("->[".to_string() + keywords[0].as_str() + "]... "),
                )
                .expect("Failed to send completed message");
        }
        let mut child = Command::new("rga")
            .args(split_args(&rga_str))
            //.args(rga_str.split_whitespace().collect::<Vec<&str>>())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");

        let mut file_list: Vec<String> = Vec::new();

        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            let mut pre_path = String::new();
            for line in reader.lines() {
                match line {
                    Ok(line) => {
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
                        // 如果是空格分隔的多关键字，则启用pip_search
                        if keywords.len() > 1 && !regexMode {
                            // 获取文件路径
                            let pathes: Vec<&str> = line.split(':').collect();
                            let mut path = String::new();
                            if pathes.len() == 1 {
                                path = pathes[0].to_string();
                            } else if pathes.len() >= 2 {
                                if OS == "windows" {
                                    path = pathes[0].to_string() + pathes[1];
                                } else {
                                    path = pathes[0].to_string();
                                }
                            } else {
                                continue;
                            }
                            if path != pre_path {
                                //let mut file_list = file_list.lock().unwrap();
                                file_list.push(path.clone());
                                pre_path = path;
                            }
                        } else {
                            window.emit("rg-output", line).unwrap();
                        }
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

        // 如果是空格分隔的多关键字，则启用pip_search
        if keywords.len() > 1 && !regexMode {
            let s = "->[".to_string()
                + keywords[0].as_str()
                + "]->("
                + file_list.len().to_string().as_str()
                + ")";

            emit_signal(window.clone(), "progress", s.as_str());

            if file_list.len() > 0 {
                let status = pip_search(
                    window.clone(),
                    keywords.to_vec(),
                    file_list.to_vec(),
                    child,
                    "-M 1000 -m 5".to_string(),
                );
                println!("Command finished with status: {}", status.code().unwrap());
                emit_completed_signal(window, status);
            }
        } else {
            let status = child.wait().expect("Command wasn't running");

            println!("Command finished with status: {}", status.code().unwrap());
            emit_completed_signal(window, status);
        }
    });
}
fn emit_completed_signal(window: tauri::Window, status: ExitStatus) {
    match status.code().unwrap() {
        0 | 1 => {
            emit_signal(window, "completed", "搜索完成!");
        }
        2 => {
            emit_signal(window, "completed", "搜索完成，已忽略无法打开的文件!");
        }
        127 => {
            emit_signal(window, "completed", "搜索命令rga不存在!");
        }
        _ => {
            emit_signal(window, "completed", "搜索失败!");
        }
    }
}
fn emit_signal(window: tauri::Window, signal: &str, message: &str) {
    window
        .emit(signal, Some(message.to_string()))
        .expect(format!("Failed to send {} message", signal).as_str());
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
    // if OS == "windows" {
    //     // 检查并安装 Scoop
    //     if !is_scoop_installed() {
    //         if let Err(e) = install_scoop() {
    //             eprintln!("Error installing Scoop: {}", e);
    //             return;
    //         }
    //     }

    //     // 检查并安装 rga
    //     if !is_rga_installed() {
    //         if let Err(e) = install_rga() {
    //             eprintln!("Error installing rga: {}", e);
    //             return;
    //         }
    //     }
    // }

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
