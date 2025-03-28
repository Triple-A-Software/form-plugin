<template>
    <UFormGroup :name="name">
        <template #label>
            <FormGroupLabel :label="label" :hint="hint" />
        </template>
        <InputBuilderTree :builderItems="builderItems" v-model="model" />
    </UFormGroup>
</template>

<script setup lang="ts">
import type { Item } from "@/components/FormBuilderTree.vue";

const props = defineProps<{
    builderItems: Array<Item>;
    label?: string;
    name: string;
    hint?: string;
}>();

const model = defineModel<Array<Item>>({
    default: [],
});
</script>
