<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { createSelect, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"select">>({
    required: true,
    default: createSelect(),
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
        <UCheckbox v-model="model.multiple" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.data_type')">
        <USelect v-model="model.data_type" :items="data_type_options" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.options')">
        <ArrayInput v-model="model.options" :empty="empty" class="w-full">
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
