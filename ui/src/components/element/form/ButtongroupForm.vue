<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "buttongroup" }>>({
    required: true,
    default: {
        type: "buttongroup",
        required: false,
        label: {},
        hint: {},
        name: "",
        class: "",
        options: [],
    },
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.options')">
        <ArrayInput
            v-model="model.options"
            :empty="{
                label: {},
                value: '',
            }"
        >
            <template #edit-item="{ item }">
                <UFormField :label="$t('field.label.label')">
                    <LocalizedTextInput v-model="item.label" />
                </UFormField>
                <UFormField :label="$t('field.label.value')">
                    <UInput v-model="item.value" />
                </UFormField>
            </template>
        </ArrayInput>
    </UFormField>
</template>
