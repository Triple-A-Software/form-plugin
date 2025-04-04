<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

const i18n = useI18n();

const selectedLang = ref(i18n.locale.value);

const model = defineModel<Record<string, string>>({ required: true, default: {} });
</script>
<template>
    <div class="flex flex-row gap-4">
        <UButtonGroup class="w-full">
            <UInput v-model="model[selectedLang]" class="w-full" />
            <USelect :items="i18n.availableLocales" v-model="selectedLang" />
        </UButtonGroup>
    </div>
</template>
