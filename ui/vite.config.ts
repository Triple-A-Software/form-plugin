import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import ui from "@nuxt/ui/vite";
import { URL } from "url";
import Layouts from "vite-plugin-vue-layouts";
import VueRouter from "unplugin-vue-router/vite";

// https://vite.dev/config/
export default defineConfig({
	base: "/api/rest/plugins/form-plugin/ui",
	define: {
		__API_BASE__: JSON.stringify("/api/rest/plugins/form-plugin/api"),
	},
	plugins: [
		VueRouter(),
		vue(),
		ui({
			ui: {
				colors: {
					primary: "orange",
					neutral: "zinc",
				},
				formField: {
					slots: {
						root: "w-full",
					},
				},
			},
		}),
		Layouts(),
	],
	resolve: {
		alias: {
			"@": fileURLToPath(new URL("./src", import.meta.url)),
		},
	},
	server: {
		proxy: {
			"/api": "http://localhost:3000",
		},
	},
});
