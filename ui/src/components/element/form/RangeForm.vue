<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { createRange, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"range">>({
    required: true,
    default: createRange(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.min')">
        <UInputNumber v-model="model.min" />
    </UFormField>
    <UFormField :label="$t('field.label.max')">
        <UInputNumber v-model="model.max" />
    </UFormField>
    <UFormField :label="$t('field.label.step')">
        <UInputNumber v-model="model.step" />
    </UFormField>
    <UFormField :label="$t('field.label.min_label')">
        <LocalizedTextInput v-model="model.min_label" />
    </UFormField>
    <UFormField :label="$t('field.label.max_label')">
        <LocalizedTextInput v-model="model.max_label" />
    </UFormField>
</template>
