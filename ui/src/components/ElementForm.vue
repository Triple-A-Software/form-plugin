<script setup lang="ts">
import type { Property } from "../../../bindings";
import CheckboxForm from "./element/form/CheckboxForm.vue";

const model = defineModel<Property>({ required: true });
</script>

<template>
    <TextForm v-if="model.type === 'text'" v-model="model" />
    <TextareaForm v-else-if="model.type === 'textarea'" v-model="model" />
    <ButtonForm v-else-if="model.type === 'button'" v-model="model" />
    <ButtongroupForm v-else-if="model.type === 'buttongroup'" v-model="model" />
    <CheckboxForm v-else-if="model.type === 'checkbox'" v-model="model" />
    <NumberForm v-else-if="model.type === 'number'" v-model="model" />
    <RangeForm v-else-if="model.type === 'range'" v-model="model" />
    <SelectForm v-else-if="model.type === 'select'" v-model="model" />
</template>
