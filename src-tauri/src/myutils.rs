//use std::path::Path;
use std::process::Command;
//use tauri::utils::pattern;
pub fn generate_filename_pattern(filename_pattern: &str) -> String {
    if filename_pattern.is_empty() {
        return String::new();
    }
    let mut ptrn = String::new();
    // 处理误输入的中文叹号
    let patternes = filename_pattern.replace("！", "!");
    for pattern in patternes.split_whitespace() {
        ptrn = ptrn + " -g '" + pattern + "' ";
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
        return " -U -e '".to_string()
            + ptrns[0]
            + ".*"
            + ptrns[1]
            + "|"
            + ptrns[1]
            + ".*"
            + ptrns[0]
            + "' ";
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
