<script setup lang="ts">
import { useCmsApiQuery } from "@/utils/cmsApi";
import { computed, ref, watch } from "vue";

const model = defineModel<Record<string, string>>({ required: true, default: {} });

const cmsSettings = useCmsApiQuery("/api/rest/settings", {
    method: "GET",
    query: {
        // we just use the default language, because for the available
        // languages, this doesn't matter
        language: "en",
    },
});
const defaultLanguage = computed(() => cmsSettings.data.value?.defaultLanguage ?? "en");
// mark languages without a translation, so missing ones are visible without clicking through
const availableLanguages = computed(() =>
    (cmsSettings.data.value?.languages ?? []).map((lang) => ({
        ...lang,
        icon: model.value[lang.value] ? "i-tabler-check" : "i-tabler-circle-dashed",
    })),
);
const missingCount = computed(() => availableLanguages.value.filter((lang) => !model.value[lang.value]).length);

const selectedLang = ref(defaultLanguage.value);
// the settings usually load after mount, so switch to the real default language once they arrive
watch(defaultLanguage, (lang, previous) => {
    if (selectedLang.value === previous) selectedLang.value = lang;
});
</script>

<template>
    <div class="flex flex-row gap-4">
        <UFieldGroup class="w-full">
            <UInput v-model="model[selectedLang]" class="w-full" />
            <USelect :items="availableLanguages" value-key="value" label-key="label" v-model="selectedLang">
                <template #leading>
                    <UChip v-if="missingCount > 0 && availableLanguages.length > 1" color="warning" size="sm" inset standalone />
                </template>
            </USelect>
        </UFieldGroup>
    </div>
</template>
