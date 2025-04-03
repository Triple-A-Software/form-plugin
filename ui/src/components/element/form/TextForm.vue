<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../../bindings";
import type { NullToUndefined } from "@/utils/types/utils";

const model = defineModel<Extract<NullToUndefined<Property>, { type: "text" }>>({
    required: true,
    default: {
        type: "text",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
    },
});

const { t } = useI18n();
const textTypeOptions = computed(() => [
    {
        label: t("field.option.type.text"),
        value: "text",
    },
    {
        label: t("field.option.type.password"),
        value: "password",
    },
    {
        label: t("field.option.type.email"),
        value: "email",
    },
    {
        label: t("field.option.type.color"),
        value: "color",
    },
    {
        label: t("field.option.type.tel"),
        value: "tel",
    },
]);
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.type')">
        <USelect v-model="model.type" :items="textTypeOptions" />
    </UFormField>
    <UFormField :label="$t('field.label.max')">
        <UInputNumber :min="0" v-model="model.max" />
    </UFormField>
    <UFormField :label="$t('field.label.placeholder')">
        <UInput v-model="model.placeholder" />
    </UFormField>
</template>
