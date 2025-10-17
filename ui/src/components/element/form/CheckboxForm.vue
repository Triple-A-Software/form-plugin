<script setup lang="ts">
import { createCheckbox, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"checkbox">>({
    required: true,
    default: createCheckbox(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UCheckbox v-model="model.default_value" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
