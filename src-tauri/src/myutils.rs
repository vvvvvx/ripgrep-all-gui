//use std::path::Path;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::process::Command;

// #[cfg(windows)]
// use winapi::shared::minwindef::{DWORD, UINT};

#[cfg(windows)]
use winapi::um::fileapi::{GetDriveTypeW, GetFileAttributesW};

#[cfg(windows)]
use std::ffi::OsStr;

#[cfg(windows)]
fn is_onedrive_path(path: &str) -> bool {
    let path_wide = OsStr::new(path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
    let drive_type = unsafe { GetDriveTypeW(path_wide.as_ptr()) };
    drive_type == 4 // DRIVE_REMOTE
}

#[cfg(windows)]
fn is_remote_file(path: &str) -> bool {
    let path_wide = OsStr::new(path)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
    let attributes = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
    attributes & 0x1000 != 0 // FILE_ATTRIBUTE_REPARSE_POINT
}
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
        " -F ".to_string() + ptrns[0] + " "
    } else {
        " -U -e ".to_string() + ptrns[0] + ".*" + ptrns[1] + "|" + ptrns[1] + ".*" + ptrns[0] + " "
    }
    // -e '数据治理.*财务|财务.*数据治理'
}

// 手动处理包含空格的文件路径
pub fn split_args(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut escaping = false;

    for ch in input.chars() {
        if escaping {
            if ch == ' ' {
                current.push(ch); // 保留转义空格
            } else {
                current.push('\\'); // 保留转义字符
                current.push(ch);
            }
            escaping = false;
        } else if ch == '\\' {
            if current.ends_with(':') {
                // 如果当前字符是路径分隔符且前一个字符是冒号，保留路径分隔符
                current.push(ch);
            } else {
                escaping = true;
            }
        } else if ch == ' ' {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
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
