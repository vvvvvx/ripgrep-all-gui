pub fn generate_filetypes(filetypes: &str) -> String {
    if filetypes.is_empty() {
        return String::new();
    }
    let mut types = String::new();
    for filetype in filetypes.split_whitespace() {
        types = types + " -g '*." + filetype + "'";
    }
    types
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
            + "'";
    }
    // -e '数据治理.*财务|财务.*数据治理'
}
