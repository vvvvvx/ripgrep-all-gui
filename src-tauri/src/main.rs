// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use dirs::home_dir;
use open::that;
use regex::Regex;
use ripgrepa_gui::myutils::*;
use serde::Serialize;
use std::env::consts::OS;
use std::io::{self, BufRead, BufReader, Read};
use std::os::linux::raw::stat;
use tauri::window;
//use tauri::{window, EventLoopMessage};
//use std::os;

//use std::os::linux::raw::stat;
use chrono::{format, DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use filetime::FileTime;
use rfd::FileDialog;
use std::fs;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Child, Stdio};
use std::process::{Command, ExitStatus};
// use tauri::Manager;

//use std::sync::{Arc, Mutex};

// 创建一个全局变量来存储子进程的句柄
//static mut CHILD_PROCESS: Option<Arc<Mutex<Child>>> = None;

const OVER_DATE: Option<NaiveDate> = NaiveDate::from_ymd_opt(2026, 12, 30);

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

// 搜索命中结果
struct Record {
    hit_count: u32,
    path: String,
    content: String,
    created_at: String,
    modified_at: String,
}
#[derive(Clone)]
struct FileRecord {
    file_path: String,
    content: String,
}
#[tauri::command]
fn run_rg_command(
    window: tauri::Window,
    searchPattern: &str,    // 全文搜索模式
    mut searchPath: String, // 搜索路径
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
    // 判断软件是否过期
    let current_date = Local::now().naive_local().date();
    if current_date > OVER_DATE.unwrap() && OS != "linux" {
        window
            .emit("overdate", Some("软件已过期！\n请根据窗口右下方联系方式索取最新版，或到下面网址下载最新版：\n\nhttps://sourceforge.net/projects/fast-full-text-search/files/latest/download ".to_string()))
            .unwrap();
        return;
    }

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

    //if regexMode {
    //    println!("Regex:{}", re.to_string());
    //    ptrn_str = format!(" --engine=auto -e  {} ", searchPattern);
    //} else {
    //    if keywords.len() > 0 {
    //        ptrn_str = " -F ".to_string() + keywords[0].as_str() + " ";
    //    }
    //}

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

    // 检查路径是否是 C:或D:格式，如果是，则自动添加反斜杠
    let r = Regex::new(r"^[A-Za-z]:$").unwrap();
    if r.is_match(searchPath.as_str()) {
        searchPath = searchPath + "\\";
    }

    let mut rga_str = " ".to_string()
        + common_args
        + max_count_str.as_str()
        + disp_hitcount_str
        + search_binary_str
        + search_hidden_str
        + max_depth_str.as_str();

    //整理rga args
    let mut rga_args = split_args(&rga_str);

    // 处理搜索关键字
    if regexMode {
        println!("Regex:{}", re.to_string());
        rga_args.push("--engine=auto".to_string());
        rga_args.push("-e".to_string());
        rga_args.push(searchPattern.to_string());
    } else {
        if keywords.len() > 0 {
            rga_args.push("-F".to_string());
            rga_args.push(keywords[0].to_string());
        }
    }

    rga_args.append(&mut split_args(&exclude_not_common_str));
    rga_args.append(&mut split_args(&file_patrn_str));

    //如果仅搜索文件名，则重新制作参数，添加 --files 参数
    if searchFilename {
        //清空重新制作
        rga_args.clear();
        rga_args.append(&mut split_args(&file_patrn_str));
        rga_args.push("--files".to_string());
        rga_args.append(&mut split_args(&search_hidden_str));
        rga_args.append(&mut split_args(&max_depth_str));
        rga_args.push(searchPattern.to_string());
    }
    //rga_str += " --no-messages ";
    rga_args.push("--no-messages".to_string());

    // 路径中可能有空格，需要转义
    rga_args.push(searchPath);
    //rga_args.push(searchPath.replace(" ", "\\ "));

    //rga_str += searchPath.replace(" ", "\\ ").as_str(); // 路径中可能有空格，需要转义
    //替换单引号，windows不需要单引号，linux需要
    // if OS == "windows" {
    //     rga_str = rga_str.replace('\'', "");
    // }

    println!("rga_str:{:?}", rga_args);
    //println!("Running command:rga {}", rga_str);
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
        println!("Filename regex:{}", re);
    }

    std::thread::spawn(move || {
        // 如果是空格分隔的多关键字，则启用pip_search ，更新进度
        if keywords.len() > 1 && !regexMode {
            window
                .emit(
                    "progress",
                    Some("->[".to_string() + keywords[0].as_str() + "]... "),
                )
                .expect("Failed to send completed message");
        }
        #[cfg(not(windows))]
        let mut child = Command::new("rga")
            .args(rga_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");

        #[cfg(windows)]
        let mut child = Command::new("rga.exe")
            .args(rga_args)
            // windows下需要设置不显示命令行窗口
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");

        // if !output.status.success() {
        //     window
        //         .emit("error", String::from_utf8_lossy(&output.stderr).to_string())
        //         .unwrap();
        //     return;
        // }
        // let mut child = if OS == "windows" {
        //     Command::new("rga")
        //         .args(split_args(&rga_str))
        //         // windows下需要设置不显示命令行窗口
        //         .creation_flags(0x08000000) // CREATE_NO_WINDOW
        //         .stdout(Stdio::piped())
        //         .spawn()
        //         .expect("Failed to start rga command")
        // } else {
        //     Command::new("rga")
        //         .args(split_args(&rga_str))
        //         .stdout(Stdio::piped())
        //         .spawn()
        //         .expect("Failed to start rga command")
        // };

        // let mut child = match child_result {
        //     Ok(child) => child,
        //     Err(e) => {
        //         window
        //             .emit("error", format!("执行错误: {}", e).as_str())
        //             .unwrap();
        //         return;
        //     }
        // };
        //let status = child.wait();
        // let status = match status {
        //     Ok(status) => status,
        //     Err(e) => {
        //         println!("Error waiting for child process: {}", e);
        //         window
        //             .emit("completed", Some("执行结束：子进程错误!".to_string()))
        //             .unwrap();
        //         return;
        //     }
        // };

        println!("Child process started.");
        let mut file_list: Vec<FileRecord> = Vec::new();

        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            let mut pre_path = String::new();

            let mut record = Record {
                hit_count: 0,
                path: String::new(),
                content: String::new(),
                created_at: String::new(),
                modified_at: String::new(),
            };

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        //println!("Line: {}", line);
                        if searchFilename {
                            println!("Filename :{}", line);
                            if let Some(filename) = Path::new(&line)
                                .file_name()
                                .and_then(|os_str| os_str.to_str())
                            {
                                if re.is_match(filename) {
                                    let (created_at, modified_at) = get_filetime(line.as_str());
                                    window
                                        .emit(
                                            "rg-output",
                                            "1".to_string()
                                                + "~"
                                                + line.as_str()
                                                + "~"
                                                + created_at.as_str()
                                                + "~"
                                                + modified_at.as_str()
                                                + "~"
                                                + "-",
                                        )
                                        .unwrap();
                                }
                                continue;
                            }
                        }
                        // 如果是空格分隔的多关键字，则启用pip_search
                        if keywords.len() > 1 && !regexMode {
                            // 获取文件路径
                            // let pathes: Vec<&str> = line.split(':').collect();
                            // let mut path = String::new();

                            // if pathes.len() == 1 {
                            //     path = pathes[0].to_string();
                            // } else if pathes.len() >= 2 {
                            //     if OS == "windows" {
                            //         path = pathes[0].to_string() + pathes[1];
                            //     } else {
                            //         path = pathes[0].to_string();
                            //     }
                            // } else {
                            //     continue;
                            // }
                            let (path, content) = split_path_content(line.as_str());
                            if path != pre_path {
                                //let mut file_list = file_list.lock().unwrap();

                                //创建传递给pip_search的file_list
                                file_list.push(FileRecord {
                                    file_path: path.clone(),
                                    content: "——————————————————[".to_string()
                                        + keywords[0].as_str()
                                        + "]——————————————————\n"
                                        + content.as_str()
                                        + "\n",
                                });
                                pre_path = path;
                            }
                        } else {
                            let (path, content) = split_path_content(line.as_str());
                            if path.len() == 0 {
                                continue;
                            }
                            if path != pre_path {
                                // 向前端发送搜索结果,以'~'分割
                                if record.hit_count > 0 {
                                    window
                                        .emit(
                                            "rg-output",
                                            record.hit_count.to_string()
                                                + "~"
                                                + record.path.as_str()
                                                + "~"
                                                + record.created_at.as_str()
                                                + "~"
                                                + record.modified_at.as_str()
                                                + "~"
                                                + record.content.as_str(),
                                        )
                                        .unwrap();
                                }
                                // 重置记录
                                record = Record {
                                    hit_count: 0,
                                    path: String::new(),
                                    content: String::new(),
                                    created_at: String::new(),
                                    modified_at: String::new(),
                                };
                                record.hit_count += 1;
                                record.path = path.clone();
                                record.content = content + "\n";

                                let (created_at, modified_at) = get_filetime(path.as_str());

                                record.created_at = created_at;
                                record.modified_at = modified_at;
                                // 重置pre_path
                                pre_path = path.clone();
                            } else {
                                // 同一文件，追加内容
                                record.hit_count += 1;
                                // 限制content长度，防止前端卡死
                                if record.content.len() < 3000 {
                                    record.content = record.content + content.as_str() + "\n";
                                }
                            }
                        }
                    }
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
        }

        // 如果是空格分隔的多关键字，则启用pip_search
        if keywords.len() > 1 && !regexMode && file_list.len() > 0 {
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

            // 如果有错误输出，则输出到前端
            if !status.success() {
                match child.stderr.take() {
                    Some(stderr) => {
                        let reader = io::BufReader::new(stderr);
                        let mut lines = String::new();
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    println!("Error: {}", line);
                                    lines.push_str(line.as_str());
                                    lines.push('\n');
                                }
                                Err(err) => eprintln!("Error reading line: {}", err),
                            }
                        }
                        window.emit("error", lines.as_str()).unwrap();
                    }
                    None => {}
                }

                emit_completed_signal(window, status);
                return;
            }

            println!("Command finished with status: {}", status.code().unwrap());
            emit_completed_signal(window, status);
        }
        // 清除全局变量中的子进程句柄
        // unsafe {
        //     CHILD_PROCESS = None;
        // }
    });
}

