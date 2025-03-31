<template>
    <UPopover>
        <UButton
            tabindex="-1"
            variant="ghost"
            color="neutral"
            size="xs"
            icon="i-tabler-info-small"
            class="text-neutral-500 dark:text-neutral-400"
        />
        <template #panel>
            <div class="py-1 px-2 text-neutral-900 dark:text-white max-w-xs">
                <p>{{ text }}</p>
            </div>
        </template>
    </UPopover>
</template>

<script setup lang="ts">
defineProps<{
    text: string;
}>();
</script>
