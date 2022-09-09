import {createApp} from "vue";
import "./style.css";
import App from "./App.vue";
// 导入 Pinia
import {createPinia} from 'pinia'
// 导入 Pinia 插件
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
// 导入 Element Plus
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
// 导入 router
import router from "./router";
const pinia = createPinia() // 初始化 Pinia
pinia.use(piniaPluginPersistedstate) // 激活 Pinia 插件
createApp(App)
    .use(pinia) // 启用 Pinia ，这一次是包含了插件的 Pinia 实例
    .use(ElementPlus) // 启用 ElementPlus
    .use(router) // 启用 router
    .mount("#app");
