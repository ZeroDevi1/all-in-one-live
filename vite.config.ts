import {defineConfig} from "vite";
// @ts-ignore
import {internalIpV4} from 'internal-ip'
import vue from "@vitejs/plugin-vue";
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import {ElementPlusResolver} from 'unplugin-vue-components/resolvers'
// https://vitejs.dev/config/
// @ts-ignore
export default defineConfig(async () => {
    const host = await internalIpV4();
    const config = {
        plugins: [vue(), AutoImport({
            resolvers: [ElementPlusResolver()],
        }),
            Components({
                resolvers: [ElementPlusResolver()],
            }),],

        // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
        // prevent vite from obscuring rust errors
        clearScreen: false,
        // tauri expects a fixed port, fail if that port is not available
        // server: {
        //     port: 1420,
        //     strictPort: true,
        // },
        server: {
            host: '0.0.0.0', // listen on all addresses
            port: 5173,
            strictPort: true,
            hmr: {
                protocol: 'ws',
                host,
                port: 5183,
            },
        },
        // to make use of `TAURI_DEBUG` and other env variables
        // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
        envPrefix: ["VITE_", "TAURI_"],
        build: {
            // Tauri supports es2021
            target: ["es2021", "chrome100", "safari13"],
            // don't minify for debug builds
            minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
            // produce sourcemaps for debug builds
            sourcemap: !!process.env.TAURI_DEBUG,
        },
    };
    return config;
});
