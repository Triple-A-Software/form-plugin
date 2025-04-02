<script setup lang="ts">
import { computed } from "vue";
import type { NavigationMenuItem } from "@nuxt/ui";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const links = computed<NavigationMenuItem[]>(() => [
    {
        label: t("pages.home"),
        to: "/",
        icon: "i-tabler-home",
    },
    {
        label: t("pages.forms"),
        to: "/forms",
        icon: "i-tabler-forms",
    },
    {
        label: t("pages.submissions"),
        to: "/submissions",
        icon: "i-tabler-send",
    },
]);
</script>
<template>
    <UContainer>
        <UNavigationMenu :items="links" />
        <RouterView />
    </UContainer>
</template>
