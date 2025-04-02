<script setup lang="ts">
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "number" }>>({
    required: true,
    default: {
        type: "number",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
    },
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
</template>
