<template>
    <div class="header " style="z-index: 1000;min-width: 1000px;">
        <div class="row" style="min-width: 1000px;">
            <div class="col-6 ht-45">
                <div class="input-group mb-0 mt-0 ">
                    <span class="input-group-text dark-mode ht-45">{{ $t('search_location_label') }}</span>
                    <input class="form-control dark-mode ht-45"  v-model="searchPath" id="inputPath" @contextmenu.prevent="clearPath" :placeholder="$t('search_location_placeholder')"
                        :title= "$t('search_location_title')" />
                    <button class="btn btn-secondary pt-1 ht-45" @click="openFolderDialog" style="padding-left: 7px;padding-right: 5px;" :title="$t('search_location_button_title')"><img src="/src/assets/folder.svg"  width="24" height="24"
                                alt="Icon"></button>
                </div>
                <div class="input-group mb-1 mt-0 ">
                    <span class="input-group-text dark-mode mt-2 ht-45 d-flex">{{ $t('search_filetype_label') }}</span>
                    <!--新加Begin-->
                    <div class="form-control dropdown mt-2 ht-45  flex-column" style="padding: 0%;" @click="handleDropdownClick"> 
                        <input  class=" dark-mode mt-0  w-100 ht-43" ref="inputFilePattern" id="inputFilePattern" v-model="filenamePattern"  style="border: none;padding-left: 10px;" @click="handleDropdownClick" @input="filterItems" @keydown="handleKeydown" :placeholder=" $t('search_filetype_placeholder')" 
                        :title="$t('search_filetype_title')">
                        <div v-if="filteredItems.length" class="dropdown-list"  :title="$t('search_filetype_droplist_title')">

                    <!---<button class="btn btn-primary  pt-2 ht-30" @click="openFolderDialog" title="点击选择搜索根路径">添加</button> -->
                            <div v-for="(item, index) in filteredItems" :key="index" @click="selectItem(item)" :class="{'selected': index === selectedIndex}" class="dropdown-item">
                                <span class="push-button" @click.stop="togglePush(item)" :style="{ color: item.include ? 'green' : 'red' }" >{{ item.include ? '✔' : '✘' }}</span>
                                <span class="list-type">{{ item.type }}</span>
                                <span class="list-content">{{ item.content }}</span>
                            </div>
                        </div>
                  </div>
                    <!--  新加End  ✘✔━—㊀㊉＋－-->
                        <button class="btn btn-secondary mt-2 pt-1 ht-45" @click="clearFilePattern" :title="$t('clear_filetype_button_title')">C</button>
                </div>
            </div>
            <div class="col-6 ">
                <div class="input-group mb-0 mt-0 ">
                    <div class="border-light  mt-2">
                        <div class="form-switch d-flex flex-column align-items-end mt-1" style="margin-right: 0.6rem;margin-left:1px;padding-left:1px;"
                            :title="$t('regex_title')">
                            <input type="checkbox" class="form-check-input mt-3" role="switch" id="regex-mode" v-model="regexMode" />
                            <label class="form-check-label d-block text-start mt-1 pt-0 ht-45" for="regex-mode">Regex </label>
                        </div>
                    </div>
                    <!--    <span class="input-group-text dark-mode">搜索模式</span> -->
                    <input class="form-control dark-mode  mt-1 ht-45 search-input" id="searchPatternInput" v-model="searchPattern" @keydown="handleSearchKeydown" :placeholder="$t('search_pattern_placeholder')"
                        :title="$t('search_pattern_title')" />
                    <button class="btn btn-success mt-1 pt-1 ht-45 search-button" @click="runCommand"
                        :title="$t('search_button_title')">{{ $t('search_label') }}</button>
                </div>
            </div>
        </div>
        <div class="row " style="margin-left: 0.2rem;margin-top:3px;min-width: 960px;">
            <div class="col-11 " style="display: flex;flex-wrap: wrap;gap:15px;padding-left: 0px;max-width: 91.66%;min-width: 900px;">
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_filename_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="search-file-name">{{ $t('checkbox_filename_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="search-file-name" v-model="searchFilename" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_hidden_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="search-hidden">{{ $t('checkbox_hidden_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="search-hidden" v-model="searchHidden" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_binary_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="search-binary">{{ $t('checkbox_binary_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="search-binary" v-model="searchBinary" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_rawcode_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="raw_code_mode">{{ $t('checkbox_rawcode_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="raw_code_mode" v-model="rawCodeMode" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_multiline_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="multi_line">{{ $t('checkbox_multiline_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="multi_line" v-model="multiLine" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" @click="toggleSearchAll" :title="$t('checkbox_fullsearch_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="search-all">{{ $t('checkbox_fullsearch_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="search-all"   v-model="searchAll" />
                </div>
                <div class="input-group mt-0" style="width: fit-content;"
                    :title="$t('input_maxcount_title')">
                <!--    <label class="form-check-label mt-0 pt-0 ht-45" for="max-count" style="width: fit-content;">最大匹配次数：</label>-->
                    <span class="input-group-text dark-mode border-0 mt-0 ht-30 d-flex fs-6 toolbar-span">{{ $t('input_maxcount_label') }}</span>
                    <input class="form-control dark-mode mt-0 ht-30" id="max-count" v-model="maxCount"
                        style="width: 50px;" />
                </div>

                <div class=" input-group mt-0" style="width: fit-content;" 
                    :title="$t('input_depth_title')">
                    <span class="input-group-text dark-mode border-0 mt-0 ht-30 d-flex fs-6 toolbar-span">{{ $t('input_depth_label') }}</span>
                    <input class="form-control dark-mode mt-0 ht-30" id="max-depth" v-model="maxDepth"
                        style="width: 50px;" />
                </div>
                
                <div class=" input-group mt-0" style="width: fit-content;" :title="$t('input_maxcolumn_title')">
                    <span class="input-group-text dark-mode border-0 mt-0 ht-30 d-flex fs-6 toolbar-span">{{ $t('input_maxcolumn_label') }}</span>
                    <input class="form-control dark-mode mt-0 ht-30" id="max-column" v-model="maxColumn"
                        style="width: 50px;" />
                </div>

            </div>
            <div class="col-1 justify-content-end" style="display: flex;gap:2px;padding-left: 0px;">
                <button class="btn  mt-0 pt-1 ht-30 btn-warning" @click="forceStop" style="width: 65px;" :title="$t('stop_button_title')">{{ $t('stop_button_label') }}</button>
            </div>

            <div style="display: inline-flex;position: absolute;bottom: 0;gap: 0.5rem;padding-left:0px;padding-right: 0px; width:calc(100% - 280px);">
                <label for="">{{ $t('status_searching_label') }}</label><label :class="['text-success', (isDone || isPipMode) ? '':'blink' ]" for=""><b>{{ cmdStatus }}</b></label>
                &ensp;&ensp;<label>{{ $t('status_searching_output_label') }}{{ output.length }} {{ $t('status_results_records') }}</label>
            </div>
            <div style="display: flex;justify-content:flex-end;align-items:center; gap: 0.5rem;padding-right: 15px;" class="parent-container">
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_display_createdate_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="display-created-at">{{ $t('checkbox_display_createdate_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="display-created-at" v-model="displayCreatedAt" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" :title="$t('checkbox_display_modifiedate_title')">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="display-modified-at">{{ $t('checkbox_display_modifiedate_label') }}</label>
                    <input type="checkbox" class="form-check-input mt-2" id="display-modified-at" v-model="displayModifiedAt" />
                </div>

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
            <thead style="position: sticky; top: 0; background-color: #333;">
                <tr>
                    <th class="text-start fs-6 " style="width: 20px;">{{ $t('table_col_dir_label') }}</th>
                    <th class="text-end fs-6 " style="width: 20px;" :title="$t('table_col_hits_title')">
                        <a href="#" @click.prevent="sortOutputByHitCount" id="sort-by-hit-count"> {{ $t('table_col_hits_label') }} </a>
                    </th>

                    <th class="text-center fs-6 " v-show="displayModifiedAt" style="width: 40px;" :title="$t('table_col_modifiedat_title')"><a href="#" @click.prevent="sortOutputByModified" id="sort-by-modified"> {{ $t('table_col_modifiedat_label') }} </a></th>
                    <th class="text-center fs-6 " v-show="displayCreatedAt" style="width: 40px;" :title="$t('table_col_createdat_title')"><a href="#" @click.prevent="sortOutputByCreated" id="sort-by-created"> {{ $t('table_col_createdat_label') }} </a></th>
                    <th class="text-start fs-6 " :title="$t('table_col_file_title')">
                        <a href="#" @click.prevent="sortOutputByFile" id="sort-by-file">&emsp;{{ $t('table_col_file_label') }} </a>
                    </th>

                </tr>
            </thead>

            <tbody>
                <tr v-for="line in output" :key="line.file">
                    <td class="text-end fs-6 " :title="$t('table_item_dir_title')"><a class="no-underline " href="#"
                            @click.prevent="gotoFolder(line.file)"><img src="/src/assets/folder.svg" class="icon"
                                alt="Icon"></a></td>
                    <td class="text-end fs-6 " :title="$t('table_item_hits_title')">{{ line.hit_count }}</td>
                    <td class="text-center fs-6 " v-show="displayModifiedAt" style="width: 40px;" :title="$t('table_item_modifiedat_title')">{{ line.modified_at}}</td>
                    <td class="text-center fs-6 " v-show="displayCreatedAt" style="width: 40px;" :title="$t('table_item_createdat_title')">{{ line.created_at}}</td>
                    <td class="text-start fs-6 "><a class="no-underline " href="#" @click.prevent="openFile(line.file)"
                            :title="line.content"> {{ line.file }} </a></td>

                </tr>
            </tbody>
        </table>
    </div>
    <div class="container-fluid fixed-bottom py-1" style="min-width: 1000px; max-width: 100%; background-color: #333; z-index: 1050; border-top: 1px solid #444;height:28px;">
        <div class="row align-items-center justify-content-between px-3 g-0 h-100 ">
            <div class="col-auto d-flex align-items-center h-100">
                <div class="lang-switch d-flex align-items-center small">
                    <!-- 💡 使用 @click.prevent 阻止超連結的預設跳轉行為 -->
                    <a 
                    href="#" 
                    class="text-decoration-none mx-2 fs-6" 
                    :class="curLang==='zh' ? 'text-success fw-bold' : 'text-secondary'"
                    @click.prevent="changeLang('zh')"
                    >
                    中文
                    </a>
                    
                    <span class="text-muted opacity-50">|</span>
                    
                    <a 
                     href="#" 
                        class="text-decoration-none mx-2 fs-6" 
                        :class="curLang === 'en' ? 'text-success fw-bold' : 'text-secondary'"
                        @click.prevent="changeLang('en')"
                    >
                    English
                    </a>
                </div>
            </div>
            <div class="col-auto text-white fs-6 h-100 align-items-center d-flex">
                <div class="col-auto">
                    <span :title="$t('developed_by_title')"> <a href="https://github.com/vvvvvx/ripgrep-all-gui" target="_blank">Developed by Viaco.</a>&emsp; Email : 106324221@qq.com&emsp;</span><span :class="[(curVersion < latestVersion && latestVersion!='') ?  'blink':'']" v-html="versionText" :title="versionTitle"></span>
                </div>
            </div>
        </div>
         
    </div>
   <!-- 
    <div class="container-fluid min-vh-100 d-flex flex-column ">
        <div class="row justify-content-end">
            <div class="col-auto">
            <span :title="$t('developed_by_title')"> <a href="https://github.com/vvvvvx/ripgrep-all-gui" target="_blank">Developed by Viaco.</a>&emsp; Email : 106324221@qq.com&emsp;</span><span :class="[(curVersion < latestVersion && latestVersion!='') ?  'blink':'']" v-html="versionText" :title="versionTitle"></span>
            </div>
        </div>
    </div>
-->
    <div id="alertBox" class="custom-alert" >
        <span style=" font-weight: bold;" class="fs-5 text-warning ">{{ searchAll ? $t('msg_text_full_search_very_slow') : $t('msg_text_filetype_empty') }} </span><br><br>
      {{ searchAll ? $t('msg_text_all_type_slow') : $t('msg_text_common_type_slow') }}<br><br>
      <span class="red-text">{{ $t('msg_text_input_filetype_to_speedup') }}</span><br><br><br>
      
      <button @click="closeCustomAlert" id="closeAlertBtn" class="btn btn-primary">{{ $t('button_text_ok') }}</button>
    
    </div>
    <div id="alertRuningBox" class="custom-alert" >
      <span style="font-weight: bold;" class="fs-5 text-warning ">{{ isDone ? $t('msg_text_search_done') : $t('msg_text_search_not_finished') }}</span><br>
      <span style="font-weight: bold;" class="fs-5 text-warning">{{ isDone ? '' : $t('msg_text_be_patient') }}</span><br><br>
      {{ isDone ? $t('msg_text_click_cancel') : $t('msg_text_if_force_stop') }}
      <br><br>
      <div class="d-flex justify-content-between">
      <button @click="forceKillSearch" id="forceKillBtn" class="btn btn-primary">{{ $t('button_text_force_stop') }}</button>
      <button @click="closeRuningAlert" id="closeRuningAlertBtn" class="btn btn-primary">{{ $t('button_text_cancel') }}</button>
      </div>
    
    </div>
    <div id="alertNewVersionBox" class="custom-alert" >
      <span style="font-weight: bold;" class="fs-5">{{ $t('msg_text_new_version_found') }}</span><br><br>
      <span  > {{ $t('msg_text_current_version') }}{{ curVersion }} &emsp; {{ $t('msg_text_latest_version') }}{{ latestVersion }}</span><br>
      {{ $t('msg_text_please_go') }} <a href="https://gitee.com/vvvvvx/fast-full-text-search/releases" target="_blank">https://gitee.com/vvvvvx/fast-full-text-search/releases</a> {{ $t('msg_text_download') }}<br>
      
      <br><br>
      <button @click="closeNewVersionAlert" id="closeNewVersionAlertBtn" class="btn btn-primary">{{ $t('button_text_ok') }}</button>
    
    </div>

</template>

<script >
import { invoke } from '@tauri-apps/api/tauri';
import { nextTick, onBeforeUnmount, onMounted,onUnmounted, ref,computed } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { el, tr } from 'vuetify/locale';
import { getVersion } from '@tauri-apps/api/app';
import { homeDir } from '@tauri-apps/api/path';
import { useI18n } from 'vue-i18n';





function getById(id) {
    return document.getElementById(id);
}
function scrollToTop() {
    getById("outputTable").scrollTop = 0;
}
function resetTableHeader() {
    let sf = getById("sort-by-file");
    let hc = getById("sort-by-hit-count") ;
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
  let re=/^[ ]*[\x00-\xFF]{1,3}[ ]*$/;
  console.log(pattern);
  console.log(re);
  console.log(re.test(pattern));
  return re.test(pattern);

}

function buildRegex(s){
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
        gotoFolder(folderPath) {
            console.log(folderPath);
            invoke('goto_folder', { folderPath: folderPath });
        },
        
    },
    setup() {
        const i18n = useI18n();
        const {t} =useI18n();
        const curLang =computed(() => i18n.locale.value);
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
        const isDone=ref(true); //搜索是否执行结束
        const displayCreatedAt=ref(false); //是否显示创建时间
        const displayModifiedAt=ref(true); //是否显示修改时间
        const maxColumn=ref(200); //匹配结果行的最大显示长度,过长将被省略显示
        const isPipMode=ref(false); //是否启用了pip模式
        const curVersion=ref(''); //当前版本号
        const latestVersion=ref(''); //最新版本号
        const latestVersionDesc=ref(''); //最新版本描述
        const versionText=computed(()=>{return ((curVersion.value.toLowerCase() < latestVersion.value.toLowerCase()) && latestVersion.value!='') ?  `<a href="https://github.com/vvvvvx/ripgrep-all-gui/releases" target="_blank" style="text-decoration:none;color:green;">New version available!</a>`:`<a href="https://github.com/vvvvvx/ripgrep-all-gui/releases" target="_blank" style="text-decoration:none;color:white;">Version: ${curVersion.value}</a>` ;}); //版本号显示文本
        const versionTitle=computed(()=>{return ((curVersion.value.toLowerCase() < latestVersion.value.toLowerCase()) && latestVersion.value!='') ? `Current version：${curVersion.value}  Latest：${latestVersion.value} \n\nVersion update：\n${latestVersionDesc.value}`:"Click me to view version information"}); //版本号鼠标悬停提示

        const items= ref([
            { include: true, type: 'ada', content: '*.adb *.ads' },
            { include: true, type: 'agda', content: '*.agda *.lagda' },
            { include: true, type: 'aidl', content: '*.aidl' },
            { include: true, type: 'alire', content: 'alire.toml' },
            { include: true, type: 'amake', content: '*.bp *.mk' },
            { include: true, type: 'asciidoc', content: '*.adoc *.asc *.asciidoc' },
            { include: true, type: 'asm', content: '*.S *.asm *.s' },
            { include: true, type: 'asp', content: '*.ascx *.ascx.cs *.ascx.vb *.asp *.aspx *.aspx.cs *.aspx.vb' },
            { include: true, type: 'ats', content: '*.ats *.dats *.hats *.sats' },
            { include: true, type: 'avro', content: '*.avdl *.avpr *.avsc' },
            { include: true, type: 'awk', content: '*.awk' },
            { include: true, type: 'bat', content: '*.bat' },
            { include: true, type: 'batch', content: '*.bat' },
            { include: true, type: 'bazel', content: '*.BUILD *.bazel *.bazelrc *.bzl BUILD MODULE.bazel WORKSPACE WORKSPACE.bazel' },
            { include: true, type: 'bitbake', content: '*.bb *.bbappend *.bbclass *.conf *.inc' },
            { include: true, type: 'brotli', content: '*.br' },
            { include: true, type: 'buildstream', content: '*.bst' },
            { include: true, type: 'bzip2', content: '*.bz2 *.tbz2' },
            { include: true, type: 'c', content: '*.[chH] *.[chH].in *.cats' },
            { include: true, type: 'cabal', content: '*.cabal' },
            { include: true, type: 'candid', content: '*.did' },
            { include: true, type: 'carp', content: '*.carp' },
            { include: true, type: 'cbor', content: '*.cbor' },
            { include: true, type: 'ceylon', content: '*.ceylon' },
            { include: true, type: 'clojure', content: '*.clj *.cljc *.cljs *.cljx' },
            { include: true, type: 'cmake', content: '*.cmake CMakeLists.txt' },
            { include: true, type: 'cmd', content: '*.bat *.cmd' },
            { include: true, type: 'cml', content: '*.cml' },
            { include: true, type: 'coffeescript', content: '*.coffee' },
            { include: true, type: 'config', content: '*.cfg *.conf *.config *.ini' },
            { include: true, type: 'coq', content: '*.v' },
            { include: true, type: 'cpp', content: '*.[ChH] *.[ChH].in *.[ch]pp *.[ch]pp.in *.[ch]xx *.[ch]xx.in *.cc *.cc.in *.hh *.hh.in *.inl' },
            { include: true, type: 'creole', content: '*.creole' },
            { include: true, type: 'crystal', content: '*.cr *.ecr Projectfile shard.yml' },
            { include: true, type: 'cs', content: '*.cs' },
            { include: true, type: 'csharp', content: '*.cs' },
            { include: true, type: 'cshtml', content: '*.cshtml' },
            { include: true, type: 'csproj', content: '*.csproj' },
            { include: true, type: 'css', content: '*.css *.scss' },
            { include: true, type: 'csv', content: '*.csv' },
            { include: true, type: 'cuda', content: '*.cu *.cuh' },
            { include: true, type: 'cython', content: '*.pxd *.pxi *.pyx' },
            { include: true, type: 'd', content: '*.d' },
            { include: true, type: 'dart', content: '*.dart' },
            { include: true, type: 'devicetree', content: '*.dts *.dtsi' },
            { include: true, type: 'dhall', content: '*.dhall' },
            { include: true, type: 'diff', content: '*.diff *.patch' },
            { include: true, type: 'dita', content: '*.dita *.ditamap *.ditaval' },
            { include: true, type: 'doc', content: '*.docx *.doc *.odt *.ods *.odf *.rtf *.xls *.xlsx *.pdf' },
            { include: true, type: 'docker', content: '*Dockerfile*' },
            { include: true, type: 'dockercompose', content: 'docker-compose.*.yml docker-compose.yml' },
            { include: true, type: 'dts', content: '*.dts *.dtsi' },
            { include: true, type: 'dvc', content: '*.dvc Dvcfile' },
            { include: true, type: 'ebuild', content: '*.ebuild *.eclass' },
            { include: true, type: 'edn', content: '*.edn' },
            { include: true, type: 'elisp', content: '*.el' },
            { include: true, type: 'elixir', content: '*.eex *.ex *.exs *.heex *.leex *.livemd' },
            { include: true, type: 'elm', content: '*.elm' },
            { include: true, type: 'erb', content: '*.erb' },
            { include: true, type: 'erlang', content: '*.erl *.hrl' },
            { include: true, type: 'fennel', content: '*.fnl' },
            { include: true, type: 'fidl', content: '*.fidl' },
            { include: true, type: 'fish', content: '*.fish' },
            { include: true, type: 'flatbuffers', content: '*.fbs' },
            { include: true, type: 'fortran', content: '*.F *.F77 *.F90 *.F95 *.f *.f77 *.f90 *.f95 *.pfo' },
            { include: true, type: 'fsharp', content: '*.fs *.fsi *.fsx' },
            { include: true, type: 'fut', content: '*.fut' },
            { include: true, type: 'gap', content: '*.g *.gap *.gd *.gi *.tst' },
            { include: true, type: 'gn', content: '*.gn *.gni' },
            { include: true, type: 'go', content: '*.go' },
            { include: true, type: 'gprbuild', content: '*.gpr' },
            { include: true, type: 'gradle', content: '*.gradle *.gradle.kts gradle-wrapper.* gradle.properties gradlew gradlew.bat' },
            { include: true, type: 'graphql', content: '*.graphql *.graphqls' },
            { include: true, type: 'groovy', content: '*.gradle *.groovy' },
            { include: true, type: 'gzip', content: '*.gz *.tgz' },
            { include: true, type: 'h', content: '*.h *.hh *.hpp' },
            { include: true, type: 'haml', content: '*.haml' },
            { include: true, type: 'hare', content: '*.ha' },
            { include: true, type: 'haskell', content: '*.c2hs *.cpphs *.hs *.hsc *.lhs' },
            { include: true, type: 'hbs', content: '*.hbs' },
            { include: true, type: 'hs', content: '*.hs *.lhs' },
            { include: true, type: 'html', content: '*.ejs *.htm *.html' },
            { include: true, type: 'hy', content: '*.hy' },
            { include: true, type: 'idris', content: '*.idr *.lidr' },
            { include: true, type: 'janet', content: '*.janet' },
            { include: true, type: 'java', content: '*.java *.jsp *.jspx *.properties' },
            { include: true, type: 'jinja', content: '*.j2 *.jinja *.jinja2' },
            { include: true, type: 'jl', content: '*.jl' },
            { include: true, type: 'js', content: '*.cjs *.js *.jsx *.mjs *.vue *.ts' },
            { include: true, type: 'json', content: '*.json *.sarif composer.lock' },
            { include: true, type: 'jsonl', content: '*.jsonl' },
            { include: true, type: 'julia', content: '*.jl' },
            { include: true, type: 'jupyter', content: '*.ipynb *.jpynb' },
            { include: true, type: 'k', content: '*.k' },
            { include: true, type: 'kotlin', content: '*.kt *.kts' },
            { include: true, type: 'lean', content: '*.lean' },
            { include: true, type: 'less', content: '*.less' },
            { include: true, type: 'license', content: '*[.-]LICEN[CS]E* AGPL-*[0-9]* APACHE-*[0-9]* BSD-*[0-9]* CC-BY-* COPYING COPYING[.-]* COPYRIGHT COPYRIGHT[.-]* EULA EULA[.-]* GFDL-*[0-9]* GNU-*[0-9]* GPL-*[0-9]* LGPL-*[0-9]* LICEN[CS]E LICEN[CS]E[.-]* MIT-*[0-9]* MPL-*[0-9]* NOTICE NOTICE[.-]* OFL-*[0-9]* PATENTS PATENTS[.-]* UNLICEN[CS]E UNLICEN[CS]E[.-]* agpl[.-]* gpl[.-]* lgpl[.-]* licen[cs]e licen[cs]e.*' },
            { include: true, type: 'lilypond', content: '*.ily *.ly' },
            { include: true, type: 'lisp', content: '*.el *.jl *.lisp *.lsp *.sc *.scm' },
            { include: true, type: 'lock', content: '*.lock package-lock.json' },
            { include: true, type: 'log', content: '*.log' },
            { include: true, type: 'lua', content: '*.lua' },
            { include: true, type: 'lz4', content: '*.lz4' },
            { include: true, type: 'lzma', content: '*.lzma' },
            { include: true, type: 'm4', content: '*.ac *.m4' },
            { include: true, type: 'make', content: '*.mak *.mk [Gg][Nn][Uu]makefile [Gg][Nn][Uu]makefile.am [Gg][Nn][Uu]makefile.in [Mm]akefile [Mm]akefile.am [Mm]akefile.in' },
            { include: true, type: 'mako', content: '*.mako *.mao' },
            { include: true, type: 'man', content: '*.[0-9][cEFMmpSx] *.[0-9lnpx]' },
            { include: true, type: 'markdown', content: '*.markdown *.md *.mdown *.mdwn *.mdx *.mkd *.mkdn' },
            { include: true, type: 'matlab', content: '*.m' },
            { include: true, type: 'meson', content: 'meson.build meson.options meson_options.txt' },
            { include: true, type: 'minified', content: '*.min.css *.min.html *.min.js' },
            { include: true, type: 'mint', content: '*.mint' },
            { include: true, type: 'mk', content: 'mkfile' },
            { include: true, type: 'ml', content: '*.ml' },
            { include: true, type: 'motoko', content: '*.mo' },
            { include: true, type: 'msbuild', content: '*.csproj *.fsproj *.proj *.props *.sln *.targets *.vcxproj' },
            { include: true, type: 'nim', content: '*.nim *.nimble *.nimf *.nims' },
            { include: true, type: 'nix', content: '*.nix' },
            { include: true, type: 'objc', content: '*.h *.m' },
            { include: true, type: 'objcpp', content: '*.h *.mm' },
            { include: true, type: 'ocaml', content: '*.ml *.mli *.mll *.mly' },
            { include: true, type: 'org', content: '*.org *.org_archive' },
            { include: true, type: 'pants', content: 'BUILD' },
            { include: true, type: 'pascal', content: '*.dpr *.inc *.lpr *.pas *.pp' },
            { include: true, type: 'picture', content: '*.[jJ][pP][gG] *.[jJ][pP][eE][gG] *.[bB][mM][pP] *.[pP][nN][gG] *.[gG][iI][fF] *.jpg *.tiff *.raw *.svg *.psd *.eps' },
            { include: true, type: 'pdf', content: '*.pdf' },
            { include: true, type: 'perl', content: '*.PL *.perl *.pl *.plh *.plx *.pm *.t' },
            { include: true, type: 'php', content: '*.php *.php3 *.php4 *.php5 *.php7 *.php8 *.pht *.phtml' },
            { include: true, type: 'po', content: '*.po' },
            { include: true, type: 'pod', content: '*.pod' },
            { include: true, type: 'postscript', content: '*.eps *.ps' },
            { include: true, type: 'prolog', content: '*.P *.pl *.pro *.prolog' },
            { include: true, type: 'protobuf', content: '*.proto' },
            { include: true, type: 'ps', content: '*.cdxml *.ps1 *.ps1xml *.psd1 *.psm1' },
            { include: true, type: 'puppet', content: '*.epp *.erb *.pp *.rb' },
            { include: true, type: 'purs', content: '*.purs' },
            { include: true, type: 'python', content: '*.py *.pyi' },
            { include: true, type: 'qmake', content: '*.prf *.pri *.pro' },
            { include: true, type: 'qml', content: '*.qml' },
            { include: true, type: 'r', content: '*.R *.Rmd *.Rnw *.r' },
            { include: true, type: 'racket', content: '*.rkt' },
            { include: true, type: 'raku', content: '*.p6 *.pl6 *.pm6 *.raku *.rakudoc *.rakumod *.rakutest' },
            { include: true, type: 'rdoc', content: '*.rdoc' },
            { include: true, type: 'readme', content: '*README README*' },
            { include: true, type: 'reasonml', content: '*.re *.rei' },
            { include: true, type: 'red', content: '*.r *.red *.reds' },
            { include: true, type: 'rescript', content: '*.res *.resi' },
            { include: true, type: 'robot', content: '*.robot' },
            { include: true, type: 'rst', content: '*.rst' },
            { include: true, type: 'ruby', content: '*.gemspec *.rb *.rbw .irbrc Gemfile Rakefile config.ru' },
            { include: true, type: 'rust', content: '*.rs' },
            { include: true, type: 'sass', content: '*.sass *.scss' },
            { include: true, type: 'scala', content: '*.sbt *.scala' },
            { include: true, type: 'sh', content: '*.bash *.bashrc *.csh *.cshrc *.ksh *.kshrc *.sh *.tcsh *.zsh .bash_login .bash_logout .bash_profile .bashrc .cshrc .kshrc .login .logout .profile .tcshrc .zlogin .zlogout .zprofile .zshenv .zshrc bash_login bash_logout bash_profile bashrc profile zlogin zlogout zprofile zshenv zshrc' },
            { include: true, type: 'slim', content: '*.skim *.slim *.slime' },
            { include: true, type: 'smarty', content: '*.tpl' },
            { include: true, type: 'sml', content: '*.sig *.sml' },
            { include: true, type: 'solidity', content: '*.sol' },
            { include: true, type: 'soy', content: '*.soy' },
            { include: true, type: 'spark', content: '*.spark' },
            { include: true, type: 'spec', content: '*.spec' },
            { include: true, type: 'sql', content: '*.psql *.sql' },
            { include: true, type: 'stylus', content: '*.styl' },
            { include: true, type: 'sv', content: '*.h *.sv *.svh *.v *.vg' },
            { include: true, type: 'svg', content: '*.svg' },
            { include: true, type: 'swift', content: '*.swift' },
            { include: true, type: 'swig', content: '*.def *.i' },
            { include: true, type: 'systemd', content: '*.automount *.conf *.device *.link *.mount *.path *.scope *.service *.slice *.socket *.swap *.target *.timer' },
            { include: true, type: 'taskpaper', content: '*.taskpaper' },
            { include: true, type: 'tcl', content: '*.tcl' },
            { include: true, type: 'tex', content: '*.bib *.cls *.dtx *.ins *.ltx *.sty *.tex' },
            { include: true, type: 'texinfo', content: '*.texi' },
            { include: true, type: 'textile', content: '*.textile' },
            { include: true, type: 'tf', content: '*.auto.tfvars *.auto.tfvars.json *.terraform.lock.hcl *.terraformrc *.tf *.tf.json *.tfrc terraform.rc terraform.tfvars terraform.tfvars.json' },
            { include: true, type: 'thrift', content: '*.thrift' },
            { include: true, type: 'toml', content: '*.toml Cargo.lock' },
            { include: true, type: 'twig', content: '*.twig' },
            { include: true, type: 'txt', content: '*.txt' },
            { include: true, type: 'typescript', content: '*.cts *.mts *.ts *.tsx' },
            { include: true, type: 'typoscript', content: '*.ts *.typoscript' },
            { include: true, type: 'usd', content: '*.usd *.usda *.usdc' },
            { include: true, type: 'v', content: '*.v *.vsh' },
            { include: true, type: 'vala', content: '*.vala' },
            { include: true, type: 'vb', content: '*.vb' },
            { include: true, type: 'vcl', content: '*.vcl' },
            { include: true, type: 'verilog', content: '*.sv *.svh *.v *.vh' },
            { include: true, type: 'vhdl', content: '*.vhd *.vhdl' },
            { include: true, type: 'vim', content: '*.vim .gvimrc .vimrc _gvimrc _vimrc gvimrc vimrc' },
            { include: true, type: 'vimscript', content: '*.vim .gvimrc .vimrc _gvimrc _vimrc gvimrc vimrc' },
            { include: true, type: 'webidl', content: '*.idl *.webidl *.widl' },
            { include: true, type: 'wiki', content: '*.mediawiki *.wiki' },
            { include: true, type: 'xml', content: '*.dtd *.rng *.sch *.xhtml *.xjb *.xml *.xml.dist *.xsd *.xsl *.xslt' },
            { include: true, type: 'xz', content: '*.txz *.xz' },
            { include: true, type: 'yacc', content: '*.y' },
            { include: true, type: 'yaml', content: '*.yaml *.yml' },
            { include: true, type: 'yang', content: '*.yang' },
            { include: true, type: 'z', content: '*.Z' },
            { include: true, type: 'zig', content: '*.zig' },
            { include: true, type: 'zip', content: '*.[zZ][iI][pP] *.[rR][aA][rR] *.gz *.tgz *.arj *.7z *.tar *.bz2 *.tbz2 *.Z *.lzh *.ace *.jar *.tar.zst' },
            { include: true, type: 'zsh', content: '*.zsh .zlogin .zlogout .zprofile .zshenv .zshrc zlogin zlogout zprofile zshenv zshrc' },
            { include: true, type: 'zstd', content: '*.zst *.zstd' }

            // Add more items as needed
        ]);
        const filteredItems= ref([]);
        const selectedIndex= ref(-1) ;
        const homeDir=ref('');
        let timeoutId= null;
        const rawCodeMode= ref(false);//是否源代码搜索模式

        const changeLang= (lang) => {
            i18n.locale.value = lang
            localStorage.setItem('lang', lang)
        }
        const resetTableHeader = () => {
            let sf = getById("sort-by-file");
            let hc = getById("sort-by-hit-count") ;
            let mt = getById("sort-by-modified");
            let ct = getById("sort-by-created");
            sf.innerText = t('table_col_file_label'); 
            hc.innerText = t('table_col_hits_label'); 
            mt.innerText = t('table_col_modifiedat_label'); 
            ct.innerText = t('table_col_createdat_label'); 
        };
        const filterItems=()=> {
            filteredItems.value = items.value.filter(item => item.content.toLowerCase().includes(filenamePattern.value.toLowerCase()) ||
            item.type.toLowerCase().includes(filenamePattern.value.toLowerCase()) );
            selectedIndex.value = -1;
        };
        const handleSearchKeydown=(event)=> {
            if (event.key === 'Enter') {
                event.preventDefault();
                runCommand();
            }
        };
        const handleKeydown=(event)=> {
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
                if(filenamePattern.value.trim().length >0 ){
                    console.log("Keydown Enter:"+filenamePattern.value);
                    runCommand();
                }
            } else if (event.ctrlKey && event.key === ' '){
                if (selectedIndex.value >= 0) {
                togglePush(filteredItems.value[selectedIndex.value]);
                }
            }
        };
        const clearFilePattern=()=> {
            filenamePattern.value = '';
            filteredItems.value = [];
            getById('inputFilePattern').focus();
        };
        const clearPath=()=> {
            if(searchPath.value.trim().length >0){
                searchPath.value = '';
            }else{
                searchPath.value = homeDir.value;
            }
            
            getById('inputPath').focus();
        };
        const processExclude=(input)=>{
            return input.replace(/\S+/g,'!$&');
        };
        const selectItem=(item)=> {
            if (item.include) {
                filenamePattern.value =item.content;
            }else{
                filenamePattern.value = processExclude(item.content);
            }
            filteredItems.value = [];
        };
        const togglePush=(item)=> {
            item.include = !item.include;
        };
        const handleDropdownClick=(event)=> {
            if (!event.target.closest('.dropdown-list')) {
                filteredItems.value = [];
            }
        };
        ////end 新加

        const showCustomAlert= ()=> {
          getById("alertBox").style.display="block";
          getById("closeAlertBtn").focus();

          clearTimeout(timeoutId);
          timeoutId = setTimeout(() => {
            closeCustomAlert();
          }, 10000);
          console.log(getById("alertBox").style.display);
        };

        const closeCustomAlert=()=>{
          getById("alertBox").style.display="none";
          console.log(getById("alertBox").style.display);
          console.log("执行隐藏对话框");
        };

        const showRuningAlert=()=>{
          getById("alertRuningBox").style.display="block";
          getById("closeRuningAlertBtn").focus();

          clearTimeout(timeoutId);
          timeoutId = setTimeout(() => {
            closeRuningAlert();
          }, 10000);
        };
        const closeRuningAlert=()=>{
          getById("alertRuningBox").style.display="none";
        };
        const closeNewVersionAlert=()=>{
          getById("alertNewVersionBox").style.display="none";
        };


         
        const sortOutputByHitCount = () => {
            let hc = getById("sort-by-hit-count");
            //let sf = getById("sort-by-file");

            //let text=curLang.value==="zh"?"命中":"Hits";
            let text=t('table_col_hits_label').slice(0,-1); //去掉最后的符号


            if (hc.innerText === text+"-") {
                //↑↓
                //descending
                resetTableHeader();
                hc.innerText = text+"↓";
                output.value.sort((a, b) => b.hit_count - a.hit_count);
            } else if (hc.innerText === text+"↓") {
                //ascending   
                hc.innerText = text+"↑";
                output.value.sort((a, b) => a.hit_count - b.hit_count);
            } else {
                //descending
                hc.innerText = text+"↓";
                output.value.sort((a, b) => b.hit_count - a.hit_count);
            }
            //sf.innerText = "文件-";
            scrollToTop();
        };
        const sortOutputByFile = () => {
            let sf = getById("sort-by-file");
            //let hc = getById("sort-by-hit-count");

            //let text=curLang.value==="zh"?"文件":"File"; 
            let text=t('table_col_file_label').slice(0,-1); //去掉最后的符号

            if (sf.innerText === text+"-") {
                //ascending
                resetTableHeader();
                sf.innerText = text+"↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            } else if (sf.innerText === text+"↑") {
                //descending
                sf.innerText = text+"↓";
                output.value.sort((a, b) => b.file.localeCompare(a.file));
            } else {
                //ascending
                sf.innerText = text+"↑";
                output.value.sort((a, b) => a.file.localeCompare(b.file));
            }

            //hc.innerText = "命中-";
            scrollToTop();
        };
        const sortOutputByCreated = () => {
            let sc = getById("sort-by-created");
            let text=t('table_col_createdat_label').slice(0,-1); //去掉最后的符号
            if (sc.innerText === text+"-") {
                //ascending
                resetTableHeader();
                sc.innerText = text+"↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            } else if (sc.innerText === text+"↑") {
                //descending
                sc.innerText = text+"↓";
                output.value.sort((a, b) => b.created_at.localeCompare(a.created_at));
            } else {
                //ascending
                sc.innerText = text+"↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            }
            scrollToTop();
        };
        const sortOutputByModified = () => {
            let sm = getById("sort-by-modified");
            let text=t('table_col_modifiedat_label').slice(0,-1); //去掉最后的符号

            if (sm.innerText === text+"-") {
                //ascending
                resetTableHeader();
                sm.innerText = text+"↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            } else if (sm.innerText === text+"↑") {
                //descending
                sm.innerText = text+"↓";
                output.value.sort((a, b) => b.modified_at.localeCompare(a.modified_at));
            } else {
                //ascending
                sm.innerText = text+"↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            }

            scrollToTop();
        };
        const toggleSearchAll=()=>{
            if(!searchAll.value){
                searchAll.value=true;
                filenamePattern.value="";
                getById("inputFilePattern").placeholder=t('search_filetype_fullsearch_label'); //"执行[全面搜索]，忽略指定类别。最全面也最慢！";
            }else{
                searchAll.value=false;
                getById("inputFilePattern").placeholder=t('search_filetype_placeholder');//"默认空，搜常用类别。例：*.zip  *.pdf 。指定类别，速度倍增。";
            }
        };
        const forceKillSearch=async ()=>{
            try {
                await invoke('kill_rga_process');
                closeRuningAlert();
              } catch (e) {
                console.error(e);
              }
        }
        const forceStop=async ()=>{
            if(!isDone.value){
                let confirmResult =await confirm(t('msg_stop_confirm')); //confirm("确定要终止当前搜索？");
                if( confirmResult){
                    forceKillSearch();
                }
            }else{
                //alert("当前无搜索任务进行。");
                alert(t('msg_no_task')); //"当前无搜索任务进行。"
            }
        }
        const runCommand =async () => {

            if (!isDone.value) {
              //alert("上一搜索未结束...\n请耐心等待。\n\n如欲强制终止，请关闭本程序后重新打开。");
              console.log("runCommand:isDone:",isDone.value);
              showRuningAlert();
              

              return;
            }
            if(isPatternNotOK(searchPattern.value) && !regexMode.value && !searchFilename.value){
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
            if(searchAll.value || (filenamePattern.value.trim() === '' && !searchFilename.value )){
                //alert('【文件类别】未指定，将搜索所有类别的文件，速度较慢！\n\n如需终止搜索，请关闭本程序。\n\n请耐心等待！');
                showCustomAlert();
            }
            isPipMode.value=false;
            isDone.value=false;

            let keywordsArr = searchPattern.value.trim().split(' ');//split search pattern into two keywords
            let ptrn = keywordsArr.filter(item => item.trim() !== '');//remove empty string

            output.value = [];//clear output before running new command
            // reset 表头排序
            resetTableHeader();
            preFile.value = "";//reset preFile before running new command
            if (ptrn.length > 1 && !regexMode.value ) {
                cmdStatus.value = t('status_pipe_mode_label'); //"管道模式中(较耗时)...";
                //isPipMode.value=true;
            }else if (searchFilename.value){
                cmdStatus.value = t('status_filename_mode_label'); //"文件名搜索中...";
            }else{
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
                    maxCount: Number(maxCount.value) , 
                    searchHidden: searchHidden.value , 
                    maxDepth: Number(maxDepth.value) , 
                    searchBinary: searchBinary.value ,
                    //excludeNotCommon: excludeNotCommon.value,
                    searchAll: searchAll.value,
                    maxColumn:Number(maxColumn.value), 
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
                let folder = await invoke('open_folder_dialog',{pathsOld:searchPath.value,});
                console.log("folder:",folder);
                searchPath.value=folder;
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
                curVersion.value="v"+curVersion.value;

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
                cmdStatus.value = t(event.payload) ;
                isDone.value=true;
                alert(t(event.payload));
            });

            listen('completed', event => {
                let keywordsArr = searchPattern.value.trim().split(' ');//split search pattern into two keywords
                let ptrn = keywordsArr.filter(item => item.trim() !== '');//remove empty string

                if (ptrn.length > 1 && regexMode.value === false) {                    
                    //cmdStatus.value +="->("+output.value.length + ")->完成" ;
                    cmdStatus.value += t('status_search_done_label'); //"->完成" ;
                }else{
                    cmdStatus.value = t(event.payload);
                }
                isDone.value=true;
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

                if(cmdStatus.value.slice(-3)!="...") {
                    isPipMode.value=true;
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

</script>

<style scoped>
/*dropdown 开始*/

.dropdown {
    position: relative;
    display: inline-block;
    z-index: 1000;
}

.dropdown-list {
    display: block;
    position: absolute;
    background-color: #3d3a2e;
    min-width: 200px;
    box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
    z-index: 1000;
}

.dropdown-item {
    padding: 0px;
    padding-left: 10px;
    padding-right: 10px;
    display: flex;
    align-items: center;
    cursor: pointer;
    z-index: 1001;
}

.dropdown-item:hover, .dropdown-item.selected {
    background-color: #4d6789;
}

.push-button {
    margin-right: 10px;
    width: 20px;
    height: 20px;
    border: 1px solid #ccc;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px; /* 调整字体大小以适应按钮尺寸 */
}

.list-type {
    margin-right: 10px;
    font-weight: bold;
    color: white;
}

.list-content {
    flex-grow: 1;
    text-align: left !important;
    align-items: left !important;
    color: yellowgreen;
}

/*dropdown 结束 */

.header {
    position: fixed;
    top: 2px;
    left: 0;
    width: 100%;
    height: 160px;/* 200px */
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
    margin-top: 162px;/* 202px */
    margin-left: 0px;
    padding-top: 0px;
    padding-left: 15px;
    padding-right: 15px;
    height: calc(100% - 190px);/* 230px */
    overflow-y: auto;
    background-color: #333;
    color: white;
}


body,
html {
    height: 100%;
    margin: 0;
    display: flex;
    flex-direction: column;
}
.icon {
    width: 22px;
    height: 22px;
    vertical-align: middle;
     

}

.ht-45 {
    height: 38px;
}
.ht-44{

    height: 37px;
}
.ht-43 {
    height: 36px;
}
.ht-30 {
    height: 30px;
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 14px; /* 调整字体大小以适应按钮尺寸 */
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
    overflow: auto;
    /*启用横向和竖向滚动 */
    border: 0px solid #ccc;

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

.custom-alert {
  display: none; 
  background:#333; 
  position: fixed; 
  top: 50%; 
  left: 50%; 
  transform: translate(-50%,-50%); 
  padding: 20px; 
  border: 2px solid #ccc; 
  box-shadow: 0 0 10px rgba(0,0,0,0.2); 
  text-align:center; 
}

.red-text {
  color: red;
  font-weight: bold;
}

.blink {
  animation: blink 1s infinite;
}
@keyframes blink {
  0% { opacity: 1;}
  50% { opacity: 0;}
  100% { opacity:1; }
}
/* 凸显搜索输入框 */
.search-input {
  width: 60%;
  height: 80px;
  font-size: 16px;
  padding: 8px 12px;
  border: 2px solid #4CAF50; /* 亮蓝色边框 4A90E2*/
  border-radius: 6px;
  background-color: #2A2D32; /* 深灰色背景，更符合暗色主题 */
  color: #FFFFFF; /* 文本颜色 */
  transition: all 0.3s ease-in-out;
}

.search-input::placeholder {
  color: rgba(255, 255, 255, 0.5); /* 让占位符更易读 */
}

.search-input:focus {
  border-color: #1976D2;
  background-color: #3A3F44; /* 聚焦时变浅一点，形成交互感 */
  box-shadow: 0 0 8px rgba(74, 144, 226, 0.6); /* 添加蓝色发光效果 */
  outline: none;
}

/* 3. 搜索按钮的优化 */
.search-button {
  height: 80px;
  padding: 0 16px;
  font-size: 16px;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  margin-left: 10px; /* 让按钮与输入框有适当间距 */
  transition: background 0.3s;
}

.search-button:hover {
  background-color: #005BBB; /* 悬停时颜色变深 */
}

/* 4. 让输入框自动获得焦点时有动画（可选） */
@keyframes input-focus-animation {
  0% {
    transform: scale(1);
    box-shadow: 0 0 0px rgba(33, 150, 243, 0);
  }
  100% {
    transform: scale(1.02);
    box-shadow: 0 0 8px rgba(33, 150, 243, 0.6);
  }
}

.search-input:focus {
  animation: input-focus-animation 0.2s ease-in-out;
}

.toolbar-span {
    padding-right:6px;
    padding-top:6px;
    padding-left: 0px;
}
</style>
