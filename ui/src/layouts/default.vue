<script setup lang="ts">
import { computed } from "vue";
import type { NavigationMenuItem } from "@nuxt/ui";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const links = computed<NavigationMenuItem[]>(() => [
    {
        label: t("pages.home.title"),
        to: "/",
        icon: "i-tabler-home",
    },
    {
        label: t("pages.forms.title"),
        to: "/forms",
        icon: "i-tabler-forms",
    },
    {
        label: t("pages.emails.title"),
        to: "/emails",
        icon: "i-tabler-mail",
    },
    {
        label: t("pages.submissions.title"),
        to: "/submissions",
        icon: "i-tabler-send",
    },
    {
        label: t("pages.settings.title"),
        to: "/settings",
        icon: "i-tabler-settings",
    },
]);
</script>
<template>
    <UContainer>
        <UNavigationMenu :items="links" />
        <RouterView />
    </UContainer>
</template>
