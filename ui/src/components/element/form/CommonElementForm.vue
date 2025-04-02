<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import type { PropertyCommon } from "../../../../../bindings";

const model = defineModel<PropertyCommon>({ required: true });
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
</template>
