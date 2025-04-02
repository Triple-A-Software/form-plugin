<script setup lang="ts">
import { computed } from "vue";
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "code" }>>({
    required: true,
    default: {
        type: "code",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
    },
});
const languages = computed(() => [
    {
        label: "Javascript",
        value: "js",
    },
    {
        label: "CSS",
        value: "css",
    },
    {
        label: "JSON",
        value: "json",
    },
    {
        label: "HTML",
        value: "html",
    },
]);
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.language')">
        <USelect v-model="model.language" :items="languages" />
    </UFormField>
</template>
