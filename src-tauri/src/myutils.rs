//use std::path::Path;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, ExitStatus, Stdio},
};

//use tauri::utils::pattern;
pub fn generate_filename_pattern(filename_pattern: &str) -> String {
    if filename_pattern.is_empty() {
        return String::new();
    }
    let mut ptrn = String::new();
    // 处理误输入的中文叹号
    let patternes = filename_pattern.replace("！", "!");
    for pattern in patternes.split_whitespace() {
        ptrn = ptrn + " -g " + pattern + " ";
    }
    ptrn
}

pub fn generate_patterns(patterns: &str) -> String {
    if patterns.is_empty() {
        return String::new();
    }
    let ptrns: Vec<&str> = patterns.split_whitespace().collect();

    if ptrns.len() == 1 {
        return " -F ".to_string() + ptrns[0] + " ";
    } else {
        return " -U -e ".to_string()
            + ptrns[0]
            + ".*"
            + ptrns[1]
            + "|"
            + ptrns[1]
            + ".*"
            + ptrns[0]
            + " ";
    }
    // -e '数据治理.*财务|财务.*数据治理'
}
pub fn is_scoop_installed() -> bool {
    Command::new("powershell")
        .arg("-Command")
        .arg("scoop")
        .output()
        .is_ok()
}

pub fn install_scoop() -> Result<(), String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Invoke-Expression (New-Object System.Net.WebClient).DownloadString('https://get.scoop.sh')")
        .output()
        .map_err(|e| format!("Failed to install Scoop: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn is_rga_installed() -> bool {
    Command::new("rga").arg("--version").output().is_ok()
}

pub fn install_rga() -> Result<(), String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("scoop install rga")
        .output()
        .map_err(|e| format!("Failed to install rga: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
// 用于三个及以上关键字的搜索采用管道过滤法，即前一个关键字的输出作为后一个关键字的输入
pub fn pip_search(
    window: tauri::Window,
    keywords: Vec<String>,
    mut file_list: Vec<String>,
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
        let mut next_file_list: Vec<String> = Vec::new();
        let keyword = keywords[i].clone();

        key = key.clone() + "->[" + keywords[i].as_str() + "]";
        // 输出当前关键字的进度信息
        window
            .emit("progress", Some(key.clone()))
            .expect("Failed to send completed message");

        for file in file_list {
            let file2 = file.clone();
            //println!("在文件 {} 中搜索: {}", file, keywords[i]);
            // 对于每个文件，使用 rg 进行进一步的关键字过滤
            rg_process = Command::new("rga")
                .args(addtional_args.split_whitespace().collect::<Vec<&str>>())
                .arg(keyword.clone())
                .arg(file)
                .arg("--no-messages")
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute rga process");

            if let Some(stdout) = rg_process.stdout.take() {
                let reader = BufReader::new(stdout);

                // 如果输出结果不为空，则说明该文件包含当前关键字
                if let Some(next) = reader.lines().next() {
                    match next {
                        Ok(line) => {
                            if i == keywords_len - 1 {
                                window
                                    .emit("rg-output", file2 + ": " + line.as_str())
                                    .unwrap();
                            } else {
                                next_file_list.push(file2);
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
        }
        // 输出最终过滤结果
    }
    rg_process
        .wait()
        .expect("Failed to pipe output from rga process")
}
