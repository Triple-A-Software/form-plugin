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
    <UFormField :label="$t('field.label.default_value')">
        <UInputNumber v-model="model.default_value as number" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.min')" required>
        <UInputNumber v-model="model.min" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.max')" required>
        <UInputNumber v-model="model.max" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.step')">
        <UInputNumber v-model="model.step" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.min_label')">
        <LocalizedTextInput v-model="model.min_label" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.max_label')">
        <LocalizedTextInput v-model="model.max_label" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.min_label_class')">
        <UInput v-model="model.min_label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.max_label_class')">
        <UInput v-model="model.max_label_class" class="w-full" />
    </UFormField>
</template>
