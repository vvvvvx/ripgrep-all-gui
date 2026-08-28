
import { invoke } from '@tauri-apps/api/core';
import { nextTick, onBeforeUnmount, onMounted, onUnmounted, ref, computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { revealItemInDir } from '@tauri-apps/plugin-opener';
import { el, tr } from 'vuetify/locale';
import { getVersion } from '@tauri-apps/api/app';
import { homeDir } from '@tauri-apps/api/path';
import { useI18n } from 'vue-i18n';
import { defaultFileTypes } from '../utils/filetypes';

function getById(id) {
    return document.getElementById(id);
}
function scrollToTop() {
    getById("outputTable").scrollTop = 0;
}
function resetTableHeader() {
    let sf = getById("sort-by-file");
    let hc = getById("sort-by-hit-count");
    let mt = getById("sort-by-modified");
    let ct = getById("sort-by-created");
    sf.innerText = t('table_col_file_label'); //"文件-";
    hc.innerText = t('table_col_hit_count_label'); // "命中-";
    mt.innerText = t('table_col_modifiedat_label'); // "修改时间-";
    ct.innerText = t('table_col_createdat_label'); // "创建时间-";
}
function isPatternNotOK(pattern) {
    // 非汉字字符过短检测，短于3字符的返回true。汉字字符不做检测。
    //   let re=/^[ ]*[\x00-\xFF]{1,3}([ ]|$)/;
    let re = /^[ ]*[\x00-\xFF]{1,3}[ ]*$/;
    console.log(pattern);
    console.log(re);
    console.log(re.test(pattern));
    return re.test(pattern);

}

function buildRegex(s) {
    // 转义正则特殊字符
    const escaped = s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

    // 构造正则：以 escaped 开头，跟随 0 个或多个空格，以中文或英文分号结尾
    const pattern = `${escaped} *\\|`;
    return new RegExp(pattern);
}
export default {
    methods: {
        openFile(filePath) {
            console.log(filePath);
            invoke('open_file', { filePath: filePath });
        },
        // gotoFolder2(folderPath) {
        //     console.log(folderPath);

        //     invoke('goto_folder', { folderPath: folderPath });
        // },
        async gotoFolder(path) {
            //alert(path);
            await revealItemInDir(path)
        }

    },
    setup() {
        const i18n = useI18n();
        const { t } = useI18n();
        const curLang = computed(() => i18n.locale.value);
        const searchPattern = ref(''); // 搜索模式
        const output = ref([]); // 搜索结果数组
        const searchPath = ref(''); // 搜索路径
        const filenamePattern = ref(''); // 搜索文件类型，空格分割
        const regexMode = ref(false);// 是否启用正则表达式模式
        const dispHitCount = ref(false); // 是否显示命中次数
        const preFile = ref('');// 记录上一个搜到的文件名，用于去重
        const cmdStatus = ref(''); // 命令状态
        const searchFilename = ref(false); // 是否只搜索文件名
        const maxCount = ref(50); // 单个文件最大匹配次数
        const OS = ref(''); // 操作系统
        const searchHidden = ref(false); // 是否搜索隐藏文件
        const maxDepth = ref(100); // 目录遍历深度
        const searchBinary = ref(false); // 是否把二进制文件作为文本搜索
        //const excludeNotCommon=ref(true); // 是否排除不常用且耗时的图片、压缩文件、数据库、视频文件
        const searchAll = ref(false); // 是否搜索所有文件
        const multiLine = ref(false); // 是否启用跨行搜索模式
        ////begin 新加
        const isDone = ref(true); //搜索是否执行结束
        const displayCreatedAt = ref(false); //是否显示创建时间
        const displayModifiedAt = ref(true); //是否显示修改时间
        const maxColumn = ref(200); //匹配结果行的最大显示长度,过长将被省略显示
        const isPipMode = ref(false); //是否启用了pip模式
        const curVersion = ref(''); //当前版本号
        const latestVersion = ref(''); //最新版本号
        const latestVersionDesc = ref(''); //最新版本描述
        const versionText = computed(() => { return ((curVersion.value.toLowerCase() < latestVersion.value.toLowerCase()) && latestVersion.value != '') ? `<a href="https://github.com/vvvvvx/ripgrep-all-gui/releases" target="_blank" style="text-decoration:none;color:green;">New version available!</a>` : `<a href="https://github.com/vvvvvx/ripgrep-all-gui/releases" target="_blank" style="text-decoration:none;color:white;">Version: ${curVersion.value}</a>`; }); //版本号显示文本
        const versionTitle = computed(() => { return ((curVersion.value.toLowerCase() < latestVersion.value.toLowerCase()) && latestVersion.value != '') ? `Current version：${curVersion.value}  Latest：${latestVersion.value} \n\nVersion update：\n${latestVersionDesc.value}` : "Click me to view version information" }); //版本号鼠标悬停提示

        const items = ref(defaultFileTypes);

        const filteredItems = ref([]);
        const selectedIndex = ref(-1);
        const homeDir = ref('');
        let timeoutId = null;
        const rawCodeMode = ref(false);//是否源代码搜索模式

        const changeLang = (lang) => {
            i18n.locale.value = lang
            localStorage.setItem('lang', lang)
        }
        const resetTableHeader = () => {
            let sf = getById("sort-by-file");
            let hc = getById("sort-by-hit-count");
            let mt = getById("sort-by-modified");
            let ct = getById("sort-by-created");
            sf.innerText = t('table_col_file_label');
            hc.innerText = t('table_col_hits_label');
            mt.innerText = t('table_col_modifiedat_label');
            ct.innerText = t('table_col_createdat_label');
        };
        const filterItems = () => {
            filteredItems.value = items.value.filter(item => item.content.toLowerCase().includes(filenamePattern.value.toLowerCase()) ||
                item.type.toLowerCase().includes(filenamePattern.value.toLowerCase()));
            selectedIndex.value = -1;
        };
        const handleSearchKeydown = (event) => {
            if (event.key === 'Enter') {
                event.preventDefault();
                runCommand();
            }
        };
        const handleKeydown = (event) => {
            if (event.key === 'ArrowDown') {
                event.preventDefault();
                selectedIndex.value = Math.min(selectedIndex.value + 1, filteredItems.value.length - 1);
            } else if (event.key === 'ArrowUp') {
                event.preventDefault();
                selectedIndex.value = Math.max(selectedIndex.value - 1, 0);
            } else if (event.key === 'Enter') {
                console.log("Keydown Enter pressed");
                event.preventDefault();
                if (selectedIndex.value >= 0) {
                    selectItem(filteredItems.value[selectedIndex.value]);
                    filteredItems.value = [];
                }
                if (filenamePattern.value.trim().length > 0) {
                    console.log("Keydown Enter:" + filenamePattern.value);
                    runCommand();
                }
            } else if (event.ctrlKey && event.key === ' ') {
                if (selectedIndex.value >= 0) {
                    togglePush(filteredItems.value[selectedIndex.value]);
                }
            }
        };
        const clearFilePattern = () => {
            filenamePattern.value = '';
            filteredItems.value = [];
            getById('inputFilePattern').focus();
        };
        const clearPath = () => {
            if (searchPath.value.trim().length > 0) {
                searchPath.value = '';
            } else {
                searchPath.value = homeDir.value;
            }

            getById('inputPath').focus();
        };
        const processExclude = (input) => {
            return input.replace(/\S+/g, '!$&');
        };
        const selectItem = (item) => {
            if (item.include) {
                filenamePattern.value = item.content;
            } else {
                filenamePattern.value = processExclude(item.content);
            }
            filteredItems.value = [];
        };
        const togglePush = (item) => {
            item.include = !item.include;
        };
        const handleDropdownClick = (event) => {
            if (!event.target.closest('.dropdown-list')) {
                filteredItems.value = [];
            }
        };
        ////end 新加

        const showCustomAlert = () => {
            getById("alertBox").style.display = "block";
            getById("closeAlertBtn").focus();

            clearTimeout(timeoutId);
            timeoutId = setTimeout(() => {
                closeCustomAlert();
            }, 10000);
            console.log(getById("alertBox").style.display);
        };

        const closeCustomAlert = () => {
            getById("alertBox").style.display = "none";
            console.log(getById("alertBox").style.display);
            console.log("执行隐藏对话框");
        };

        const showRuningAlert = () => {
            getById("alertRuningBox").style.display = "block";
            getById("closeRuningAlertBtn").focus();

            clearTimeout(timeoutId);
            timeoutId = setTimeout(() => {
                closeRuningAlert();
            }, 10000);
        };
        const closeRuningAlert = () => {
            getById("alertRuningBox").style.display = "none";
        };
        const closeNewVersionAlert = () => {
            getById("alertNewVersionBox").style.display = "none";
        };



        const sortOutputByHitCount = () => {
            let hc = getById("sort-by-hit-count");
            //let sf = getById("sort-by-file");

            //let text=curLang.value==="zh"?"命中":"Hits";
            let text = t('table_col_hits_label').slice(0, -1); //去掉最后的符号


            if (hc.innerText === text + "-") {
                //↑↓
                //descending
                resetTableHeader();
                hc.innerText = text + "↓";
                output.value.sort((a, b) => b.hit_count - a.hit_count);
            } else if (hc.innerText === text + "↓") {
                //ascending   
                hc.innerText = text + "↑";
                output.value.sort((a, b) => a.hit_count - b.hit_count);
            } else {
                //descending
                hc.innerText = text + "↓";
                output.value.sort((a, b) => b.hit_count - a.hit_count);
            }
            //sf.innerText = "文件-";
            scrollToTop();
        };
        const sortOutputByFile = () => {
            let sf = getById("sort-by-file");
            //let hc = getById("sort-by-hit-count");

            //let text=curLang.value==="zh"?"文件":"File"; 
            let text = t('table_col_file_label').slice(0, -1); //去掉最后的符号

            if (sf.innerText === text + "-") {
                //ascending
                resetTableHeader();
                sf.innerText = text + "↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            } else if (sf.innerText === text + "↑") {
                //descending
                sf.innerText = text + "↓";
                output.value.sort((a, b) => b.file.localeCompare(a.file));
            } else {
                //ascending
                sf.innerText = text + "↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            }

            //hc.innerText = "命中-";
            scrollToTop();
        };
        const sortOutputByCreated = () => {
            let sc = getById("sort-by-created");
            let text = t('table_col_createdat_label').slice(0, -1); //去掉最后的符号
            if (sc.innerText === text + "-") {
                //ascending
                resetTableHeader();
                sc.innerText = text + "↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            } else if (sc.innerText === text + "↑") {
                //descending
                sc.innerText = text + "↓";
                output.value.sort((a, b) => b.created_at.localeCompare(a.created_at));
            } else {
                //ascending
                sc.innerText = text + "↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            }
            scrollToTop();
        };
        const sortOutputByModified = () => {
            let sm = getById("sort-by-modified");
            let text = t('table_col_modifiedat_label').slice(0, -1); //去掉最后的符号

            if (sm.innerText === text + "-") {
                //ascending
                resetTableHeader();
                sm.innerText = text + "↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            } else if (sm.innerText === text + "↑") {
                //descending
                sm.innerText = text + "↓";
                output.value.sort((a, b) => b.modified_at.localeCompare(a.modified_at));
            } else {
                //ascending
                sm.innerText = text + "↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            }

            scrollToTop();
        };
        const toggleSearchAll = () => {
            if (!searchAll.value) {
                searchAll.value = true;
                filenamePattern.value = "";
                getById("inputFilePattern").placeholder = t('search_filetype_fullsearch_label'); //"执行[全面搜索]，忽略指定类别。最全面也最慢！";
            } else {
                searchAll.value = false;
                getById("inputFilePattern").placeholder = t('search_filetype_placeholder');//"默认空，搜常用类别。例：*.zip  *.pdf 。指定类别，速度倍增。";
            }
        };
        const forceKillSearch = async () => {
            try {
                await invoke('kill_rga_process');
                closeRuningAlert();
            } catch (e) {
                console.error(e);
            }
        }
        const forceStop = async () => {
            if (!isDone.value) {
                let confirmResult = await confirm(t('msg_stop_confirm')); //confirm("确定要终止当前搜索？");
                if (confirmResult) {
                    forceKillSearch();
                }
            } else {
                //alert("当前无搜索任务进行。");
                alert(t('msg_no_task')); //"当前无搜索任务进行。"
            }
        }
        const runCommand = async () => {

            if (!isDone.value) {
                //alert("上一搜索未结束...\n请耐心等待。\n\n如欲强制终止，请关闭本程序后重新打开。");
                console.log("runCommand:isDone:", isDone.value);
                showRuningAlert();


                return;
            }
            if (isPatternNotOK(searchPattern.value) && !regexMode.value && !searchFilename.value) {
                //alert("搜索关键字过短！\n\n英文字符每关键字至少4个字符，或用Regex模式。汉字长短不限。");
                alert(t('msg_keyword_too_short')); //"搜索关键字过短！\n\n英文字符每关键字至少4个字符，或用Regex模式。汉字长短不限。"
                return;
            }
            if (searchPattern.value === '' && !searchFilename.value) {    //if search pattern is empty, show error message and return
                // alert('搜索关键字不能为空');
                alert(t('msg_keyword_cant_blank')); //"搜索关键字不能为空"
                return;
            }

            if (searchPath.value === '') {    //if search path is empty, show error message and return  
                //alert('搜索路径不能为空');
                alert(t('msg_search_location_cant_blank')); //"搜索位置不能为空"
                return;
            }
            //处理路径中的空格，进行转义。
            // searchPath.value = searchPath.value.replace(/ /g, '\\ ');

            if (searchFilename.value && filenamePattern.value.trim() === '' && searchPattern.value.trim() === '') {
                //alert('搜索文件名时，搜索模式和文件名特征不能同时为空');
                alert(t('msg_keyword_and_filetype_all_blank')); //"搜索文件名时，搜索模式和文件名特征不能同时为空"
                return;
            }
            if (isNaN(maxCount.value) || maxCount.value < 0) {
                //alert('最大匹配次数必须为0，或正整数');
                alert(t('msg_maxcount_invalid')); //"最大匹配次数必须为0，或正整数"
                return;
            }
            if (isNaN(maxDepth.value) || maxDepth.value < 0) {
                //alert('目录遍历深度必须正整数');
                alert(t('msg_depth_invalid')); //"目录遍历深度必须正整数"
                return;
            }
            if (isNaN(maxColumn.value) || maxColumn.value < 0) {
                //alert('匹配行最大显示长度必须为正整数');
                alert(t('msg_maxcolumn_invalid')); //"匹配行最大显示长度必须为正整数"
                return;
            }

            //if(filenamePattern.value.trim() === '') {
            if (searchAll.value || (filenamePattern.value.trim() === '' && !searchFilename.value)) {
                //alert('【文件类别】未指定，将搜索所有类别的文件，速度较慢！\n\n如需终止搜索，请关闭本程序。\n\n请耐心等待！');
                showCustomAlert();
            }
            isPipMode.value = false;
            isDone.value = false;

            let keywordsArr = searchPattern.value.trim().split(' ');//split search pattern into two keywords
            let ptrn = keywordsArr.filter(item => item.trim() !== '');//remove empty string

            output.value = [];//clear output before running new command
            // reset 表头排序
            resetTableHeader();
            preFile.value = "";//reset preFile before running new command
            if (ptrn.length > 1 && !regexMode.value) {
                cmdStatus.value = t('status_pipe_mode_label'); //"管道模式中(较耗时)...";
                //isPipMode.value=true;
            } else if (searchFilename.value) {
                cmdStatus.value = t('status_filename_mode_label'); //"文件名搜索中...";
            } else {
                cmdStatus.value = t('status_fulltext_search_mode_label'); //"全文搜索中...";
            }
            //强制更新页面 显示搜索进度
            await nextTick();
            await new Promise(resolve => setTimeout(resolve, 50));
            try {
                await invoke('run_rg_command', {
                    searchPattern: searchPattern.value,
                    searchPath: searchPath.value,
                    filenamePattern: filenamePattern.value,
                    regexMode: regexMode.value,
                    dispHitCount: dispHitCount.value,
                    searchFilename: searchFilename.value,
                    maxCount: Number(maxCount.value),
                    searchHidden: searchHidden.value,
                    maxDepth: Number(maxDepth.value),
                    searchBinary: searchBinary.value,
                    //excludeNotCommon: excludeNotCommon.value,
                    searchAll: searchAll.value,
                    maxColumn: Number(maxColumn.value),
                    rawCodeMode: rawCodeMode.value,
                    multiLine: multiLine.value,
                    curLang: curLang.value
                });
            } catch (e) {
                console.error(e);
            }
        };
        const openFolderDialog = async () => {

            try {
                let folder = await invoke('open_folder_dialog', { pathsOld: searchPath.value, });
                console.log("folder:", folder);
                searchPath.value = folder;
                /*
                if (searchPath.value === homeDir.value) {
                    searchPath.value = folder+"  |  ";
                }else{
                    // 已有不加

                    let re=buildRegex(folder);
                    //console.log("regex:",re);
                    //console.log("re.test result:",re.test(searchPath.value));
                    //console.log("searchPath:",searchPath.value);
                    if (!re.test(searchPath.value)){
                    //if(searchPath.value.search(folder+";")===-1){
                        searchPath.value+=folder+"  |  ";
                    }
                }*/
                //searchPath.value = folderPath;
            } catch (e) {
                console.error(e);
            }
        };
        const getHomeDir = async () => {
            try {
                searchPath.value = await invoke('get_home_dir');
                homeDir.value = searchPath.value;
            } catch (e) {
                console.error(e);
            }
        };

        onMounted(async () => {
            //设置默认搜索目录
            getHomeDir();

            getById('searchPatternInput').focus();

            try {
                curVersion.value = await getVersion();
                curVersion.value = "v" + curVersion.value;

                let latest = await invoke('check_update');
                latestVersion.value = latest.tag_name;
                latestVersionDesc.value = latest.body;

                console.log(latest);
                // if(curVersion.value.toLowerCase()  < latestVersion.value.toLowerCase()){
                //     //alert("有新版本发布！\n当前版本："+curVersion.value+"，最新版本："+latestVersion.value+"\n请前往 https://gitee.com/vvvvvx/fast-full-text-search/releases 下载最新版本。");
                //     getById("alertNewVersionBox").style.display="block";
                // }
            } catch (e) {
                console.log(e);
            }

            listen('rg-output', event => {

                console.log(event.payload);
                output.value.push(...event.payload);

            });


            listen('rg-process-killed', event => {
                console.log(event.payload);
                cmdStatus.value = t(event.payload);
                isDone.value = true;
                alert(t(event.payload));
            });

            listen('completed', event => {
                let keywordsArr = searchPattern.value.trim().split(' ');//split search pattern into two keywords
                let ptrn = keywordsArr.filter(item => item.trim() !== '');//remove empty string

                if (ptrn.length > 1 && regexMode.value === false) {
                    //cmdStatus.value +="->("+output.value.length + ")->完成" ;
                    cmdStatus.value += t('status_search_done_label'); //"->完成" ;
                } else {
                    cmdStatus.value = t(event.payload);
                }
                isDone.value = true;
                closeRuningAlert();
                closeCustomAlert();
                console.log(event.payload);
            });

            listen('get-os', event => {
                OS.value = event.payload;
                console.log(event.payload);
                //alert(OS);
            });
            listen('progress', event => {
                cmdStatus.value = event.payload;

                if (cmdStatus.value.slice(-3) != "...") {
                    isPipMode.value = true;
                }
                //console.log("cmdsStatus:",event.payload.slice(-3));
                //alert(OS);
            });
            listen('overdate', event => {
                alert(t(event.payload));
                console.log(event.payload);
                //alert(OS);
            });
            listen('error', event => {
                alert(event.payload);
                console.log(event.payload);
            });
            //begin 新加
            document.addEventListener('click', handleDropdownClick);
            //end 新加
        });
        //onUnmounted(() => {
        //        console.log("onUnmounted");
        //清理监听\
        //        invoke('kill_rga_process');
        //    });

        //begin 新加
        onBeforeUnmount(() => {
            document.removeEventListener('click', handleDropdownClick);
        });
        //end 新加

        return {
            changeLang,
            resetTableHeader,
            i18n,
            curLang,
            searchPattern,
            searchPath,
            runCommand,
            openFolderDialog,
            getHomeDir,
            filenamePattern,
            output,
            dispHitCount,
            sortOutputByFile,
            sortOutputByHitCount,
            sortOutputByCreated,
            sortOutputByModified,
            preFile,
            cmdStatus,
            searchFilename,
            maxCount,
            OS,
            searchHidden,
            maxDepth,
            searchBinary,
            //begin 新加
            items,
            filteredItems,
            selectedIndex,
            filterItems,
            handleKeydown,
            selectItem,
            togglePush,
            handleDropdownClick,
            clearFilePattern,
            handleSearchKeydown,
            //excludeNotCommon,
            closeCustomAlert,
            closeRuningAlert,
            closeNewVersionAlert,
            toggleSearchAll,
            //end 新加
            regexMode,
            isDone,
            isPipMode,
            displayCreatedAt,
            displayModifiedAt,
            maxColumn,
            searchAll,
            multiLine,
            latestVersion,
            latestVersionDesc,
            curVersion,
            versionText,
            versionTitle,
            forceKillSearch,
            forceStop,
            homeDir,
            clearPath,
            rawCodeMode,
        };
    }

};