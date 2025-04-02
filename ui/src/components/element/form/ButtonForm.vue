<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "button" }>>({
    required: true,
    default: {
        type: "button",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
    },
});

const { t } = useI18n();
const buttonTypeOptions = computed(() => [
    {
        label: t("field.option.type.button"),
        value: "button",
    },
    {
        label: t("field.option.type.submit"),
        value: "submit",
    },
    {
        label: t("field.option.type.reset"),
        value: "reset",
    },
]);
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.type')">
        <USelect v-model="model.type" :items="buttonTypeOptions" />
    </UFormField>
</template>
