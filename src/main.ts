import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

import * as ElementPlusIconsVue from '@element-plus/icons-vue'
import ElementPlus from "element-plus";
import 'element-plus/dist/index.css'

import router from './router.ts'


import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import ContextMenu from '@imengyu/vue3-context-menu'


let app = createApp(App);
app.use(ElementPlus).use(ContextMenu)
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}
app.use(router).mount('#app')

