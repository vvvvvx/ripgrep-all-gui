<template>
    <div class="header " style="z-index: 1000;">
        <div class="row">
            <div class="col-6 ht-45">
                <div class="input-group mb-0 mt-0 ">
                    <span class="input-group-text dark-mode ht-45">搜索位置</span>
                    <input class="form-control dark-mode ht-45" v-model="searchPath" placeholder="Enter search path"
                        title="搜索根目录&#10;&#10;全盘搜索时间也不会太久，&#10;但缩小搜索范围，会大大缩短搜索时间。" />
                    <button class="btn btn-secondary pt-1 ht-45" @click="openFolderDialog" title="点击选择搜索根路径">...</button>
                </div>
                <div class="input-group mb-1 mt-0 ">
                    <span class="input-group-text dark-mode mt-2 ht-45 d-flex">文件类别</span>
                    <!--新加Begin-->
                    <div class="form-control dropdown mt-2 ht-45  flex-column" style="padding: 0%;" @click="handleDropdownClick"> 
                        <input  class=" dark-mode mt-0  w-100 ht-43" ref="inputFilePattern" id="inputFilePattern" v-model="filenamePattern"  style="border: none;padding-left: 10px;" @click="handleDropdownClick" @input="filterItems" @keydown="handleKeydown" placeholder="默认空，搜所有类别，较慢。可输：*.zip  *.pdf  ==>全文关键字在右框输入" title="文件类别，支持通配符。&#10;将只搜索指定类别的所有文件&#10;&#10;用空格分隔多个类别。默认为空，即搜索所有类别，速度较慢。&#10;不建议留空，明确文件类别将成倍提高搜索速度。&#10;&#10;例如：&#10;*.docx *.pdf  表示只搜索扩展名为 docx 和 pdf 的两类文件。&#10;*研究报告*.*  表示只搜索文件名包含“研究报告”的任何类型文件。&#10;*研究报告*.pdf   表示只搜索文件名包含“研究报告”的 pdf 文件。&#10;&#10;叹号 !，表示排除，例如：&#10;!*.txt 表示搜索会排除扩展名为 txt 的文件。&#10;!*研究报告*.* 表示搜索会排除文件名包含“研究报告”的任何类型文件。&#10;!*研究报告*.pdf 表示搜索会排除文件名包含“研究报告”的 pdf 文件。">
                        <div v-if="filteredItems.length" class="dropdown-list"  title="✔ Include 包含 此特征&#9;✘ Exclude 排除 此特征&#10;&#10;点击复选框/Ctrl+Space切换&#10;&#10;上下方向键选中，鼠标点击/Enter确认">

                    <!---<button class="btn btn-primary  pt-2 ht-30" @click="openFolderDialog" title="点击选择搜索根路径">添加</button> -->
                            <div v-for="(item, index) in filteredItems" :key="index" @click="selectItem(item)" :class="{'selected': index === selectedIndex}" class="dropdown-item">
                                <span class="push-button" @click.stop="togglePush(item)" :style="{ color: item.include ? 'green' : 'red' }" >{{ item.include ? '✔' : '✘' }}</span>
                                <span class="list-type">{{ item.type }}</span>
                                <span class="list-content">{{ item.content }}</span>
                            </div>
                        </div>
                  </div>
                    <!--  新加End  ✘✔━—㊀㊉＋－-->
                        <button class="btn btn-secondary mt-2 pt-1 ht-45" @click="clearFilePattern" title="点击清除文件类别">C</button>
                </div>
            </div>
            <div class="col-6 ">
                <div class="input-group mb-0 mt-0 ">
                    <div class="border-light  mt-2">
                        <div class="form-switch d-flex flex-column align-items-end mt-1" style="margin-right: 0.6rem;margin-left:1px;padding-left:1px;"
                            title="勾选此项，启用正则表达式模式。&#9;&#10;勾选后，右侧框输入内容将被视为正则表达式">
                            <input type="checkbox" class="form-check-input mt-3" role="switch" id="regex-mode" v-model="regexMode" />
                            <label class="form-check-label d-block text-start mt-1 pt-0 ht-45" for="regex-mode">Regex </label>
                        </div>
                    </div>
                    <!--    <span class="input-group-text dark-mode">搜索模式</span> -->
                    <input class="form-control dark-mode  mt-1 ht-45 search-input" v-model="searchPattern" @keydown="handleSearchKeydown" placeholder="全文搜索关键字  如：市场调研 销售数据 人工智能"
                        title="全文搜索关键字&#10;&#10;1. 支持正则表达式（需开启Regex模式）。&#10;2. 普通模式：即单关键字搜索，最常用！&#10;3. 管道模式：空格分隔多关键字，将漏斗式逐关键字过滤，较耗时。&#10;&#10;注意：&#10;管道模式下，低频关键字靠前放有利于缩短搜索时间&#10;管道模式下，每关键字仅显示一次命中结果&#10;&#10;正则说明：&#10;1. 全文搜索时，支持高级正则模式先行和后行断言，如：(?=.*K1)(?=.*K2)(?=.*K3)&emsp; 表示字符串同时含K1、K2、K3&#10;2. 仅搜文件名 时，默认为普通正则模式，不必勾选Regex" />
                    <button class="btn btn-success mt-1 pt-1 ht-45 search-button" @click="runCommand"
                        title="为缩短搜索时间，程序会多线程并发搜索。&#10;因此，CPU占用率很高是正常现象！">搜索</button>
                </div>
            </div>
        </div>
        <div class="row" style="margin-left: 0.2rem;margin-top:3px;">

            <div class="form-check mt-0" style="width: fit-content;" title="只搜索文件名，不搜索内容。">
                <label class="form-check-label mt-0 pt-0 ht-45" for="search-file-name">只搜文件名</label>
                <input type="checkbox" class="form-check-input mt-2" id="search-file-name" v-model="searchFilename" />
            </div>
            <div class="form-check mt-0" style="width: fit-content;" title="默认不搜索隐藏文件，勾选此项，将搜索隐藏文件">
                <label class="form-check-label mt-0 pt-0 ht-45" for="search-hidden">搜隐藏文件</label>
                <input type="checkbox" class="form-check-input mt-2" id="search-hidden" v-model="searchHidden" />
            </div>
            <div class="form-check mt-0" style="width: fit-content;" title="把二进制文件作为文本搜索&#10;&#10;速度较慢，输出内容可能包含非法字符。">
                <label class="form-check-label mt-0 pt-0 ht-45" for="search-binary">搜二进制文件</label>
                <input type="checkbox" class="form-check-input mt-2" id="search-binary" v-model="searchBinary" />
            </div>
            <div class="form-check mt-0" style="width: fit-content;" title="排除不常用且耗时的图片、压缩文件、数据库、视频文件">
                <label class="form-check-label mt-0 pt-0 ht-45" for="">排除冷门耗时文件</label>
                <input type="checkbox" class="form-check-input mt-2"  v-model="excludeNotCommon" />
            </div>
            <div class="input-group mt-0 " style="width: fit-content;display: inline-flex;padding-left:0px;padding-right:0px;"
                title="单个文件中出现的关键字次数达到[最大匹配次数]后，程序将不再搜索此文件，以提高效率。&#10;&#10;0 表示无限制。&#10;过大可能会导致搜索时间过长。">
            <!--    <label class="form-check-label mt-0 pt-0 ht-45" for="max-count" style="width: fit-content;">最大匹配次数：</label>-->
                <span class="input-group-text dark-mode border-0 mt-0 ht-30 d-flex fs-6 " style="background:#333;padding-right:6px;padding-top:4px;">最大匹配次数</span>
                <input class="form-control dark-mode mt-0 ht-30" id="max-count" v-model="maxCount"
                    style="width: 80px;" />
            </div>

            <div class=" input-group mt-0  " style="width: fit-content;display: inline-flex;padding-left:0px;padding-right: 0px;"
                title="目录遍历深度&#10;&#10;1-表示当前文件夹&#10;2-表示当前文件夹及子文件夹&#10;3-表示当前文件夹及子文件夹及子文件夹...&#10;&#10;深度越大，耗时越长">
                <span class="input-group-text dark-mode border-0 mt-0 ht-30 d-flex fs-6 " style="background:#333;padding-right:6px;padding-top:4px;">目录遍历深度</span>
                <input class="form-control dark-mode mt-0 ht-30" id="max-depth" v-model="maxDepth"
                    style="width: 80px;" />
            </div>

            <div style="display: inline-flex;position: absolute;bottom: 0;gap: 0.5rem;padding-left:0px;padding-right: 0px; width:calc(100% - 280px);">
                <label for="">执行状态：</label><label :class="['text-success', isDone ? '':'blink' ]" for=""><b>{{ cmdStatus }}</b></label>
                &ensp;&ensp;<label>搜索结果：{{ output.length }} 条</label>
            </div>
            <div style="display: flex;justify-content:flex-end;align-items:center; gap: 0.5rem;padding-right: 15px;" class="parent-container">
                <div class="form-check mt-0" style="width: fit-content;" title="显示搜索结果文件创建时间">
                    <label class="form-check-label mt-0 pt-0 ht-45" for="display-created-at">显示创建时间</label>
                    <input type="checkbox" class="form-check-input mt-2" id="display-created-at" v-model="displayCreatedAt" />
                </div>
                <div class="form-check mt-0" style="width: fit-content;" title="显示搜索结果文件修改时间">  
                    <label class="form-check-label mt-0 pt-0 ht-45" for="display-modified-at">显示修改时间</label>
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
                    <th class="text-start fs-6 " style="width: 20px;">目录</th>
                    <th class="text-end fs-6 " style="width: 20px;" title="点击按命中次数排序">
                        <a href="#" @click.prevent="sortOutputByHitCount" id="sort-by-hit-count"> 命中- </a>
                    </th>

                    <th class="text-center fs-6 " v-if="displayModifiedAt" style="width: 40px;" title="点击按修改时间排序"><a href="#" @click.prevent="sortOutputByModified" id="sort-by-modified">修改时间-</a></th>
                    <th class="text-center fs-6 " v-if="displayCreatedAt" style="width: 40px;" title="点击按创建时间排序"><a href="#" @click.prevent="sortOutputByCreated" id="sort-by-created">创建时间-</a></th>
                    <th class="text-start fs-6 " title="点击按文件名排序">
                        <a href="#" @click.prevent="sortOutputByFile" id="sort-by-file">&emsp;文件- </a>
                    </th>

                </tr>
            </thead>
            <tbody>
                <tr v-for="line in output" :key="line.file">
                    <td class="text-end fs-6 " title="转到文件所在目录"><a class="no-underline " href="#"
                            @click.prevent="gotoFolder(line.file)"><img src="/src/assets/folder.svg" class="icon"
                                alt="Icon"></a></td>
                    <td class="text-end fs-6 " title="关键字在该文件中的出现次数/命中次数&#10;&#10;受限于【最大匹配次数】设置">{{ line.hitCount }}</td>
                    <td class="text-center fs-6 " v-if="displayModifiedAt" title="">{{ line.modified_at}}</td>
                    <td class="text-center fs-6 " v-if="displayCreatedAt" title="">{{ line.created_at}}</td>
                    <td class="text-start fs-6 "><a class="no-underline " href="#" @click.prevent="openFile(line.file)"
                            :title="line.content"> {{ line.file }} </a></td>

                </tr>
            </tbody>
        </table>
            </div>
    <div class="container-fluid min-vh-100 d-flex flex-column ">
        <div class="row justify-content-end">
            <div class="col-auto">
            <span>Version:1.1.0 &emsp; Developed by Viaco.&emsp; Email : viaco.xu@qq.com</span>
        </div>
        </div>
    </div>
    <div id="alertBox" class="custom-alert" >
        <span style=" font-weight: bold;" class="fs-5 text-warning ">【文件类别】未指定</span><br><br>
      将搜索所有类别的文件，速度较慢！ 请耐心等待...<br><br>
      <span class="red-text">如需终止漫长搜索，请关闭本程序后重开。</span>
      
      <br><br>
      <button @click="closeCustomAlert" id="closeAlertBtn" class="btn btn-primary">确定</button>
    
    </div>
    <div id="alertRuningBox" class="custom-alert" >
      <span style="font-weight: bold;" class="fs-5 text-warning ">{{ isDone ? '搜索完成！':'当前搜索未结束！'}}</span><br>
      <span style="font-weight: bold;" class="fs-5 text-warning">{{ isDone ? '':'请耐心等待...'}}</span><br><br>
      {{ isDone ? '请点击确定':'如欲强行终止，可关闭本程序后重新打开。'}}
      <br><br>
      <button @click="closeRuningAlert" id="closeRuningAlertBtn" class="btn btn-primary">确定</button>
    
    </div>

