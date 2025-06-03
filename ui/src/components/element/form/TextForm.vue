<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { createText, type PropertySelector } from "@/utils";
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import type { TextInputType } from "../../../../../bindings";

const model = defineModel<PropertySelector<"text">>({
    required: true,
    default: createText(),
});

const { t } = useI18n();
const textTypeOptions = computed(
    () =>
        [
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
        ] satisfies { label: string; value: TextInputType }[],
);
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UInput v-model="model.default_value" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.type')">
        <USelect v-model="model.input_type" :items="textTypeOptions" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.max')">
        <UInputNumber :min="0" v-model="model.max" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.placeholder')">
        <LocalizedTextInput v-model="model.placeholder" class="w-full" />
    </UFormField>
</template>
