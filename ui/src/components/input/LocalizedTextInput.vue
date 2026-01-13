<script setup lang="ts">
import { useCmsApiQuery } from "@/utils/cmsApi";
import { computed, ref } from "vue";

const model = defineModel<Record<string, string>>({ required: true, default: {} });

const cmsSettings = useCmsApiQuery("/api/rest/settings", {
    method: "GET",
    query: {
        // we just use the default language, because for the available
        // languages, this doesn't matter
        language: "en",
    },
});
const availableLanguages = computed(() => cmsSettings.data.value?.languages ?? []);
const defaultLanguage = computed(() => cmsSettings.data.value?.defaultLanguage ?? "en");

const selectedLang = ref(defaultLanguage.value);
</script>

<template>
    <div class="flex flex-row gap-4">
        <UFieldGroup class="w-full">
            <UInput v-model="model[selectedLang]" class="w-full" />
            <USelect :items="availableLanguages" value-key="value" label-key="label" v-model="selectedLang" />
        </UFieldGroup>
    </div>
</template>
