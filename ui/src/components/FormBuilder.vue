<template>
    <UFormField :name="name">
        <template #label>
            {{ label }}
            <span v-if="required" class="ms-0.5 text-red-500 dark:text-red-400"> * </span>
            <InfoPopover :text="hint" v-if="hint" />
        </template>
        <FormBuilderTree :builderItems="builderItems" v-model="model" />
    </UFormField>
</template>

<script setup lang="ts">
import type { BuilderItem } from "../../../bindings";

defineProps<{
    builderItems: Array<BuilderItem>;
    label?: string;
    name: string;
    hint?: string;
    required?: boolean;
}>();

const model = defineModel<Array<BuilderItem>>({
    default: [],
});
</script>