</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { tr } from 'vuetify/locale';


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
    sf.innerText = "文件-";
    hc.innerText = "命中-";
    mt.innerText = "修改时间-";
    ct.innerText = "创建时间-";
}
function isPatternNotOK(pattern) {
  // 非汉字字符过短检测，短于3字符的返回true。汉字字符不做检测。
  let re=/^[ ]*[\x00-\xFF]{1,3}([ ]|$)/;
  console.log(pattern);
  console.log(re);
  console.log(re.test(pattern));
  return re.test(pattern);

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
        const excludeNotCommon=ref(true); // 是否排除不常用且耗时的图片、压缩文件、数据库、视频文件
        ////begin 新加
        const isDone=ref(true); //搜索是否执行结束
        const displayCreatedAt=ref(false); //是否显示创建时间
        const displayModifiedAt=ref(true); //是否显示修改时间
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
        };
        const closeRuningAlert=()=>{
          getById("alertRuningBox").style.display="none";
        };

         
        const sortOutputByHitCount = () => {
            let hc = getById("sort-by-hit-count");
            //let sf = getById("sort-by-file");

            if (hc.innerText === "命中-") {
                //↑↓
                //descending
                resetTableHeader();
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
            //sf.innerText = "文件-";
            scrollToTop();
        };
        const sortOutputByFile = () => {
            let sf = getById("sort-by-file");
            //let hc = getById("sort-by-hit-count");

            if (sf.innerText === "文件-") {
                //ascending
                resetTableHeader();
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

            //hc.innerText = "命中-";
            scrollToTop();
        };
        const sortOutputByCreated = () => {
            let sc = getById("sort-by-created");
            if (sc.innerText === "创建时间-") {
                //ascending
                resetTableHeader();
                sc.innerText = "创建时间↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            } else if (sc.innerText === "创建时间↑") {
                //descending
                sc.innerText = "创建时间↓";
                output.value.sort((a, b) => b.created_at.localeCompare(a.created_at));
            } else {
                //ascending
                sc.innerText = "创建时间↑";
                output.value.sort((a, b) => a.created_at.localeCompare(b.created_at));
            }
            scrollToTop();
        };
        const sortOutputByModified = () => {
            let sm = getById("sort-by-modified");
            if (sm.innerText === "修改时间-") {
                //ascending
                resetTableHeader();
                sm.innerText = "修改时间↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            } else if (sm.innerText === "修改时间↑") {
                //descending
                sm.innerText = "修改时间↓";
                output.value.sort((a, b) => b.modified_at.localeCompare(a.modified_at));
            } else {
                //ascending
                sm.innerText = "修改时间↑";
                output.value.sort((a, b) => a.modified_at.localeCompare(b.modified_at));
            }

            scrollToTop();
        };
        const runCommand = () => {

            if (!isDone.value) {
              //alert("上一搜索未结束...\n请耐心等待。\n\n如欲强制终止，请关闭本程序后重新打开。");
              console.log("runCommand:isDone:",isDone.value);
              showRuningAlert();

              return;
            }
            if(isPatternNotOK(searchPattern.value) && !regexMode.value){
              alert("搜索关键字过短！\n英文字符每关键字至少4个字符，或用Regex模式，汉字不限。");
              return;
            } 
            if (searchPattern.value === '' && !searchFilename.value) {    //if search pattern is empty, show error message and return
                alert('搜索模式不能为空');
                return;
            }

            if (searchPath.value === '') {    //if search path is empty, show error message and return  
                alert('搜索路径不能为空');
                return;
            }
            //处理路径中的空格，进行转义。
            // searchPath.value = searchPath.value.replace(/ /g, '\\ ');

            if (searchFilename.value && filenamePattern.value.trim() === '' && searchPattern.value.trim() === '') {
                alert('搜索文件名时，搜索模式和文件名特征不能同时为空');
                return;
            }
            if (isNaN(maxCount.value) || maxCount.value < 0) {
                alert('最大匹配次数必须为0，或正整数');
                return;
            }
            if (isNaN(maxDepth.value) || maxDepth.value < 0) {
                alert('目录遍历深度必须正整数');
                return;
            }
          
            if(filenamePattern.value.trim() === '') {
                //alert('【文件类别】未指定，将搜索所有类别的文件，速度较慢！\n\n如需终止搜索，请关闭本程序。\n\n请耐心等待！');
                showCustomAlert();
            }
         
            isDone.value=false;


            let t = searchPattern.value.trim().split(' ');//split search pattern into two keywords
            let ptrn = t.filter(item => item.trim() !== '');//remove empty string

            //if (ptrn.length > 1 && regexMode.value === false) {
            //    alert('搜索关键词大于1个，将启用管道模式，可能比较耗时!\n\n请耐心等待......');
            //}
            output.value = [];//clear output before running new command
            // reset
            getById("sort-by-file").innerText = "文件-";
            getById("sort-by-hit-count").innerText = "命中-";
            preFile.value = "";//reset preFile before running new command
            if (ptrn.length > 1 && regexMode.value === false) {
                cmdStatus.value = "管道模式中(较耗时)...";
            }else{
                cmdStatus.value = "搜索中...";
            }
            invoke('run_rg_command', { searchPattern: searchPattern.value, searchPath: searchPath.value, filenamePattern: filenamePattern.value, regexMode: regexMode.value, dispHitCount: dispHitCount.value, searchFilename: searchFilename.value, maxCount: Number(maxCount.value) , searchHidden: searchHidden.value , maxDepth: Number(maxDepth.value) , searchBinary: searchBinary.value ,excludeNotCommon: excludeNotCommon.value });
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
            //  event.payload格式：hit_count~file~created_at~modified_at~content
                let record=event.payload.split('~');
                if (searchFilename.value) {
                    output.value.push({hitCount:0, file: record[1],created_at:record[2], modified_at:record[3], content: '' });
                    return;
                }
                // 如果file字段为空
                if (record[1] === '') {
                    return;
                }
                output.value.push({ hitCount: record[0], file: record[1],created_at:record[2], modified_at:record[3], content: record[4] });
            });

            // listen('rg-output-bak', event => {

            //     if (searchFilename.value) {

            //         output.value.push({ file: event.payload, content: '' });
            //         return;
            //     }
            //     // 找到第一个冒号的位置

            //     let firstIndex = event.payload.indexOf(':');
            //     // 如果是windows系统，冒号可能出现两次，所以需要找到第二个冒号的位置
            //     if (OS.value === 'windows') {
            //         firstIndex = event.payload.indexOf(':', firstIndex + 1);
            //     }

            //     // 如果找不到冒号，返回原始字符串和空字符串
            //     if (firstIndex === -1) {
            //         output.value.push({ file: event.payload, content: '' });
            //         return;
            //     }
            //     // 使用slice方法分割字符串
            //     let firstPart = event.payload.slice(0, firstIndex);
            //     let secondPart = event.payload.slice(firstIndex + 1);
            //     if (secondPart.length > 1000) {
            //         secondPart = secondPart.slice(0, 1000) + '...';
            //     }
            //     // 非显示命中次数模式，文件名去重。把命中字符串content和前次命中字符串合并

            //     if (firstPart === preFile.value) {
            //         // 去重
            //         output.value[output.value.length - 1].hitCount += 1;
            //         // 命中次数大于20时，显示前20个命中内容,其他忽略
            //         if (output.value[output.value.length - 1].hitCount < 20) {
            //             output.value[output.value.length - 1].content += "\n" + secondPart;
            //         }
            //         if (output.value[output.value.length - 1].hitCount === 20) {
            //             output.value[output.value.length - 1].content += "\n...";
            //         }
            //         return;
            //     }
            //     // 更改preFile为当前文件名
            //     preFile.value = firstPart;
            //     output.value.push({ hitCount: 1, file: firstPart, content: secondPart });
            // });
            listen('completed', event => {
                let t = searchPattern.value.trim().split(' ');//split search pattern into two keywords
                let ptrn = t.filter(item => item.trim() !== '');//remove empty string

                if (ptrn.length > 1 && regexMode.value === false) {                    
                    cmdStatus.value +="->("+output.value.length + ")->完成" ;
                }else{
                    cmdStatus.value = event.payload;
                }
                isDone.value=true;
                console.log(event.payload);
            });

            listen('get-os', event => {
                OS.value = event.payload;
                console.log(event.payload);
                //alert(OS);
            });
            listen('progress', event => {
                cmdStatus.value = event.payload;
                console.log(event.payload);
                //alert(OS);
            });
            listen('overdate', event => {
                alert(event.payload);
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
        //begin 新加
        onBeforeUnmount(() => {
            document.removeEventListener('click', handleDropdownClick);
        });
        //end 新加

        return {
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
            excludeNotCommon,
            closeCustomAlert,
            closeRuningAlert,
            //end 新加
            regexMode,
            isDone,
            displayCreatedAt,
            displayModifiedAt,
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
</style>
