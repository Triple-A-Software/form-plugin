<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { type PropertySelector, createTextarea } from "@/utils";

const model = defineModel<PropertySelector<"textarea">>({
    required: true,
    default: createTextarea(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.placeholder')">
        <LocalizedTextInput v-model="model.placeholder" />
    </UFormField>
    <UFormField :label="$t('field.label.maxLength')">
        <UInputNumber :min="0" v-model="model.max_length" />
    </UFormField>
    <UFormField :label="$t('field.label.rows')">
        <UInputNumber :min="0" v-model="model.rows" />
    </UFormField>
    <UFormField :label="$t('field.label.columns')">
        <UInputNumber :min="0" v-model="model.columns" />
    </UFormField>
</template>
