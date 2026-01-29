<script setup lang="ts">
import { createCheckbox, type PropertySelector } from "@/utils";
import { computed } from "vue";

const model = defineModel<PropertySelector<"checkbox">>({
    required: true,
    default: createCheckbox(),
});

const default_value = computed<boolean>({
    get() {
        return Boolean(model.value.default_value);
    },
    set(v) {
        model.value.default_value = v;
    },
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UCheckbox v-model="default_value" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.label_class')">
        <UInput v-model="model.label_class" class="w-full" />
    </UFormField>
    <UFormField :label="$t('field.label.input_class')">
        <UInput v-model="model.input_class" class="w-full" />
    </UFormField>
</template>
