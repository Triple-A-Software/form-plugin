<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../bindings";
import type { NullToUndefined } from "@/utils/types/utils";

defineProps<{
    element: NullToUndefined<Property>;
}>();

defineEmits<{
    (e: "edit"): void;
    (e: "delete"): void;
}>();
const { locale } = useI18n();
</script>
<template>
    <div class="flex gap-2 w-full items-center">
        <UIcon name="i-tabler-grip-vertical" class="cursor-grab" />
        <label>
            <span class="font-bold">
                {{ $t(`builder.item.label.${element.type}`) }}
            </span>
            <span class="ms-4">
                {{ element.label[locale] }}
            </span>
            <span class="text-red-600" v-if="element.required"> * </span>
        </label>
        <USeparator orientation="vertical" class="h-4" v-if="element.name || element.hint?.[locale]" />
        <p class="italic text-gray-600 dark:text-gray-400 text-sm" v-if="element.name">{{ element.name }}</p>
        <USeparator orientation="vertical" class="h-4" v-if="element.name && element.hint?.[locale]" />
        <p class="italic text-gray-600 dark:text-gray-400 text-sm" v-if="element.hint?.[locale]">{{ element.hint[locale] }}</p>
        <div class="flex-1"></div>
        <UButton color="neutral" variant="ghost" icon="i-tabler-edit" @click="$emit('edit')" />
        <UButton color="error" variant="ghost" icon="i-tabler-trash" @click="$emit('delete')" />
    </div>
</template>
