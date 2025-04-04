<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import type { PropertyCommon, TailwindColor } from "../../../../../bindings";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

const model = defineModel<PropertyCommon>({ required: true });

const { t } = useI18n();

type ColorOption = { label: string; value: TailwindColor };
const colorOptions = computed<ColorOption[]>(
    () =>
        [
            { label: t("color.red"), value: "red" },
            { label: t("color.orange"), value: "orange" },
            { label: t("color.amber"), value: "amber" },
            { label: t("color.yellow"), value: "yellow" },
            { label: t("color.lime"), value: "lime" },
            { label: t("color.green"), value: "green" },
            { label: t("color.emerald"), value: "emerald" },
            { label: t("color.teal"), value: "teal" },
            { label: t("color.cyan"), value: "cyan" },
            { label: t("color.sky"), value: "sky" },
            { label: t("color.blue"), value: "blue" },
            { label: t("color.indigo"), value: "indigo" },
            { label: t("color.violett"), value: "violet" },
            { label: t("color.purple"), value: "purple" },
            { label: t("color.fuchsia"), value: "fuchsia" },
            { label: t("color.pink"), value: "pink" },
            { label: t("color.rose"), value: "rose" },
            { label: t("color.slate"), value: "slate" },
            { label: t("color.gray"), value: "gray" },
            { label: t("color.zinc"), value: "zinc" },
            { label: t("color.neutral"), value: "neutral" },
            { label: t("color.stone"), value: "stone" },
        ] satisfies ColorOption[],
);
</script>
<template>
    <UFormField :label="$t('field.label.label')">
        <LocalizedTextInput v-model="model.label" />
    </UFormField>
    <UFormField :label="$t('field.label.hint')">
        <LocalizedTextInput v-model="model.hint" />
    </UFormField>
    <UFormField :label="$t('field.label.class')">
        <UInput v-model="model.class" />
    </UFormField>
    <UFormField :label="$t('field.label.name')">
        <UInput v-model="model.name" />
    </UFormField>
    <UFormField :label="$t('field.label.required')">
        <UCheckbox v-model="model.required" />
    </UFormField>
    <UFormField :label="$t('field.label.color')">
        <USelect v-model="model.color" :items="colorOptions" />
    </UFormField>
    <!-- TODO -->
    <!-- <UFormField :label="$t('field.label.default_value')"> -->
    <!--     <UInput v-model="model.default_value" /> -->
    <!-- </UFormField> -->
</template>
