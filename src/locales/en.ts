export default {
    "welcome": "Welcome to Ripgrep All GUI",
    "search_label": "Search",
    "search_location_label": "Location",
    "search_location_title": `Search Root Directories
    
Searching the entire disk usually does not take too long,
but narrowing the search scope can greatly reduce search time.
    
Use "\\|" to separate multiple directories or drives, for example:
or  C \\| D \\| E
or  C: \\| D: \\| E:
or  C:\\ \\| D:\\ \\| E:\\
or  D:\\Documents \\| E:\\Diary \\| F:
    
Right-click to clear, right-click again to restore the default value,
or click "..." to select directories.`,

    "search_location_placeholder": "Select or enter search location",
    "search_location_button_title": "Click to select the search root path",
    "search_filetype_label": "File Type",
    "search_filetype_placeholder": "Default: common types (*.docx, *.pdf). Specify types to speed up.",
    "search_filetype_fullsearch_label": "[Full Search] Active: Ignores types. Most thorough, slowest.",

    "search_filetype_title": `【File Type】specifies which types of files to search for content, usually determined by the file extension.

1. When specific types are entered, only those file types are searched, providing the fastest search speed;
2. When left empty, common types are searched: Office, PDF, Markdown, Txt, Web pages, eml, LaTeX, etc. This is slower;
3. When 【Full Search】 is enabled, all file types are searched (excluding binary and hidden files). This is the slowest option.

It is not recommended to leave this empty. Specifying file types explicitly can greatly improve search speed.

Multiple types can be separated by spaces.
The wildcard * represents any characters. Examples:
*.docx *.pdf  means only files with the docx and pdf extensions are searched.
*Research Report*  means any type of file whose filename contains "Research Report" is searched.
*Research Report*.pdf   means only PDF files whose filename contains "Research Report" are searched.

An exclamation mark ! means exclude. For example:
!*.txt means TXT files are excluded from the search.
!*Research Report* means any file whose filename contains "Research Report" is excluded.
!*Research Report*.pdf means PDF files whose filename contains "Research Report" are excluded.`,

    "search_filetype_droplist_title": `✔ Include: Include this feature
✘ Exclude: Exclude this feature
    
Click the checkbox / Ctrl+Space to toggle
    
Use the Up/Down arrow keys to select, then click the mouse / press Enter to confirm`,

    "clear_filetype_button_title": "Clear file types",
    "regex_title": "Check this option to enable regular expression mode.\nWhen enabled, the input on the right will be treated as a regular expression.",
    "search_pattern_placeholder": "Space-separated keywords. e.g., Marketing Sales AI",
    "search_pattern_title": `Full-text search keywords
    
1. Supports regular expressions (Regex mode must be enabled).
2. Normal mode: searches for a single keyword; this is the most commonly used mode.
3. Pipeline mode: separate multiple keywords with spaces; keywords are filtered sequentially like a funnel. This is more time-consuming.

Notes:
In Pipeline mode, placing low-frequency keywords first can help reduce search time.
In Pipeline mode, each keyword displays at most three matching results; only the match count of the final keyword is displayed.
    
Regular expression notes:
1. During full-text searches, advanced regular expression lookahead and lookbehind assertions are supported. For example: (?=.*K1)(?=.*K2)(?=.*K3) means the string must contain K1, K2, and K3 simultaneously.
2. When searching filenames only, standard regular expression mode is used by default, so there is no need to enable Regex.`,

    "search_button_title": "To reduce search time, the program performs multi-threaded concurrent searches.\nTherefore, high CPU usage is normal!",
    "checkbox_filename_title": "Search filenames only; do not search file contents",
    "checkbox_filename_label": "Filename",
    "checkbox_hidden_title": "Hidden files are not searched by default. Check this option to include hidden files.",
    "checkbox_hidden_label": "Hidden Files",
    "checkbox_binary_title": "Search binary files as text\n\nThis is slower, and the output may contain invalid characters.",
    "checkbox_binary_label": "Binary",
    "checkbox_rawcode_title": "Search files as source code rather than parsed text.\n\nFor example, when searching HTML, enabling this option allows HTML tags to be found; otherwise, only the parsed text content can be searched.",
    "checkbox_rawcode_label": "Code Mode",
    "checkbox_multiline_title": "In advanced regular expression mode, perform cross-line multi-keyword matching when using lookahead or lookbehind assertions.\n\nNote: This will slow down the search!",
    "checkbox_multiline_label": "Cross Line",
    "checkbox_fullsearch_title": "Search files of any format, including compressed files such as ZIP, but excluding hidden and binary files.\n\nWhen unchecked, only common file types are searched.\n\nWhen checked, this is the slowest option.\n\nWhen checked, user-specified file types are ignored and the most comprehensive search is performed.",
    "checkbox_fullsearch_label": "Full Search",
    "checkbox_display_createdate_title": "Display the creation time of search result files",
    "checkbox_display_createdate_label": "Create Time",
    "checkbox_display_modifiedate_title": "Display the modification time of search result files",
    "checkbox_display_modifiedate_label": "Modify Time",

    "input_maxcount_title": "Stops searching a file once [Max Hits] is reached to boost efficiency.\n\n0 means unlimited.\nToo large may cause long search times.",
    "input_maxcount_label": "Max Hits",
    "input_depth_title": "Directory traversal depth\n\n1 - current folder only\n2 - current folder and its subfolders\n3 - current folder, subfolders, and sub-subfolders...\n\nThe greater the depth, the longer the search takes.",
    "input_depth_label": "Depth",
    "input_maxcolumn_title": "Maximum character length for row preview. Over-length text will be omitted.",
    "input_maxcolumn_label": "Length",

    "stop_button_title": "Force terminate the current search",
    "stop_button_label": "Stop",
    "status_searching_label": "Status : ",
    "status_searching_output_label": "Results : ",
    "status_pipe_mode_label": "Pipe mode (slower)...",
    "status_filename_mode_label": "Filename searching...",
    "status_fulltext_search_mode_label": "Full-text searching...",
    "status_search_done_label": "->Done",
    "status_results_records": "records",

    "table_col_dir_label": "Dir",
    "table_col_hits_label": "Hits-",
    "table_col_hits_title": "Click to sort by number of matches",
    "table_col_createdat_label": "CreatedAt-",
    "table_col_createdat_title": "Click to sort by creation time",
    "table_col_modifiedat_label": "ModifiedAt-",
    "table_col_modifiedat_title": "Click to sort by modification time",
    "table_col_file_label": "File-",
    "table_col_file_title": "Click to sort by filename",
    "table_item_dir_title": "Click to open the directory containing the file",
    "table_item_hits_title": "Keyword hit count in this file.Limited by [Max hits] setting",
    "table_item_createdat_title": "File creation time\n\nclick title to sort by creation time",
    "table_item_modifiedat_title": "File modification time\n\nclick title to sort by modification time",

    "developed_by_title": "Click to visit the software homepage to report bugs or submit suggestions.",
    "version_title": "Click me to view version update information",

    "msg_stop_confirm": "Sure to stop the current search?",
    "msg_no_task": "No search task in progress",
    "msg_keyword_too_short": "Min 4 chars for English (or use Regex). No limit for Chinese.",
    "msg_keyword_cant_blank": "Search keyword cannot be empty",
    "msg_search_location_cant_blank": "Search location cannot be empty",
    "msg_keyword_and_filetype_all_blank": "For filename search, Keywords or FileType cannot be both empty.",
    "msg_maxcount_invalid": "Maximum matches must be a non-negative integer",
    "msg_depth_invalid": "Traversal depth must be a non-negative integer",
    "msg_maxcolumn_invalid": "Preview length must be a non-negative integer",

    "msg_text_full_search_very_slow": "Full search is extremely slow!",
    "msg_text_filetype_empty": "[File Type] not specified",
    "msg_text_all_type_slow": "Searching [All Types] is slowest. Please wait...",
    "msg_text_common_type_slow": "Searching [Common Types] may be slow. Please wait...",
    "msg_text_input_filetype_to_speedup": "Specify [File Type] for faster search!",

    "msg_text_search_done": "Search completed!",
    "msg_text_search_not_finished": "Current search still running!",
    "msg_text_be_patient": "Please be patient...",
    "msg_text_click_cancel": "Please click cancel",
    "msg_text_if_force_stop": "Force stop?",

    "msg_text_new_version_found": "New version found!",
    "msg_text_current_version": "Current version:",
    "msg_text_latest_version": "Latest version:",
    "msg_text_please_go": "Please go to",
    "msg_text_download": "to download!",

    "button_text_cancel": "Cancel",
    "button_text_stop": "Stop",
    "button_text_ok": "OK",

    "bg_msg_regex_error": "Finished: Regex parsing error!",
    "bg_msg_search_done": "Search completed!",
    "bg_msg_rga_not_found": "Command rga not found!",
    "bg_msg_search_failed": "Search failed!",
    "bg_msg_task_terminated": "Search terminated forcefully!",
    "bg_msg_overdate": "Software has expired!\nPlease get the latest version!",
}