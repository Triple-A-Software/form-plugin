<script setup lang="ts">
import { useUrlSearchParams } from "@vueuse/core";
import { onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

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

const builder = ref([]);
</script>

<template>
    <UApp>
        <RouterView />
        <UFormField :label="$t('field.label.formbuilder')">
            <FormBuilder v-model="builder" />
        </UFormField>
    </UApp>
</template>
