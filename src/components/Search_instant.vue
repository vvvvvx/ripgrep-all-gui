<template>


    <div class="header ">
        <div class="row">
            <div class="col-12 ">
                <div class="input-group mb-0 mt-0 ">
                    <span class="input-group-text dark-mode ht-45">搜索路径</span>
                    <input class="form-control dark-mode ht-45" v-model="searchPath" placeholder="Enter search path"
                        title="全盘搜索时间也不会太久，&#10;但缩小搜索范围，会大大缩短搜索时间。" />
                    <button class="btn btn-primary  pt-1 ht-45" @click="openFolderDialog" title="点击选择搜索根路径">...</button>
                </div>
            </div>
            <div class="col-4 ">
                <div class="input-group mb-0 mt-0 ">
                    <span class="input-group-text dark-mode mt-1 ht-45">文件类型</span>
                    <input class="form-control dark-mode mt-1 ht-45" v-model="fileType" placeholder="空格分隔，如：docx zip"
                        title="默认为空，搜索所有文件类型。&#10;指定文件类型可大大缩短搜索时间。" />
                </div>
            </div>
            <div class="col-8 ">
                <div class="input-group mb-0 mt-0 ">
                    <!--    <span class="input-group-text dark-mode">搜索模式</span> -->
                    <div class="border-light  mt-1">
                        <div class="form-check mt-0" style="margin-right: 0.6rem;"
                            title="勾选此项，启用正则表达式模式。&#10;注意：启用正则表达式模式，搜索速度会变慢。">
                            <label class="form-check-label mt-0 pt-0 ht-45" for="regex-mode">Regex </label>
                            <input type="checkbox" class="form-check-input mt-2" id="regex-mode" v-model="regexMode" />
                        </div>
                    </div>
                    <input class="form-control dark-mode  mt-1 ht-45" v-model="searchPattern"
                        placeholder="Enter search pattern"
                        title="支持正则表达式，如：.*\.(txt|md) 。&#10;支持空格分隔两个关键字，表示同时满足这两个关键字" />
                    <button class="btn btn-primary mt-1 pt-1 ht-45" @click="runCommand"
                        title="为缩短搜索时间，程序会多线程并发搜索。&#10;因此，CPU占用率很高是正常现象！">搜索</button>
                </div>
            </div>
        </div>
        <div class="row" style="margin-left: 0.2rem;">

            <div class="form-check mt-0" style="width: fit-content;" title="只搜索文件名">
                <label class="form-check-label mt-0 pt-0 ht-45" for="search-file-name">只搜文件名</label>
                <input type="checkbox" class="form-check-input mt-2" id="search-file-name" v-model="searchFilename" />
            </div>
            <div class="form-check mt-0" style="width: 260px;display: inline-flex;"
                title="单个文件最大匹配次数, 0 表示无限制。过大可能会导致搜索时间过长。">
                <label class="form-check-label mt-0 pt-0 ht-45" for="max-count" style="width: 180px;">最大匹配次数:</label>
                <input class="form-control dark-mode mt-1 ht-30" id="max-count" v-model="maxCount"
                    style="width: 70px;" />
            </div>

            <div style="display: inline-flex;position: absolute;bottom: 0;gap: 0.5rem;">
                <label for="">执行状态：</label><label class="text-success" for=""><b>{{ cmdStatus }}</b></label>
                &ensp;&ensp;<label>搜索结果：{{ output.length }} 条</label>
            </div>
        </div>
    </div>
    <!--
    <div class="sidebar " >
        <button class="btn btn-primary mt-1 p-1 pt-1 ht-45" style="width: 45px;"  @click="sortOutput" title="搜索结果排序">↑↓</button>
    </div>
       -->
    <div class="content scrollable flex-height" id="outputTable">
        <table class="table  table-sm text-white " v-show="output.length > 0">
            <thead style="position: sticky; top: 0; z-index: 1;background-color: #333;">
                <tr>
                    <th class="text-start fs-6 " style="width: 20px;">目录</th>
                    <th class="text-end fs-6 " style="width: 20px;" title="点击按命中次数排序">
                        <a href="#" @click.prevent="sortOutputByHitCount" id="sort-by-hit-count"> 命中< </a>
                    </th>
                    <th class="text-start fs-6 " title="点击按文件名排序">
                        <a href="#" @click.prevent="sortOutputByFile" id="sort-by-file">文件< </a>
                    </th>

                </tr>
            </thead>
            <tbody>
                <tr v-for="line in output" :key="line.file">
                    <td class="text-end fs-6 " title="转到文件所在目录"><a class="no-underline " href="#"
                            @click.prevent="gotoFolder(line.file)"><img src="/src/assets/folder.svg" class="icon"
                                alt="Icon"></a></td>
                    <td class="text-end fs-6 ">{{ line.hitCount }}</td>
                    <td class="text-start fs-6 "><a class="no-underline " href="#" @click.prevent="openFile(line.file)"
                            :title="line.content"> {{ line.file }} </a></td>

                </tr>
            </tbody>
        </table>
        <!--
        <ol>
            <li class="text-start fs-6 " v-for="line in output" :key="line.file" :title="line.content">
                <a class="no-underline " href="#" @click.prevent="openFile(line.file)" v-if="dispHitCount && line.sn!== ''" >
                    [ {{ line.sn }} ] {{ line.file }} </a>
                <a class="no-underline " href="#" @click.prevent="openFile(line.file)" v-else > {{ line.file }} </a>
            </li>
        </ol>
        -->
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';


