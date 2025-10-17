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
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
