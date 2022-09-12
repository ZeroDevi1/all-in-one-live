import type {RouteRecordRaw} from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Huya",
        component: () => import("../../views/Huya.vue"),
    },
    {
        path: "/bilibili",
        name: "Bilibili",
        component: () => import("../../views/Bilibili.vue"),
    },
]

export default routes;