<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../bindings";
import { computed } from "vue";

const props = defineProps<Property>();

defineEmits<{
    (e: "edit"): void;
    (e: "delete"): void;
}>();
const { locale } = useI18n();

const extra = computed(() => {
    switch (props.type) {
        case "number":
            return `${props.name}: ${props.min} - ${props.max}`;
        case "textarea":
            return `${props.name}`;
        case "text":
            return `${props.name}`;
        case "markdown":
            return `${props.name}`;
        case "richtext":
            return `${props.name}`;
        case "checkbox":
            return `${props.name}`;
        case "select":
            return `${props.name}`;
        case "code":
            return `${props.name}`;
        case "range":
            return `${props.name}`;
        case "buttongroup":
            return `${props.name}`;
        case "button":
            return `${props.name}`;
    }
});
</script>
<template>
    <div class="flex gap-2 w-full items-center">
        <UIcon name="i-tabler-grip-vertical" class="cursor-grab" />
        <label>
            <span class="font-bold">
                {{ $t(`builder.item.label.${type}`) }}
            </span>
            {{ label[locale] }}
            <span class="text-red-600" v-if="required"> * </span>
        </label>
        <p class="italic text-gray-600 dark:text-gray-400 text-sm" v-if="extra">{{ extra }}</p>
        <div class="flex-1"></div>
        <UButton color="neutral" variant="ghost" icon="i-tabler-edit" @click="$emit('edit')" />
        <UButton color="error" variant="ghost" icon="i-tabler-trash" @click="$emit('delete')" />
    </div>
</template>
