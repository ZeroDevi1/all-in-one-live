import type {RouteRecordRaw} from "vue-router";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "Favorite",
        component: () => import("../../views/Favorite.vue"),
    }
]

export default routes;