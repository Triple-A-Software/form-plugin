<script setup lang="ts">
import { createDate, type PropertySelector } from "@/utils";
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";

const model = defineModel<PropertySelector<"date">>({
    required: true,
    default: createDate(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UInput type="date" v-model="model.default_value as string | undefined" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.placeholder')">
        <LocalizedTextInput v-model="model.placeholder" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
