<script setup lang="ts">
import { createFile, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"file">>({
    required: true,
    default: createFile(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UInput v-model="model.default_value as string | undefined" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.multipleFiles')">
        <UCheckbox v-model="model.multiple" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.accept')" :help="$t('hint.accept')">
        <UInput v-model="model.accept" placeholder=".pdf,.zip,image/*" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.maxFileSizeMb')" :help="$t('hint.maxFileSizeMb')">
        <UInputNumber v-model="model.max_file_size_mb" :min="1" placeholder="20" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
