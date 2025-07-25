// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use dirs::home_dir;
use open::that;
use regex::Regex;
use ripgrepa_gui::myutils::*;
use serde::{Deserialize, Serialize};

use std::env::consts::OS;
use std::io::{self, BufRead, BufReader, Read};
use std::thread::sleep;
use std::time::Duration;
//use tauri::window;
//use tauri::{window, EventLoopMessage};
//use std::os;

//use std::os::linux::raw::stat;
use chrono::{Local, NaiveDate, TimeZone, Utc};
use filetime::FileTime;
use rfd::FileDialog;
use std::fs  ;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process:: Stdio;
use std::process::{Command, ExitStatus};
// use tauri::Manager;

//use std::sync::{Arc, Mutex};

// 创建一个全局变量来存储子进程的句柄
//static mut CHILD_PROCESS: Option<Arc<Mutex<Child>>> = None;

const OVER_DATE: Option<NaiveDate> = NaiveDate::from_ymd_opt(2026, 12, 30);
const PIP_SEARCH_MAX_HITS: usize = 3; // pip search每个关键字会记录的最大结果数
const MAX_CONTENT_SIZE: usize = 3000; // 命中记录Content字段最大长度
const MAX_RESULT_CACHE: usize = 100; // 命中结果缓存最大数量
const EMIT_INTERVAL: u64 = 2; // 前端消息最大推送间隔,秒
const COMMON_EXT:&str="*.docx *.pdf *.doc *.wps *.ppt *.pptx *.md *.odt *.rtf *.pages *.txt *.csv *.html *.htm *.xhtml *.xml  *.srt *.eml *.sub  *.tex";
//const COMMON_EXT:&str="*.docx *.pdf *.doc *.wps *.md *.odt *.rtf *.pages *.txt *.csv *.html *.htm *.xhtml *.xml *.epub *.srt *.eml *.sub *.sql *.mobi *.azw *.azw3 *.tex *.vtt";

#[tauri::command]
fn goto_folder(folder_path: &str) {
    println!("folder_path:{}", folder_path);

    let dir_path = Path::new(folder_path);

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
        path.to_string_lossy().to_string()
    } else {
        "No folder selected".to_string()
    }
}

fn split_path_arg(path_arg: &str) -> Vec<String> {

    // 把中文分号替换为英文分号
    //let  path_arg=path_arg.replace("；", ";");
    // 按英文分号分割路径
    // let mut path_vec:Vec<String>=path_arg.trim().split(';').map(String::from).collect();
    let mut path_vec:Vec<String>=path_arg.trim().split('|').map(|s| s.trim().to_string()).collect();

    //去重
    path_vec.sort();
    path_vec.dedup();

    if OS != "windows" {
        path_vec
    } else {
        let mut pathes=Vec::new();
        
        let r1 = Regex::new(r"^[A-Za-z]:$").unwrap();
        let r2 = Regex::new(r"^[A-Za-z]$").unwrap();
        let r3 = Regex::new(r"^[A-Za-z]:[^\\]").unwrap();

        for mut path in path_vec {
            if r1.is_match(path.clone().as_str()) {
                path+="\\"; 
            }
            if r2.is_match(path.clone().as_str()) {
                path+=":\\"; 
            }
            if r3.is_match(path.clone().as_str()) {
                path=path.replace(":", ":\\"); 
            }
            pathes.push(path);
        }
        pathes
    }

}

