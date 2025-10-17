<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { createSelect, type PropertySelector } from "@/utils";
import type { SelectData } from "../../../../../bindings";

const model = defineModel<PropertySelector<"select">>({
    required: true,
    default: createSelect(),
});

const { t, locale } = useI18n();
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

const options = computed(() => model.value.options.map((option) => ({ ...option, label: option.label[locale.value] })));
</script>

<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <USelect v-model="model.default_value as SelectData | undefined" :items="options" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.multiple')">
        <UCheckbox v-model="model.multiple" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.data_type')">
        <USelect v-model="model.data_type" :items="data_type_options" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.options')">
        <ArrayInput v-model="model.options" :empty="empty" class="w-full">
            <template #edit-item="{ item }">
                <UFormField :label="$t('field.label.label')" required>
                    <LocalizedTextInput v-model="item.label" />
                </UFormField>
                <UFormField :label="$t('field.label.value')" required>
                    <UInput v-if="typeof item.value === 'string'" v-model="item.value" />
                    <UInputNumber v-else-if="typeof item.value === 'number'" v-model="item.value" />
                </UFormField>
            </template>
        </ArrayInput>
    </UFormField>
</template>
