<script setup lang="ts">
import type { NullToUndefined } from "@/utils/types/utils";
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<NullToUndefined<Property>, { type: "textarea" }>>({
    required: true,
    default: {
        type: "textarea",
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
    <UFormField :label="$t('field.label.placeholder')">
        <UInput v-model="model.placeholder" />
    </UFormField>
    <UFormField :label="$t('field.label.maxLength')">
        <UInputNumber :min="0" v-model="model.max_length" />
    </UFormField>
</template>
