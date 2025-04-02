<script setup lang="ts">
import type { Property } from "../../../bindings";

const model = defineModel<Property>({ required: true });
</script>

<template>
    <TextForm v-if="model.type === 'text'" v-model="model" />
    <TextareaForm v-else-if="model.type === 'textarea'" v-model="model" />
    <ButtonForm v-else-if="model.type === 'button'" v-model="model" />
</template>
