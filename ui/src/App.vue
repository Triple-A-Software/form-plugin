<script setup lang="ts">
import { useUrlSearchParams } from "@vueuse/core";
import { onMounted, watch } from "vue";
import { useI18n } from "vue-i18n";
import { en, de } from "@nuxt/ui/locale";

const searchParams = useUrlSearchParams();
const { locale } = useI18n();
function updateLocaleFromSearchParams() {
    const lang = searchParams.lang;
    if (lang && typeof lang === "string") {
        locale.value = lang;
    }
}
onMounted(() => {
    updateLocaleFromSearchParams();
});
watch(
    () => searchParams.lang,
    () => {
        updateLocaleFromSearchParams();
    },
);
</script>

<template>
    <UApp :locale="locale === 'en' ? en : de">
        <RouterView />
    </UApp>
</template>
