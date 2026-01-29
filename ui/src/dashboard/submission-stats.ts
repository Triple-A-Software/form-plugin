// this file is the entry point for the dashboard submission-stats card
import { createApp } from "vue";
import "../style.css";
import ui from "@nuxt/ui/vue-plugin";
import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";
import { createPinia } from "pinia";
import { createI18n } from "vue-i18n";
import { createRouter, createWebHashHistory } from "vue-router";
import de from "../locales/de";
import en from "../locales/en";
import App from "./submission-stats/App.vue";
import SubmissionStatsPage from "./submission-stats/page.vue";

dayjs.extend(utc);

const router = createRouter({
    routes: [
        {
            path: "/",
            component: SubmissionStatsPage,
        },
    ],
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

createApp(App).use(router).use(ui).use(pinia).use(i18n).mount("#app");
