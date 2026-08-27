export default {
    "welcome": "欢迎使用 Ripgrep All GUI",
    "search_label": "搜索",
    "search_location_label": "搜索位置",
    "search_location_title": `搜索根目录
    
全盘搜索时间也不会太久，
但缩小搜索范围，会大大缩短搜索时间。
    
可用“\\|”分隔多个目录或盘符，例如：
或 C \\| D \\| E \n或 C: \\| D: \\| E: 
或 C:\\ \\| D:\\ \\| E:\\
或 D:\\文件 \\| E:\\日记 \\| F:
    
鼠标右击清空，再右击恢复初始，点击“...”选择目录。`,

    "search_location_placeholder": "选择或输入搜索位置",
    "search_location_button_title": "点击选择搜索根路径",
    "search_filetype_label": "文件类别",
    "search_filetype_placeholder": "默认空，搜常用类别。例：*.docx  *.pdf 。指定类别，速度倍增。",
    "search_filetype_fullsearch_label": "执行[全面搜索]，忽略指定类别。最全面也最慢！",

    "search_filetype_title": `【文件类别】，即在哪些类别的文件中搜索内容，通常由扩展名决定。
1. 输入指定类别时，则只搜该类别的文件，速度最快；
2. 留空时，搜常用类别：Office、PDF、Markdown、Txt、网页、eml、LaTeX等，速度较慢；
3. 勾选【全面搜索】时，搜所有类别文件(不含二进制和隐藏文件)，速度最慢。

不建议留空，明确文件类别，可成倍提高搜索速度。

可空格分隔多个类别。
通配符*表任意字符。例：
*.docx *.pdf  表示只搜索扩展名为 docx 和 pdf 的两类文件。
*研究报告*  表示只搜索文件名包含“研究报告”的任何类型文件。
*研究报告*.pdf   表示只搜索文件名包含“研究报告”的 pdf 文件。

叹号 !，表示排除，例如：
!*.txt 表示搜索会排除扩展名为 txt 的文件。
!*研究报告* 表示搜索会排除文件名包含“研究报告”的任何类型文件。
!*研究报告*.pdf 表示搜索会排除文件名包含“研究报告”的 pdf 文件。`,

    "search_filetype_droplist_title": `✔ Include 包含 此特征
✘ Exclude 排除 此特征
    
点击复选框/Ctrl+Space切换
    
上下方向键选中，鼠标点击/Enter确认`,

    "clear_filetype_button_title": "清空文件类别",
    "regex_title": "勾选此项，启用正则表达式模式。\n勾选后，右侧框输入内容将被视为正则表达式",
    "search_pattern_placeholder": "如：市场调研 销售数据 人工智能。多关键字以空格分隔",
    "search_pattern_title": `全文搜索关键字
    
1. 支持正则表达式（需开启Regex模式）。
2. 普通模式：即单关键字搜索，最常用！
3. 管道模式：空格分隔多关键字，将漏斗式逐关键字过滤，较耗时。

注意：
管道模式下，低频关键字靠前放有利于缩短搜索时间
管道模式下，每关键字最多显示三次命中结果，仅显示最后关键字的命中次数。
    
正则说明：
1. 全文搜索时，支持高级正则模式先行和后行断言，如：(?=.*K1)(?=.*K2)(?=.*K3) ; 表示字符串同时含K1、K2、K3
2. 仅搜文件名 时，默认为普通正则模式，不必勾选Rege`,

    "search_button_title": "为缩短搜索时间，程序会多线程并发搜索。\n因此，CPU占用率很高是正常现象！",
    "checkbox_filename_title": "只搜索文件名，不搜索内容",
    "checkbox_filename_label": "搜文件名",
    "checkbox_hidden_title": "默认不搜索隐藏文件，勾选此项，将搜索隐藏文件",
    "checkbox_hidden_label": "隐藏文件",
    "checkbox_binary_title": "把二进制文件作为文本搜索\n\n速度较慢，输出内容可能包含非法字符。",
    "checkbox_binary_label": "二进制",
    "checkbox_rawcode_title": "把文件作为源代码搜索，而不是解析后的文本。\n\n如：HTML,如果不作为源代码搜索，则无法搜索到HTML标签，只能搜索到解析后的正文。",
    "checkbox_rawcode_label": "源码模式",
    "checkbox_multiline_title": "高级正则模式下，先行或后行断言时，进行跨行多关键字匹配。\n\n注意：速度将变慢！",
    "checkbox_multiline_label": "跨行匹配",
    "checkbox_fullsearch_title": "搜索任何格式的文件，包括zip等压缩文件，但不包括隐藏文件或二进制文件。\n\n不勾选此项，只搜索常用文件类型。\n\n勾选此项，速度最慢。\n\n勾选此项，将忽略用户指定的文件类别，执行最全面的搜索。",
    "checkbox_fullsearch_label": "全面搜索",
    "checkbox_display_createdate_title": "显示搜索结果文件创建时间",
    "checkbox_display_createdate_label": "显示创建时间",
    "checkbox_display_modifiedate_title": "显示搜索结果文件修改时间",
    "checkbox_display_modifiedate_label": "显示修改时间",

    "input_maxcount_title": "单个文件中出现的关键字次数达到[最大匹配次数]后，程序将不再搜索此文件，以提高效率。\n\n0 表示无限制。\n过大可能会导致搜索时间过长",
    "input_maxcount_label": "匹配次数",
    "input_depth_title": "目录遍历深度\n\n1-表示当前文件夹\n2-表示当前文件夹及子文件夹\n3-表示当前文件夹及子文件夹及子文件夹...\n\n深度越大，耗时越",
    "input_depth_label": "遍历深度",
    "input_maxcolumn_title": "匹配结果行预览最大字符数，超过此长度将被截断，并省略显示长",
    "input_maxcolumn_label": "预览长度",


    "stop_button_title": "强制终止当前搜索",
    "stop_button_label": "终止",
    "status_searching_label": "执行状态：",
    "status_searching_output_label": "搜索结果：",
    "status_pipe_mode_label": "管道模式中(较耗时)...",
    "status_filename_mode_label": "文件名搜索中...",
    "status_fulltext_search_mode_label": "全文搜索中...",
    "status_search_done_label": "->完成",
    "status_results_records": "条",

    "table_col_dir_label": "目录",
    "table_col_hits_label": "命中-",
    "table_col_hits_title": "点击按命中次数排序",
    "table_col_createdat_label": "创建时间-",
    "table_col_createdat_title": "点击按创建时间排序",
    "table_col_modifiedat_label": "修改时间-",
    "table_col_modifiedat_title": "点击按修改时间排序",
    "table_col_file_label": "文件-",
    "table_col_file_title": "点击按文件名排序",
    "table_item_dir_title": "点击打开文件所在目录",
    "table_item_hits_title": "关键字在该文件中的出现次数/命中次数\n\n受限于【最大匹配次数】设置",
    "table_item_createdat_title": "文件创建时间\n\n点击标题栏可按创建时间排序",
    "table_item_modifiedat_title": "文件修改时间\n\n点击标题栏可按修改时间排序",

    "developed_by_title": "点我访问软件主页，可提Bug或建议。",
    "version_title": "点击我查看版本更新信息",

    "msg_stop_confirm": "确定要终止当前搜索吗？",
    "msg_no_task": "当前无搜索任务进行中",
    "msg_keyword_too_short": "搜索关键字过短！\n\n英文字符每关键字至少4个字符，或用Regex模式。汉字长短不限。",
    "msg_keyword_cant_blank": "搜索关键字不能为空",
    "msg_search_location_cant_blank": "搜索位置不能为空",
    "msg_keyword_and_filetype_all_blank": "文件名搜索时，搜索模式和文件名特征不能同时为空",
    "msg_maxcount_invalid": "最大匹配次数必须为非负整数",
    "msg_depth_invalid": "遍历深度必须为非负整数",
    "msg_maxcolumn_invalid": "预览长度必须为非负整数",

    "msg_text_full_search_very_slow": "全面搜索，极耗时！",
    "msg_text_filetype_empty": "[文件类别]未指定",
    "msg_text_all_type_slow": "搜索【所有类别】的文件，速度最慢，请耐心等待...",
    "msg_text_common_type_slow": "搜索【常用类别】的文件，速度可能较慢，请耐心等待...",
    "msg_text_input_filetype_to_speedup": "指定[文件类别]，速度倍增！",

    "msg_text_search_done": "搜索完成!",
    "msg_text_search_not_finished": "当前搜索未结束!",
    "msg_text_be_patient": "请耐心等待...",
    "msg_text_click_cancel": "请点击取消",
    "msg_text_if_force_stop": "是否强行终止？",

    "msg_text_new_version_found": "发现新版本！",
    "msg_text_current_version": "当前版本：",
    "msg_text_latest_version": "最新版本：",
    "msg_text_please_go": "请前往",
    "msg_text_download": "下载！",

    "button_text_cancel": "取消",
    "button_text_stop": "终止",
    "button_text_ok": "确定",

    "bg_msg_regex_error": "执行结束：正则表达式解析错误!",
    "bg_msg_search_done": "搜索完成!",
    "bg_msg_rga_not_found": "搜索命令rga不存在!",
    "bg_msg_search_failed": "搜索失败!",
    "bg_msg_task_terminated": "当前搜索已强制终止!",
    "bg_msg_overdate": "软件已过期！\n请根据窗口右下方联系方式获取最新版!",
}