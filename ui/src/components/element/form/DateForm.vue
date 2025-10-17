<script setup lang="ts">
import { createDate, type PropertySelector } from "@/utils";
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { DateInputType } from "../../../../../bindings";

const model = defineModel<PropertySelector<"date">>({
    required: true,
    default: createDate(),
});

const { t } = useI18n();
const dateTypeOptions = computed<{ label: string; value: DateInputType }[]>(() => [
    {
        label: t("field.option.type.date"),
        value: "date",
    },
    {
        label: t("field.option.type.datetimeLocal"),
        value: "datetime-local",
    },
    {
        label: t("field.option.type.time"),
        value: "time",
    },
    {
        label: t("field.option.type.month"),
        value: "month",
    },
    {
        label: t("field.option.type.week"),
        value: "week",
    },
]);
</script>

<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UInput type="date" v-model="model.default_value as string | undefined" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.placeholder')">
        <LocalizedTextInput v-model="model.placeholder" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.type')">
        <USelect v-model="model.input_type" :items="dateTypeOptions" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
