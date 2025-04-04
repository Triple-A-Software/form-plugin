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
</template>
