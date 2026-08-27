import { createI18n } from 'vue-i18n'
import zh from './locales/zh'
import en from './locales/en'

// 預設語言先讀取瀏覽器快取，若無則預設中文
const savedLanguage = localStorage.getItem('lang') || 'zh'

const i18n = createI18n({
    legacy: false,          // 使用 Composition API 模式必須設為 false
    globalInjection: true, // 全域註冊 $t 方法
    locale: savedLanguage,  // 當前語系
    fallbackLocale: 'en',   // 若找不到對應翻譯時的後備語系
    messages: {
        zh: zh as Record<string, any>,
        en: en as Record<string, any>
    }    // 載入語言包
})

export default i18n