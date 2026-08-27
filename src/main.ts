import { createApp } from "vue";
import App from "./App.vue";
import i18n from "./i18n"; // 引入 i18n 配置

const app = createApp(App);

// 使用 i18n 插件
app.use(i18n);
app.mount("#app");