function getById(id) {
    return document.getElementById(id);
}
function scrollToTop() {
    getById("outputTable").scrollTop = 0;
}

export default {
    methods: {
        openFile(filePath) {
            console.log(filePath);
            invoke('open_file', { filePath: filePath });
        },
        gotoFolder(folderPath) {
            console.log(folderPath);
            invoke('goto_folder', { folderPath: folderPath });
        },

    },
    setup() {
        const searchPattern = ref(''); // 搜索模式
        const output = ref([]); // 搜索结果数组
        const searchPath = ref(''); // 搜索路径
        const fileType = ref(''); // 搜索文件类型，空格分割
        const regexMode = ref(false);// 是否启用正则表达式模式
        const dispHitCount = ref(false); // 是否显示命中次数
        const preFile = ref('');// 记录上一个搜到的文件名，用于去重
        const cmdStatus = ref(''); // 命令状态
        const searchFilename = ref(false); // 是否只搜索文件名
        const maxCount = ref(500); // 单个文件最大匹配次数

        const sortOutputByHitCount = () => {
            let hc = getById("sort-by-hit-count");
            let sf = getById("sort-by-file");

            if (hc.innerText === "命中<") {
                //↑↓
                //descending
                hc.innerText = "命中↓";
                output.value.sort((a, b) => b.hitCount - a.hitCount);
            } else if (hc.innerText === "命中↓") {
                //ascending   
                hc.innerText = "命中↑";
                output.value.sort((a, b) => a.hitCount - b.hitCount);
            } else {
                //descending
                hc.innerText = "命中↓";
                output.value.sort((a, b) => b.hitCount - a.hitCount);
            }
            sf.innerText = "文件<";
            scrollToTop();
        };
        const sortOutputByFile = () => {
            let sf = getById("sort-by-file");
            let hc = getById("sort-by-hit-count");

            if (sf.innerText === "文件<") {
                //ascending
                sf.innerText = "文件↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            } else if (sf.innerText === "文件↑") {
                //descending
                sf.innerText = "文件↓";
                output.value.sort((a, b) => b.file.localeCompare(a.file));
            } else {
                //ascending
                sf.innerText = "文件↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            }

            hc.innerText = "命中<";
            scrollToTop();
        };
        const runCommand = () => {

            if (searchPattern.value === '') {    //if search pattern is empty, show error message and return
                alert('搜索模式不能为空');
                return;
            }

            if (searchPath.value === '') {    //if search path is empty, show error message and return  
                alert('搜索路径不能为空');
                return;
            }
            let t = searchPattern.value.trim().split(' ');//split search pattern into two keywords
            let ptrn = t.filter(item => item.trim() !== '');//remove empty string

            if (ptrn.length > 2) {//if search pattern has more than two keywords, show error message and return
                alert('搜索模式最多只能有两个关键字，请重新输入');
                return;
            }
            if (isNaN(maxCount.value) || maxCount.value < 0) {
                alert('最大匹配次数必须为0，或正整数');
                return;
            }
            output.value = [];//clear output before running new command
            // reset
            getById("sort-by-file").innerText = "文件<";
            getById("sort-by-hit-count").innerText = "命中<";
            preFile.value = "";//reset preFile before running new command
            cmdStatus.value = "搜索中...";
            invoke('run_rg_command', { searchPattern: searchPattern.value.trim(), searchPath: searchPath.value, fileType: fileType.value, regexMode: regexMode.value, dispHitCount: dispHitCount.value, searchFilename: searchFilename.value, maxCount: Number(maxCount.value) });
        };
        const openFolderDialog = async () => {
            try {
                searchPath.value = await invoke('open_folder_dialog');
                //searchPath.value = folderPath;
            } catch (e) {
                console.error(e);
            }
        };
        const getHomeDir = async () => {
            try {
                searchPath.value = await invoke('get_home_dir');
            } catch (e) {
                console.error(e);
            }
        };
        onMounted(() => {
            getHomeDir();

            listen('rg-output', event => {

                if (searchFilename.value) {

                    output.value.push({ file: event.payload, content: '' });
                    return;
                }
                // 找到第一个冒号的位置
                const firstIndex = event.payload.indexOf(':');
                // 如果找不到冒号，返回原始字符串和空字符串
                if (firstIndex === -1) {
                    output.value.push({ file: event.payload, content: '' });
                    return;
                }
                // 使用slice方法分割字符串
                let firstPart = event.payload.slice(0, firstIndex);
                let secondPart = event.payload.slice(firstIndex + 1);
                // 非显示命中次数模式，文件名去重。把命中字符串content和前次命中字符串合并

                if (firstPart === preFile.value) {
                    // 去重
                    output.value[output.value.length - 1].hitCount += 1;
                    // 命中次数大于20时，显示前20个命中内容,其他忽略
                    if (output.value[output.value.length - 1].hitCount <= 20) {
                        output.value[output.value.length - 1].content += "\n" + secondPart;
                    }
                    return;
                }
                // 更改preFile为当前文件名
                preFile.value = firstPart;
                output.value.push({ hitCount: 1, file: firstPart, content: secondPart });
            });
            listen('completed', event => {
                console.log(event.payload);
                cmdStatus.value = event.payload;
                // if (event.payload.exit status === 0) {
                //     cmdStatus.value = "搜索完成！";
                // } else {
                //     cmdStatus.value = "搜索失败！";
                // }
            });
        });

        return {
            searchPattern,
            searchPath,
            runCommand,
            openFolderDialog,
            getHomeDir,
            fileType,
            output,
            dispHitCount,
            sortOutputByFile,
            sortOutputByHitCount,
            preFile,
            cmdStatus,
            searchFilename,
            maxCount,
            regexMode
        };
    }

};

</script>
<style scoped>
.header {
    position: fixed;
    top: 0px;
    left: 0;
    width: 100%;
    height: 200px;
    background-color: #333;
    color: white;
    text-align: center;
    line-height: 30px;
    padding-left: 10px;
    padding-right: 10px;
}

.sidebar {
    position: fixed;
    top: 200px;
    left: 0;
    width: 65px;
    height: calc(100% - 200px);
    background-color: #333;
    color: white;
    overflow-y: auto;
    padding-left: 10px;
}

.content {
    margin-top: 200px;
    margin-left: 0px;
    padding-top: 0px;
    padding-left: 15px;
    height: calc(100% - 205px);
    overflow-y: auto;
}

/*
body,
html {
    height: 100%;
    margin: 0;
    display: flex;
    flex-direction: column;
}
*/
.icon {
    width: 28px;
    height: 28px;
    vertical-align: middle;
}

.ht-45 {
    height: 45px;
}

.ht-30 {
    height: 30px;
}

a {
    color: inherit;
    text-decoration: none;
}

a:hover {
    color: #535bf2;
}

.scrollable {
    overflow-x: auto;
    /* 允许横向滚动 */
    overflow-y: auto;
    /* 允许竖向滚动 */
    white-space: nowrap;
    /* 不换行 */
    /*width: 100%;
     设置宽度 */
    border: 1px solid #ccc;
    /* 可选，边框样式 */

}

.flex-height {
    flex-grow: 1;
    overflow: auto;


}

.scroll-container {
    /* width: 100%;
     设置容器宽度 */
    /*height: 800px;
     设置容器高度 */

    overflow: auto;
    /*启用横向和竖向滚动 */
    border: 0px solid #ccc;
    /* 添加边框以便于查看滚动区域 
    padding-left: 0px;
    padding-top: 5px;
    padding-right: 0px;
    margin: 20px;
    margin-right: 20px;*/
}

/*
.scroll-container ul {
    padding: 0;
    margin: 0;
    list-style: none;
}

.scroll-container li {
    padding: 5px;
    background-color: #f9f9f9;
    border-bottom: 1px solid #ddd;
}
*/
.scroll-container li:nth-child(odd) {
    /*background-color: #f2f2f2;
     奇数行颜色 */
}

.scroll-container li:nth-child(even) {
    /*background-color: #ffffff;
     偶数行颜色 */


}

.light-mode {
    background-color: white;
    color: black;
}

.dark-mode {
    background-color: #2c3137;
    /* 深色背景 #343a40 */
    color: white;
    /* 深色模式下的文字颜色 */
}
</style>
