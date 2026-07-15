// 路由
import { createRouter,createWebHashHistory,RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
    {
        path:'/',
        name:'HomePage',
        component:()=> import('@/views/MainMenu.vue')
    },
    {
        path:'/exhibit',
        name:'exhibit',
        component:()=>import('@/views/ExihibitZone.vue')
    }
]

const router = createRouter({
    history:createWebHashHistory(),
    routes
})

export default router