// 用于三个及以上关键字的搜索采用管道过滤法，即前一个关键字的输出作为后一个关键字的输入
fn pip_search(
    window: tauri::Window,
    keywords: Vec<String>,
    mut file_list: Vec<FileRecord>,
    mut rg_process: Child,
    addtional_args: String,
) -> ExitStatus {
    // keywords从第二个元素开始，第一个元素已搜索
    if keywords.len() == 1 || file_list.len() == 0 {
        return rg_process
            .wait()
            .expect("Failed to pipe output from rga process");
    }
    // 针对每一个后续关键字进行过滤
    let keywords_len = keywords.len();
    let mut key = "->[".to_string()
        + keywords[0].as_str()
        + "]->("
        + file_list.len().to_string().as_str()
        + ")";

    for i in 1..keywords.len() {
        let mut next_file_list: Vec<FileRecord> = Vec::new();
        let keyword = keywords[i].clone();

        key = key.clone() + "->[" + keywords[i].as_str() + "]";
        // 输出当前关键字的进度信息
        window
            .emit("progress", Some(key.clone()))
            .expect("Failed to send completed message");

        for file in file_list {
            let file2 = file.clone();
            println!("在文件 {} 中搜索: {}", file.file_path, keywords[i]);
            // 对于每个文件，使用 rg 进行进一步的关键字过滤
            let mut rg_process = Command::new("rga")
                //.args(addtional_args.split_whitespace().collect::<Vec<&str>>())
                .args(split_args(&addtional_args))
                .arg(keyword.clone())
                .arg("--no-messages")
                .arg(file.file_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute rga process");

            let status = rg_process.wait().expect("Failed to execute rga process");
            if !status.success() {
                match rg_process.stderr.take() {
                    Some(stderr) => {
                        let reader = io::BufReader::new(stderr);
                        let mut lines = String::new();
                        for line in reader.lines() {
                            match line {
                                Ok(line) => {
                                    println!("Error: {}", line);
                                    lines.push_str(line.as_str());
                                    lines.push('\n');
                                }
                                Err(err) => eprintln!("Error reading line: {}", err),
                            }
                        }
                        eprintln!(
                            "Pip_srearch() running rga process Error : {}",
                            lines.as_str()
                        );
                    }
                    None => {}
                }

                eprintln!("pip_srearch() Error running rga process: {}", status);
                continue;
            }
            if let Some(stdout) = rg_process.stdout.take() {
                let reader = BufReader::new(stdout);

                // 如果输出结果不为空，则说明该文件包含当前关键字
                if let Some(next) = reader.lines().next() {
                    match next {
                        Ok(line) => {
                            //搜索完毕，输出结果
                            if i == keywords_len - 1 {
                                let (created_at, modified_at) =
                                    get_filetime(file2.file_path.as_str());
                                //let (_, content) = split_path_content(line.as_str());
                                window
                                    .emit(
                                        "rg-output",
                                        "1".to_string()
                                            + "~"
                                            + file2.file_path.as_str()
                                            + "~"
                                            + created_at.as_str()
                                            + "~"
                                            + modified_at.as_str()
                                            + "~"
                                            + file2.content.as_str()
                                            + "——————————————————["
                                            + keywords[i].as_str()
                                            + "]——————————————————\n"
                                            + line.as_str(),
                                    )
                                    .unwrap();
                            } else {
                                // 不是最后一个关键字，则将Filelist结果传递给下一个关键字
                                //let (_, content) = split_path_content(line.as_str());
                                next_file_list.push(FileRecord {
                                    file_path: file2.file_path.clone(),
                                    content: file2.content.clone()
                                        + "——————————————————["
                                        + keywords[i].as_str()
                                        + "]——————————————————\n"
                                        + line.as_str()
                                        + "\n",
                                });
                            }
                        }
                        Err(err) => eprintln!("Error reading line: {}", err),
                    }
                }
            }
        }

        // 更新文件列表为当前匹配的文件列表
        file_list = next_file_list;

        if file_list.len() > 0 {
            key = key.clone() + "->(" + file_list.len().to_string().as_str() + ")";
            window
                .emit("progress", Some(key.clone()))
                .expect("Failed to send completed message");
        } else {
            break;
        }
        // 输出最终过滤结果
    }
    rg_process
        .wait()
        .expect("Failed to pipe output from rga process")
}
//return (created_at, modified_at)
fn get_filetime(file_path: &str) -> (String, String) {
    let metadata = fs::metadata(file_path).expect("Failed to get metadata");
    let mut created_at = String::new();
    let mut modified_at = String::new();
    if let Some(created) = FileTime::from_creation_time(&metadata) {
        created_at = format_filetime(&created);
    } else {
        created_at = "".to_string();
    }
    modified_at = format_filetime(&FileTime::from_last_modification_time(&metadata));

    (created_at, modified_at)
}
fn format_filetime(filetime: &FileTime) -> String {
    let seconds = filetime.seconds();
    let nanos = filetime.nanoseconds();
    let datetime = Utc.timestamp_opt(seconds, nanos).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

//从stdout中分割出路径和匹配命中内容
fn split_path_content(line: &str) -> (String, String) {
    let pathes: Vec<&str> = line.split(':').collect();
    let mut path = String::new();
    let mut content = String::new();
    if pathes.len() == 1 {
        path = pathes[0].to_string();
    } else if pathes.len() >= 2 {
        //windows下，路径中盘符后一定有一个冒号，所以第二个冒号后才是content
        if OS == "windows" {
            path = pathes[0].to_string() + pathes[1];
            for i in 2..pathes.len() {
                content += pathes[i];
            }
        } else {
            //linux下，第一个冒号后是content
            path = pathes[0].to_string();
            for i in 1..pathes.len() {
                content += pathes[i];
            }
        }
    } else {
        return (String::new(), String::new());
    }

    (path.trim().to_string(), content.trim().to_string())
}
// 新增一个命令来终止子进程
// #[tauri::command]
// fn stop_rg_command(window: tauri::Window) {
//     unsafe {
//         if let Some(child_process) = CHILD_PROCESS.clone() {
//             let mut child = child_process.lock().unwrap();
//             if let Err(e) = child.kill() {
//                 println!("Failed to kill the child process: {}", e);
//                 window
//                     .emit("completed", Some("终止命令失败!".to_string()))
//                     .unwrap();
//             } else {
//                 println!("Child process terminated successfully.");
//                 window
//                     .emit("completed", Some("命令已终止!".to_string()))
//                     .unwrap();
//             }
//         } else {
//             println!("No child process to terminate.");
//             window
//                 .emit("completed", Some("没有正在运行的进程!".to_string()))
//                 .unwrap();
//         }
//     }
// }

fn emit_completed_signal(window: tauri::Window, status: ExitStatus) {
    match status.code().unwrap() {
        0 | 1 => {
            emit_signal(window, "completed", "搜索完成!");
        }
        2 => {
            emit_signal(window, "completed", "搜索完成!");
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
            get_home_dir,
            //stop_rg_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
