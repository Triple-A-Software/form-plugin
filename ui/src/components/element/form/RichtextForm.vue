<script setup lang="ts">
import type { Property } from "../../../../../bindings";

const model = defineModel<Extract<Property, { type: "richtext" }>>({
    required: true,
    default: {
        type: "richtext",
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
