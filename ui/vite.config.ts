import { resolve } from "node:path";
import { fileURLToPath, URL } from "node:url";
import ui from "@nuxt/ui/vite";
import vue from "@vitejs/plugin-vue";
import VueRouter from "unplugin-vue-router/vite";
import { defineConfig } from "vite";
import Layouts from "vite-plugin-vue-layouts";

// https://vite.dev/config/
export default defineConfig({
	base:
		process.env.NODE_ENV === "production"
			? "/api/rest/plugins/form-plugin/ui"
			: undefined,
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
	build: {
		rollupOptions: {
			input: {
				main: resolve(__dirname, "index.html"),
				dashboard_recent_submissions: resolve(
					__dirname,
					"dashboard/recent-submissions.html",
				),
				dashboard_submission_stats: resolve(
					__dirname,
					"dashboard/submission-stats.html",
				),
			},
		},
	},
});
