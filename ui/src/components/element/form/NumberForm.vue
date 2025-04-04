<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { createNumber, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"number">>({
    required: true,
    default: createNumber(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.min')">
        <UInputNumber v-model="model.min" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.max')">
        <UInputNumber v-model="model.max" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.step')">
        <UInputNumber v-model="model.step" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.placeholder')">
        <LocalizedTextInput v-model="model.placeholder" class="w-full" />
    </UFormField>
</template>
