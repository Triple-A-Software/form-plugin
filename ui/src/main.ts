import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import ui from "@nuxt/ui/vue-plugin";
import { createRouter, createWebHistory } from "vue-router";
import { createI18n } from "vue-i18n";
import en from "./locales/en";
import de from "./locales/de";

const router = createRouter({
	routes: [],
	history: createWebHistory(),
});

const i18n = createI18n({
	legacy: false,
	availableLocales: ["en", "de"],
	fallbackLocale: "en",
	messages: {
		en,
		de,
	},
});

createApp(App).use(router).use(ui).use(i18n).mount("#app");
