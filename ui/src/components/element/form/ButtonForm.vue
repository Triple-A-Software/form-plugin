<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { createButton, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"button">>({
    required: true,
    default: createButton(),
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
const variantOptions = computed(() => [
    {
        label: t("field.option.variant.solid"),
        value: "solid",
    },
    {
        label: t("field.option.variant.alternative"),
        value: "alternative",
    },
]);
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.type')">
        <USelect v-model="model.button_type" :items="buttonTypeOptions" />
    </UFormField>
    <UFormField :label="$t('field.label.variant')">
        <USelect v-model="model.variant" :items="variantOptions" />
    </UFormField>
</template>
