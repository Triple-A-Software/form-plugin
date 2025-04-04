<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { createButtongroup, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"buttongroup">>({
    required: true,
    default: createButtongroup(),
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
            class="w-full"
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
