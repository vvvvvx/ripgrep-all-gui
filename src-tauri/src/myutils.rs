use tauri::utils::pattern;

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
        return ptrns[0].to_string();
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
