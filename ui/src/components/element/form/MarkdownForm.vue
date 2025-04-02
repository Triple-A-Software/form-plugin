<script setup lang="ts">
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "markdown" }>>({
    required: true,
    default: {
        type: "markdown",
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
