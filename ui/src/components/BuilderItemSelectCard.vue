<template>
    <UCard
        class="hover:ring-primary-500 hover:dark:ring-primary-400 hover:ring-2 transition-all duration-150 select-none"
        @click="emit('click', element)"
    >
        <UIcon :name="element.icon ?? fallbackIcon ?? ''" class="size-10 w-full"></UIcon>
        <template #footer>
            <p class="text-center w-full">
                {{ element.label }}
            </p>
        </template>
    </UCard>
</template>
<script setup lang="ts">
import type { Property } from "../../../bindings";

export type Item = {
    icon?: string | null | undefined;
    label: string;
    type: Property["type"];
};
defineProps<{
    element: Item;
    fallbackIcon?: string;
}>();
const emit = defineEmits<{
    (e: "click", data: Item): void;
}>();
</script>
