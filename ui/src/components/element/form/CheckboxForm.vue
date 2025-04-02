<script setup lang="ts">
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "checkbox" }>>({
    required: true,
    default: {
        type: "checkbox",
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
</template>
