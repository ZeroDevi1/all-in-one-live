import type {RouteRecordRaw} from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Huya",
        component: () => import("../../views/Huya.vue"),
    }
]

export default routes;