// 搜索命中结果
#[derive(Clone, Serialize)]
struct Record {
    hit_count: u32,
    file: String,
    created_at: String,
    modified_at: String,
    content: String,
}
impl Record {
    fn clear(&mut self) {
        self.hit_count = 0;
        self.file.clear();
        self.content.clear();
        self.created_at.clear();
        self.modified_at.clear();
    }
}
#[derive(Clone)]
struct FileRecord {
    file_path: String,
    content: String,
}
#[derive(Clone, Debug)]
struct PipKeywordRecord {
    keyword: String,
    hits: usize,
}
#[tauri::command]
async fn run_rg_command(
    window: tauri::Window,
    search_pattern: &str,    // 全文搜索模式
    search_path: String, // 搜索路径
    filename_pattern: &str,  // 文件名模式特征
    regex_mode: bool,        // 是否使用正则模式
    disp_hit_count: bool,     // 是否显示匹配行数
    search_filename: bool,   // 是否仅搜索文件名
    max_count: u32,          // 最大匹配行数
    search_hidden: bool,     // 是否搜索隐藏文件
    max_depth: u32,          // 最大搜索深度
    search_binary: bool,     // 是否搜索二进制文件
    //excludeNotCommon: bool, // 是否排除常见压缩文件
    search_all: bool, // 是否搜索所有文件,但不包含隐藏和二进制文件
    max_column: u32,  // 匹配结果最大显示长度，超过将被省略显示
    raw_code_mode:bool, //是否原始代码搜索模式
) -> Result<(), String> {
    // 判断软件是否过期
    let current_date = Local::now().naive_local().date();
    if current_date > OVER_DATE.unwrap() && OS != "linux" {
        window
            .emit("overdate", Some("软件已过期！\n请根据窗口右下方联系方式获取最新版".to_string()))
            .unwrap();
        return Ok(());
    }

    //let pattern = searchPattern.clone();
    //let ptrn_str = String::new();
    let file_patrn_str = generate_filename_pattern(filename_pattern);
    let mut disp_hitcount_str = " ";
    let mut max_count_str = " ".to_string();
    let mut re = Regex::new(".*").unwrap();
    let mut search_hidden_str = " ";
    let max_depth_str = " -d ".to_string() + max_depth.to_string().as_str() + " ";
    let mut search_binary_str = " ";
    let mut pip_keyword_records: Vec<PipKeywordRecord> = Vec::new();

    // -i 忽略大小写
    // --glob-case-insensitive 忽略文件名大小写
    let common_args = " -i --trim --max-columns-preview --glob-case-insensitive ";
    let keywords: Vec<String> = search_pattern
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    //let mut exclude_not_common_str=" -g !*.[zZ][iI][pP] -g !*.[rR][aA][rR] -g !*.gz -g !*.tgz -g !*.arj -g !*.7z -g !*.tar -g !*.bz2 -g !*.tbz2 -g !*.Z -g !*.lzh -g !*.ace -g !*.jar -g !*.zst -g !*.db -g !*.[mM][pP]4 -g !*.avi -g !*.mkv -g !*.[mM][pP]3 -g !*.[jJ][pP][gG] -g !*.[jJ][pP][eE][gG] -g !*.[bB][mM][pP] -g !*.[pP][nN][gG] -g !*.[gG][iI][fF]  -g !*.tiff -g !*.raw -g !*.svg -g !*.psd -g !*.eps -g !*.sqlite ";

    // 告知前端OS情况
    window.emit("get-os", OS.to_string()).unwrap();
    println!("OS:{}", OS);

    if disp_hit_count {
        disp_hitcount_str = " --count-matches ";
    }
    if max_count > 0 {
        max_count_str = " -m ".to_string() + max_count.to_string().as_str() + " ";
    }
    if search_hidden {
        search_hidden_str = " --hidden ";
    }
    if search_binary {
        search_binary_str = " -a ";
    }

    // if !excludeNotCommon || filenamePattern.trim().len() > 0 {
    //     exclude_not_common_str = " ";
    // }

    // 检查路径是否是 C:或D:格式，如果是，则自动添加反斜杠
    // let r = Regex::new(r"^[A-Za-z]:$").unwrap();
    // if r.is_match(search_path.as_str()) {
    //     search_path +=  "\\";
    // }
    

    let rga_str = " ".to_string()
        + common_args
        + max_count_str.as_str()
        + disp_hitcount_str
        + search_binary_str
        + search_hidden_str
        + max_depth_str.as_str();

    //整理rga args
    let mut rga_args = split_args(&rga_str);

    // 处理搜索关键字
    if regex_mode {
        println!("Regex:{}", re);
        rga_args.push("--engine=auto".to_string());
        rga_args.push("-e".to_string());
        rga_args.push(search_pattern.to_string());
    } else if !keywords.is_empty() {
            rga_args.push("-F".to_string());
            rga_args.push(keywords[0].to_string());
        
    }
    rga_args.push("-M".to_string());
    rga_args.push(max_column.to_string());

    // 处理filename pattern
    // 如果如果不是搜索所有格式，则按用户指定文件扩展名或常用扩展名搜索，否则搜索所有格式，即不指定文件扩展名
    if !search_all {
        // 指定了文件名模式，则只搜索匹配的文件名
        if !filename_pattern.trim().is_empty() {
            rga_args.append(&mut split_args(&file_patrn_str));
        } else {
            // 否则搜索所有常用格式
            rga_args.append(&mut split_args(
                generate_filename_pattern(COMMON_EXT).as_str(),
            ));
        }
    }
    //rga_args.append(&mut split_args(&exclude_not_common_str));
    //rga_args.append(&mut split_args(&file_patrn_str));

    //如果仅搜索文件名，则重新制作参数，添加 --files 参数
    if search_filename {
        //清空重新制作
        rga_args.clear();

        rga_args.append(&mut split_args(&file_patrn_str));
        rga_args.push("--files".to_string());
        rga_args.append(&mut split_args(search_hidden_str));
        rga_args.append(&mut split_args(&max_depth_str));
        rga_args.push(search_pattern.to_string());
        rga_args.push("-i".to_string());
        rga_args.push("--glob-case-insensitive".to_string());
    }
    rga_args.push("--no-messages".to_string());

    // 加入搜索路径, 多个路径用空格分隔
    // let pathes: Vec<String> = search_path
    //     .split_whitespace()
    //     .map(|s| s.to_string())
    //     .collect();
    rga_args.append(&mut split_path_arg(search_path.as_str()));

    println!("rga args:{:?}", rga_args);
    // re用于文件名搜索的模式匹配
    if search_filename && !search_pattern.trim().is_empty() {
        re = match Regex::new(search_pattern) {
            Ok(r) => r,
            Err(e) => {
                println!("Error compiling regex: {}", e);
                window
                    .emit(
                        "completed",
                        Some("执行结束：正则表达式解析错误!".to_string()),
                    )
                    .unwrap();
                return Ok(());
            }
        };
        println!("Filename regex:{}", re);
    }
    use std::time::Instant;
    let start = Instant::now();

    let mut emit_start=Instant::now();

    //如是原来代码模式，则直接调用rg命令，否则调用rga命令
    let cmd= if raw_code_mode { "rg" } else { "rga" };

    //std::thread::spawn(move || {
    tokio::task::spawn_blocking(move || {
        let is_pip_search = keywords.len() > 1 && !regex_mode;
        // 如果是空格分隔的多关键字，则启用pip_search ，更新进度
        if is_pip_search {
            window
                .emit(
                    "progress",
                    Some("[管道搜索]->[".to_string() + keywords[0].as_str() + "]..."),
                )
                .expect("Failed to send completed message");
        }
        #[cfg(not(windows))]
        let mut child = Command::new(cmd)
            .args(rga_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");

        #[cfg(windows)]
        let mut child = Command::new(cmd)
            .args(rga_args)
            // windows下需要设置不显示命令行窗口
            .creation_flags(0x08000000) // CREATE_NO_WINDOW
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start rga command");

        println!("Child process started.");
        let mut file_list: Vec<FileRecord> = Vec::new();

        if let Some(stdout) = child.stdout.take() {
            //let reader = io::BufReader::new(stdout);
            let reader = io::BufReader::with_capacity(1024 * 1024, stdout);
            let mut pre_path = String::new(); // 前一个文件路径
            let mut pre_content = String::new(); // 前一个文件内容
            let mut pip_content_count = 0; //pip_search时，前一文件累计content追加预览数
                                           //初始化pip_keyword_records

            if is_pip_search {
                for keywd in keywords.clone() {
                    pip_keyword_records.push(PipKeywordRecord {
                        keyword: keywd,
                        hits: 0,
                    });
                }
            }
            let mut record = Record {
                hit_count: 0,
                file: String::new(),
                content: String::new(),
                created_at: String::new(),
                modified_at: String::new(),
            };
            let mut result_records: Vec<Record> = Vec::new();
            let mut first_line = true;

                ////////////////////////
            //         let mut buffer_size=MAX_RESULT_CACHE;
            //         let mut buffer = String::new();

            // let start1 = Instant::now();
            //         let _ = reader.read_to_string(&mut buffer);

            // let duration1 = start1.elapsed();
            // println!("To String Time taken: {:?}", duration1);
            //         // 如果结果不多，就逐条发送
            //         if buffer.lines().count() <=200 {
            //             buffer_size=1;
            //         }

            for line in reader.lines() {
            //for line in buffer.lines() {
                ///////////////////////////////
                match line {
                    Ok(line) => {
                        //println!("Line: {}", line);
                        if search_filename {
                            //println!("Filename :{}", line);

                            // if let Some(filename) = Path::new(&line)
                            //     .file_name()
                            //     .and_then(|os_str| os_str.to_str())
                            let path = Path::new(&line);
                            if let Some(filename) =
                                path.file_name().and_then(|os_str| os_str.to_str())
                            {
                                if re.is_match(filename) {
                                    let (created_at, modified_at) = get_filetime(&line);

                                    record.hit_count = 1;
                                    record.file = line.to_string();
                                    record.content = "文件名匹配".to_string();
                                    record.created_at = created_at;
                                    record.modified_at = modified_at;

                                    result_records.push(record.clone());
                                    // 如果命中结果缓存满了就发送
                                    if result_records.len() >= MAX_RESULT_CACHE || emit_start.elapsed() >= Duration::from_secs(EMIT_INTERVAL) {
                                        window.emit("rg-output", &result_records).unwrap();
                                        result_records.clear();
                                        emit_start=Instant::now();
                                    } 
                                    
                                    record.clear();
                                }
                                
                                continue;
                            }
                        }
                        // 如果是空格分隔的多关键字，则启用pip_search
                        if is_pip_search {
                            let (path, content) = split_path_content(&line);
                            if first_line {
                                pre_path = path.clone();
                                pre_content = "[1] ".to_string() + &content.clone() + "\n";
                                pip_content_count = 1;
                                //first_line = false;
                            }
                            if path != pre_path {
                                //把新文件加入file_list
                                file_list.push(FileRecord {
                                    file_path: pre_path,
                                    content: "————————————————".to_string()
                                        + keywords[0].as_str()
                                        + "["
                                        + &pip_content_count.to_string()
                                        + "]————————————————\n"
                                        + &pre_content
                                            .replace("omitted end of long line", "行尾过长略...")
                                            .replace(
                                                "Omitted long matching line",
                                                "...省略过长匹配行...",
                                            )
                                        + "\n",
                                });
                                // 第一关键字记录数更新
                                pip_keyword_records[0].hits +=  1;
                                // 更新进度到前端
                                pip_progress_update(window.clone(), &pip_keyword_records, 0);
                                // 追加了一次命中，重置变量
                                pip_content_count = 1;
                                pre_path = path;
                                pre_content ="[1] ".to_string() + &content.clone() + "\n";
                            } else {
                                // 避免第一个文件内容重复
                                if first_line {
                                    first_line = false;
                                    continue;
                                }
                                pip_content_count +=  1;
                                // 同一文件，最多追加3个命中内容
                                if pip_content_count <= PIP_SEARCH_MAX_HITS {
                                    pre_content =
                                        //pre_content.clone() + content.clone().as_str() + "\n";
                                        pre_content.clone() +"["+&pip_content_count.to_string()+"] "+ &content + "\n";
                                }
                            }
                        } else {
                            //正常全文搜索

                            //println!("Line: {}", line.as_str());
                            let (path, content) = split_path_content(&line);
                            if path.is_empty() {
                                continue;
                            }
                            // 如果是新文件，则记录
                            if path != pre_path {
                                // 向前端发送搜索结果
                                if record.hit_count > 0 {
                                    //先替换英文提示字符
                                    record.content = record
                                            .content
                                            .replace("omitted end of long line", "行尾过长略...")
                                            .replace(
                                                "Omitted long matching line",
                                                "...省略过长匹配行...",
                                            );
                                    result_records.push(record.clone());
                                    //if record.content.len() < MAX_CONTENT_SIZE {
                                    // 如果命中结果缓存满了就发送，否则继续缓存
                                    if result_records.len() >= MAX_RESULT_CACHE || emit_start.elapsed() >= Duration::from_secs(EMIT_INTERVAL) {
                                       
                                        window.emit("rg-output", &result_records).unwrap();
                                        result_records.clear();
                                        emit_start=Instant::now();
                                    } 
                                    
                                }
                                // 重置记录
                                record.clear();
                                // 记录命中
                                record.hit_count +=  1;
                                record.file = path.clone();
                                record.content = "[1] ".to_string() +&content + "\n";

                                let (created_at, modified_at) = get_filetime(&path);

                                record.created_at = created_at;
                                record.modified_at = modified_at;
                                // 重置pre_path
                                pre_path = path.clone();
                            } else {
                                // 同一文件，追加内容
                                record.hit_count += 1;
                                // 限制content长度，防止前端卡死
                                if record.content.len() < MAX_CONTENT_SIZE {
                                    record.content = record.content +"["+&record.hit_count.to_string()+"] "+ &content + "\n";
                                }
                            }
                            
                        }
                    }
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
            // 循环结束，处理最后一个文件
            // 如果是pip_search
            if is_pip_search && !pre_path.trim().is_empty() {
                //把最后一个文件加入file_list
                file_list.push(FileRecord {
                    file_path: pre_path,
                    content: "————————————————".to_string()
                        + keywords[0].as_str()
                        + "["
                        + pip_content_count.to_string().as_str()
                        + "]————————————————\n"
                        + &pre_content
                            .replace("omitted end of long line", "行尾过长略...")
                            .replace("Omitted long matching line", "...省略过长匹配行...")
                        + "\n",
                });
                pip_keyword_records[0].hits += 1;
            }
            // 处理普通全文搜索和文件名搜索的最后一个文件
            if record.hit_count > 0 {
                record.content = record
                    .content
                    .replace("omitted end of long line", "行尾过长略...")
                    .replace("Omitted long matching line", "...省略过长匹配行...");
                result_records.push(record.clone());
            }
            // 发送缓存剩下的结果
            if !result_records.is_empty() {
                window.emit("rg-output", &result_records).unwrap();
                result_records.clear();
            }
        }

        // 如果是空格分隔的多关键字，则启用pip_search
        if is_pip_search && !file_list.is_empty() {

            //if !file_list.is_empty() {
                //let status=child.wait().expect("Command wasn't running");
                //let status =
                pip_search(
                    window.clone(),
                    keywords.to_vec(),
                    file_list.to_vec(),
                    //child,
                    cmd,
                    "-M ".to_string()
                        + max_column.to_string().as_str()
                        + " -m "
                        + max_count.to_string().as_str()
                        + " -i --trim ",
                    pip_keyword_records.clone(),
                );
                //println!("Command finished with status: {}", status.code().unwrap());
                //emit_completed_signal(window.clone(), status);
            //}
        } //else {
            let status = child.wait().expect("Command wasn't running");

            // 如果有错误输出，则输出到前端
            if !status.success() {
                if let Some(stderr) = child.stderr.take() {
                        let mut reader = io::BufReader::new(stderr);
                        let mut buffer = String::new();
                        let _ = reader.read_to_string(&mut buffer);
                        if !buffer.trim().is_empty() {
                            window.emit("error", buffer.as_str()).unwrap();
                        }
                }

                if is_pip_search && file_list.is_empty() {
                    pip_progress_update(window.clone(), &pip_keyword_records, 0);
                }
                emit_completed_signal(window, status);
                return;
            }

            println!("Command finished with status: {}", status.code().unwrap());
            emit_completed_signal(window, status);
        //}
    })
    .await
    .map_err(|e| e.to_string())?;
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
fn pip_progress_update(
    window: tauri::Window,
    //pip_keyword_records: &Vec<PipKeywordRecord>,
    pip_keyword_records: &[PipKeywordRecord],
    index: usize,
) {
    let mut s = "[管道搜索]".to_string();
    //println!("pip_keywords_records: {:?}", pip_keyword_records);
    //for i in 0..index + 1 {
    for record in pip_keyword_records.iter().take(index + 1) {
        s = s.clone()
            + "->["
            +record.keyword.as_str()
            + "]->("
            + record.hits.to_string().as_str()
            + ")";
        if record.hits == 0 {
            break;
        }
    }
    emit_signal(window.clone(), "progress", s.as_str());
}

// 用于三个及以上关键字的搜索采用管道过滤法，即前一个关键字的输出作为后一个关键字的输入
fn pip_search(
    window: tauri::Window,
    keywords: Vec<String>,
    mut file_list: Vec<FileRecord>,
    //mut rg_process: Child,
    cmd:&str,// rga or rg
    addtional_args: String,
    mut pip_keyword_records: Vec<PipKeywordRecord>,
) 
//-> ExitStatus
 {
    // keywords从第二个元素开始，第一个元素已搜索
    // if keywords.len() == 1 || file_list.is_empty() {
    //     return rg_process
    //         .wait()
    //         .expect("Failed to pipe output from rga process");
    // }
    // 针对每一个后续关键字进行过滤
    let keywords_len = keywords.len();
  
    for i in 1..keywords.len() {
        let mut next_file_list: Vec<FileRecord> = Vec::new();
        let keyword = keywords[i].clone();
        let mut result_records: Vec<Record> = Vec::new();
       

        for file in file_list {
            let file2 = file.clone();
            //println!("在文件 {} 中搜索: {}", file.file_path, keywords[i]);
            // 对于每个文件，使用 rg 进行进一步的关键字过滤
            #[cfg(windows)]
            let mut rg_process = Command::new(cmd)
                //.args(addtional_args.split_whitespace().collect::<Vec<&str>>())
                .args(split_args(&addtional_args))
                .arg(keyword.clone())
                .arg("--no-messages")
                .arg(file.file_path)
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute rga process");
            #[cfg(not(windows))]
            let mut rg_process = Command::new(cmd)
                //.args(addtional_args.split_whitespace().collect::<Vec<&str>>())
                .args(split_args(&addtional_args))
                .arg(keyword.clone())
                .arg("--no-messages")
                .arg(file.file_path)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute rga process");
            if let Some(stdout) = rg_process.stdout.take() {
                let reader = BufReader::with_capacity(1024 * 64, stdout); // new(stdout);
                                                                          //总命中行数
                let mut total_lines = 0;
                //前3行内容
                let mut top3_lines: Vec<String> = Vec::new();

                for line_result in reader.lines() {
                    match line_result {
                        Ok(line) => {
                            total_lines += 1;
                            if total_lines <= PIP_SEARCH_MAX_HITS {
                                top3_lines.push(
                                    "[".to_string() + &total_lines.to_string() + "] " + &line,
                                );
                            }
                        }
                        Err(err) => eprintln!("Error reading line: {}", err),
                    }
                }

                // 如果输出结果不为空，则说明该文件包含当前关键字
                //if let Some(next) = reader.lines().next() {
                if total_lines > 0 {
                    let lines_str = top3_lines.join("\n");
                    //match next {
                    //Ok(line) => {
                    //搜索完毕，输出结果
                    if i == keywords_len - 1 {
                        let (created_at, modified_at) = get_filetime(&file2.file_path);

                        result_records.push(Record {
                            hit_count: total_lines as u32,
                            file: file2.file_path.clone(),
                            created_at: created_at,
                            modified_at: modified_at,
                            content: file2.content.clone()
                                + "————————————————"
                                + &keywords[i]
                                + "["
                                + &total_lines.to_string()
                                + "]———————————————\n"
                                + &lines_str
                                    .replace("omitted end of long line", "行尾过长略...")
                                    .replace("Omitted long matching line", "...省略过长匹配行..."),
                        });
                        window.emit("rg-output", &result_records).unwrap();
                        result_records.clear();

                        pip_keyword_records[i].hits += 1;
                        pip_progress_update(window.clone(), &pip_keyword_records, i);
                    } else {
                        // 不是最后一个关键字，则将Filelist结果传递给下一个关键字
                        //let (_, content) = split_path_content(line.as_str());
                        next_file_list.push(FileRecord {
                            file_path: file2.file_path.clone(),
                            content: file2.content.clone()
                                + "————————————————"
                                + &keywords[i]
                                + "["
                                + &total_lines.to_string()
                                + "]————————————————\n"
                                + &lines_str
                                    .replace("omitted end of long line", "行尾过长略...")
                                    .replace("Omitted long matching line", "...省略过长匹配行...")
                                + "\n",
                        });
                        // 关键字命中数更新
                        pip_keyword_records[i].hits += 1;
                        // 更新进度到前端
                        pip_progress_update(window.clone(), &pip_keyword_records, i);
                    }
                    //}
                    //Err(err) => eprintln!("Error reading line: {}", err),
                    //}
                }
            }
            // 等待每次循环中的rga进程结束
            let status = rg_process.wait().expect("Failed to execute rga process");
            if !status.success() {
                if let Some(stderr) = rg_process.stderr.take() {
                        let mut reader = io::BufReader::new(stderr);
                        let mut lines = String::new();
                        let _ = reader.read_to_string(&mut lines).unwrap();
                        if !lines.trim().is_empty() {
                            eprintln!("Pip_srearch() running rga process Error : {}", &lines);
                        }
                }

                //eprintln!("pip_srearch() Error running rga process: {}", status);
                continue;
            }
        }

        // 更新文件列表为当前匹配的文件列表
        file_list = next_file_list;

        if !file_list.is_empty() {
            // 更新进度信息
            pip_progress_update(window.clone(), &pip_keyword_records, i);
        
        } else {
            pip_progress_update(window.clone(), &pip_keyword_records, i);
            break;
        }
        // 输出最终过滤结果
    }
    // rg_process
    //     .wait()
    //     .expect("Failed to pipe output from rga process")
}

// return (created_at, modified_at)
fn get_filetime(file_path: &str) -> (String, String) {
    //println!("get_filetime: file_path:{}", file_path);
    let metadata = fs::metadata(file_path).expect("Failed to get metadata");
    //let mut created_at = String::new();
    //let mut modified_at = String::new();
    let created_at=if let Some(created) = FileTime::from_creation_time(&metadata) {
         format_filetime(&created)
    } else {
         "".to_string()
    };
    let modified_at = format_filetime(&FileTime::from_last_modification_time(&metadata));

    (created_at, modified_at)
}
fn format_filetime(filetime: &FileTime) -> String {
    #[cfg(windows)]
    let seconds = (filetime.seconds() - 11644473600) as i64;
    #[cfg(not(windows))]
    let seconds = filetime.unix_seconds();

    let nanos = filetime.nanoseconds();
    let datetime = Utc.timestamp_opt(seconds, nanos).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

//从stdout中分割出路径和匹配命中内容
fn split_path_content(line: &str) -> (String, String) {
    let pathes: Vec<&str> = line.split(':').collect();
    //let mut path = String::new();
    let mut content = String::new();
    if pathes.len() == 1 {
        let path = pathes[0].trim().to_string();
         (path, "".to_string())
    } else if pathes.len() >= 2 {
        //windows下，路径中盘符后一定有一个冒号，所以第二个冒号后才是content
        if OS == "windows" {
            let path = pathes[0].to_string() + ":" + pathes[1];
            for i in 2..pathes.len() {
                content += pathes[i];
                if i < pathes.len() - 1 {
                    content += ":"; //补充上之前的冒号
                }
            }
             (path.trim().to_string(), content.trim().to_string())
        } else {
            //linux下，第一个冒号后是content
            let path = pathes[0].trim().to_string();
            for i in 1..pathes.len() {
                content += pathes[i];
                if i < pathes.len() - 1 {
                    content += ":"; //补充上之前的冒号
                }
            }
            (path, content.trim().to_string())
        }
    } else {
         (String::new(), String::new())
    }
    
    //(path.trim().to_string(), content.trim().to_string())
}


fn emit_completed_signal(window: tauri::Window, status: ExitStatus) {
    if let Some(code) = status.code() {
    //match status.code().unwrap() {
    match code {
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
    } else {
        emit_signal(window, "completed", "当前搜索已强制终止!");
    }
}
fn emit_signal(window: tauri::Window, signal: &str, message: &str) {
    window
        .emit(signal, Some(message.to_string()))
        .expect(format!("Failed to send {} message", signal).as_str());
}

#[derive(Debug, Deserialize, Serialize)]
struct GiteeRelease {
    tag_name: String,
    body: String,
}
#[tauri::command]
async fn check_update() -> Result<GiteeRelease, String> {
    use reqwest;
    let url = "https://gitee.com/api/v5/repos/vvvvvx/fast-full-text-search/releases/latest?access_token=c7ebacadf8aa266ec1cd71b271d3f4c3";
    //let url = "https://api.github.com/repos/vvvvvx/fast-full-text-search/releases/latest";
    match reqwest::Client::new().get(url)
        //.header("User-Agent", "fast-full-text-search")
        //.header("Accept", "application/vnd.github.v3+json")
        .send()
        .await {
        Ok(response) => {
            //let text= response.text().await.unwrap();
            //println!("check_update Response: {:?}", response);
            if let Ok(release) = response.json::<GiteeRelease>().await {
                println!("Latest release: {:?}", release);
                Ok(release)
            } else {
                Err("Failed to parse release info from Gitee".to_string())
            }
        }
        Err(e) => {
            eprintln!("Error getting latest release info from Gitee: {}", e);
            Err("Failed to get latest release info from Gitee".to_string())
        }
    }
}
#[tauri::command]
fn open_file(file_path: &str) {
    println!("file_path:{}", file_path);
    if let Err(e) = that(file_path) {
        println!("Error opening file: {}", e);
    }
}

#[tauri::command]
fn get_home_dir() -> String {
    let home_dir = dirs::home_dir().unwrap();

    #[cfg(not(windows))]
    {
        home_dir.to_str().unwrap().to_string()+"  |  "
    }

    #[cfg(windows)]
    {//如果是Windows则获取所有盘符，排除C盘+用户目录
        use sysinfo::{Disks, System};

        let mut drives = String::new();
        let disks = Disks::new_with_refreshed_list(); // 获取所有磁盘

        for disk in disks.list() {
            //排除C盘
            if !disk.mount_point().to_string_lossy().starts_with(r"C:\") {
                drives += &(disk.mount_point().to_string_lossy().to_string()+"  |  ");
            }
        }
        home_dir.to_str().unwrap().to_string()+"  |  "+&drives
    }
}

#[tauri::command]
fn kill_rga_process(window: tauri::Window) {
    //println!("rga process cleanning");
    #[cfg(windows)]
    let mut child = Command::new("taskkill")
        .arg("/IM")
        .arg("rg*")
        .arg("/F")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .expect("Failed to execute taskkill process");
    #[cfg(not(windows))]
    let mut child = Command::new("pkill")
        .arg("-9")
        .arg("-f")
        .arg("^rg")
        .spawn()
        .expect("Failed to execute pkill process");


    let status   = child.wait().expect("Failed to execute taskkill process");
    if !status.success() {
        if let Some(stderr) = child.stderr.take() {
                let mut reader = io::BufReader::new(stderr);
                let mut lines = String::new();
                let _ = reader.read_to_string(&mut lines).unwrap();
                if !lines.trim().is_empty() {
                    eprintln!("Error killing rga process: {}", &lines);
                }
        }
        eprintln!("Error killing rga process: {}", status);
    }
    window.emit("rg-process-killed", "当前搜索已终止！").unwrap();
    println!("Searching process cleanned up");
}

fn kill_rga_process_fn() {
    println!("Searching process cleanning up");
    #[cfg(windows)]
    let mut child = Command::new("taskkill")
        .arg("/IM")
        .arg("rg*")
        .arg("/F")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .spawn()
        .expect("Failed to execute taskkill process");
    #[cfg(not(windows))]
    let mut child = Command::new("pkill")
        .arg("-9")
        .arg("-f")
        .arg("^rg")
        .spawn()
        .expect("Failed to execute pkill process");


    let status   = child.wait().expect("Failed to execute taskkill process");
    if !status.success() {
        if let Some(stderr) = child.stderr.take() {
                let mut reader = io::BufReader::new(stderr);
                let mut lines = String::new();
                let _ = reader.read_to_string(&mut lines).unwrap();
                if !lines.trim().is_empty() {
                    eprintln!("Error killing rga process: {}", &lines);
                }
        }
        eprintln!("Error killing rga process: {}", status);
    }else{
        println!("Searching process cleanned up");
    }
}

 use tauri::Manager ;

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

    let app=tauri::Builder::default()
        //窗口居中显示
        .setup(|app| {
            let window= app.get_window("main").unwrap();
            
            window.center().unwrap();
            #[cfg(windows)]
            sleep(Duration::from_millis(300));
            window.show().unwrap();
            //注册应用退出事件
            //let app_handle = app.handle().clone();
            app.listen_global("tauri://exit", move |_| {
                kill_rga_process_fn();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            run_rg_command,
            open_file,
            open_folder_dialog,
            goto_folder,
            get_home_dir,
            check_update,
            kill_rga_process, 
            //stop_rg_command
        ])
        //.run(tauri::generate_context!())
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

        app.run(|_app_handle, event, | { match event {
            tauri::RunEvent::ExitRequested { .. }=> {
                kill_rga_process_fn();
            }
            _ => {}
            
        }
});

}
