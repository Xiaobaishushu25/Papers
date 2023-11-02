// history模式
import {
    createRouter,
    createWebHashHistory,
} from 'vue-router'

import Menu from './components/Menu.vue'
import Paper from './components/PaperList.vue'
import Markdown1 from './components/EditorMarkdown1.vue'
import Markdown2 from './components/EditorMarkdown2.vue'
import Edit from './components/Edit.vue'

const routes = [
// 路由的默认路径
    {
        path:'/',
        redirect:"/paperList"
    },
    {
        path: '/menu',
        component: Menu,
    },
    {
        path:'/paperList',
        component: Paper,
    },
    {
        path:'/markdown1',
        component: Markdown1,
    },
    {
        path:'/markdown2',
        component: Markdown2,
    },
    {
        path:'/edit',
        component: Edit,
    }
]

// 创建路由对象
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
export default router;