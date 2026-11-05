<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../bindings";

defineProps<{
    element: Property;
    error?: string;
    open?: boolean;
}>();

defineEmits<{
    (e: "edit"): void;
    (e: "duplicate"): void;
    (e: "delete"): void;
}>();
const { locale } = useI18n();
</script>
<template>
    <div
        class="flex gap-2 w-full items-center rounded-md px-1"
        :class="[error ? 'ring-1 ring-error' : '', open ? 'bg-elevated' : '']"
    >
        <UIcon name="i-tabler-grip-vertical" class="drag-handle cursor-grab shrink-0" />
        <label class="min-w-0 truncate">
            <span class="font-bold">
                {{ $t(`builder.item.label.${element.type}`) }}
            </span>
            <span class="ms-4" v-if="element.type !== 'captcha'">
                <template v-if="element.label[locale]">{{ element.label[locale] }}</template>
                <span v-else class="italic text-warning">{{ $t("builder.noLabel") }}</span>
            </span>
            <span class="text-error" v-if="element.type === 'captcha' || element.required"> * </span>
        </label>
        <template v-if="element.type !== 'captcha'">
            <USeparator orientation="vertical" class="h-4" v-if="element.name || element.hint?.[locale]" />
            <p class="italic text-muted text-sm font-mono" v-if="element.name">{{ element.name }}</p>
            <USeparator orientation="vertical" class="h-4" v-if="element.name && element.hint?.[locale]" />
            <p class="italic text-muted text-sm truncate hidden md:block" v-if="element.hint?.[locale]">
                {{ element.hint[locale] }}
            </p>
        </template>
        <div class="flex-1"></div>
        <p v-if="error" class="text-error text-sm flex items-center gap-1">
            <UIcon name="i-tabler-alert-circle" class="shrink-0" />
            {{ error }}
        </p>
        <UButton
            v-if="element.type !== 'captcha'"
            color="neutral"
            :variant="open ? 'soft' : 'ghost'"
            icon="i-tabler-edit"
            :aria-label="$t('action.edit')"
            :aria-expanded="open"
            @click="$emit('edit')"
        />
        <UButton
            v-if="element.type !== 'captcha'"
            color="neutral"
            variant="ghost"
            icon="i-tabler-copy"
            :aria-label="$t('action.duplicate')"
            @click="$emit('duplicate')"
        />
        <UButton color="error" variant="ghost" icon="i-tabler-trash" :aria-label="$t('action.delete')" @click="$emit('delete')" />
    </div>
</template>
