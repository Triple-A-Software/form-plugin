<script setup lang="ts">
import { computed } from "vue";
import type { Property } from "../../../../../bindings";
import { useI18n } from "vue-i18n";
import type { NullToUndefined } from "@/utils/types/utils";

const model = defineModel<Extract<NullToUndefined<Property>, { type: "select" }>>({
    required: true,
    default: {
        type: "select",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
        data_type: "string",
    },
});

const { t } = useI18n();
const data_type_options = computed(() => [
    {
        label: t("field.option.data_type.string"),
        value: "string",
    },
    {
        label: t("field.option.data_type.number"),
        value: "number",
    },
]);

const empty = computed(() => ({
    label: {},
    value: model.value.data_type === "string" ? "" : 0,
}));
</script>

<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.multiple')">
        <UCheckbox v-model="model.multiple" />
    </UFormField>
    <UFormField :label="$t('field.label.data_type')">
        <USelect v-model="model.data_type" :items="data_type_options" />
    </UFormField>
    <UFormField :label="$t('field.label.options')">
        <ArrayInput v-model="model.options" :empty="empty">
            <template #edit-item="{ item }">
                <UFormField :label="$t('field.label.label')">
                    <LocalizedTextInput v-model="item.label" />
                </UFormField>
                <UFormField :label="$t('field.label.value')">
                    <UInput v-if="typeof item.value === 'string'" v-model="item.value" />
                    <UInputNumber v-else-if="typeof item.value === 'number'" v-model="item.value" />
                </UFormField>
            </template>
        </ArrayInput>
    </UFormField>
</template>
