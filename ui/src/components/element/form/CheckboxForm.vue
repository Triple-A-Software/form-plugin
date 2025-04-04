<script setup lang="ts">
import { createCheckbox, type PropertySelector } from "@/utils";

const model = defineModel<PropertySelector<"checkbox">>({
    required: true,
    default: createCheckbox(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
</template>
