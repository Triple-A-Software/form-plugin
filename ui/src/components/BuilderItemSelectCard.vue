<template>
    <button
        type="button"
        class="flex flex-col items-center gap-2 rounded-lg p-3 ring-1 ring-default bg-default hover:ring-2 hover:ring-primary focus-visible:outline-2 focus-visible:outline-primary transition-all duration-150 select-none cursor-pointer"
        @click="emit('click', element)"
    >
        <UIcon :name="element.icon ?? fallbackIcon ?? ''" class="size-8" />
        <span class="text-sm text-center">{{ element.label }}</span>
    </button>
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
