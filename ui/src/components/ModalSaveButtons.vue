<script setup lang="ts">
defineEmits<{
    (e: "save"): void;
    (e: "close"): void;
}>();
</script>
<template>
    <UButton icon="i-tabler-device-floppy" @click="$emit('save')">
        {{ $t("action.save") }}
    </UButton>
    <UButton
        color="neutral"
        variant="subtle"
        icon="i-tabler-device-floppy"
        @click="
            $emit('save');
            $emit('close');
        "
    >
        {{ $t("action.saveAndClose") }}
    </UButton>
</template>
