<script setup lang="ts">
defineProps<{
    loading?: boolean;
    disabled?: boolean;
}>();
defineEmits<{
    (e: "save", close: boolean): void;
}>();
</script>
<template>
    <UButton icon="i-tabler-device-floppy" :loading="loading" :disabled="disabled" @click="$emit('save', false)">
        {{ $t("action.save") }}
    </UButton>
    <UButton
        color="neutral"
        variant="subtle"
        icon="i-tabler-device-floppy"
        :loading="loading"
        :disabled="disabled"
        @click="$emit('save', true)"
    >
        {{ $t("action.saveAndClose") }}
    </UButton>
</template>
