import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import ui from "@nuxt/ui/vue-plugin";
import { createRouter, createWebHashHistory } from "vue-router";
import { createI18n } from "vue-i18n";
import en from "./locales/en";
import de from "./locales/de";
import { handleHotUpdate, routes } from "vue-router/auto-routes";
import { setupLayouts } from "virtual:generated-layouts";
import { createPinia } from "pinia";

const router = createRouter({
	routes: setupLayouts(routes),
	history: createWebHashHistory(),
});

const i18n = createI18n({
	legacy: false,
	availableLocales: ["en", "de"],
	fallbackLocale: "en",
	locale: "en",
	messages: {
		en,
		de,
	},
});

const pinia = createPinia();

if (import.meta.hot) {
	handleHotUpdate(router);
}
createApp(App).use(router).use(ui).use(pinia).use(i18n).mount("#app");
