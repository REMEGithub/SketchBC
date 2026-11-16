// 路由
import { createRouter,createWebHashHistory,RouteRecordRaw } from "vue-router";

const routes: RouteRecordRaw[] = [
    {
        path:'/',
        name:'HomePage',
        component:()=> import('@/views/MainMenuV2.vue')
    },
    {
        path:'/exhibit',
        name:'exhibit',
        component:()=>import('@/views/ExhibitZoneV2.vue')
    }
]

const router = createRouter({
    history:createWebHashHistory(),
    routes
})

export default router