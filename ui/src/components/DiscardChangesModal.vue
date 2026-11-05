<script setup lang="ts">
const open = defineModel<boolean>("open", { default: false });
const emit = defineEmits<{
    (e: "discard"): void;
}>();
function discard() {
    open.value = false;
    emit("discard");
}
</script>
<template>
    <UModal v-model:open="open" :title="$t('modal.discard.title')" :description="$t('modal.discard.description')">
        <template #footer>
            <div class="flex flex-row items-center justify-end gap-2 w-full">
                <UButton color="neutral" variant="subtle" @click="open = false;">
                    {{ $t("action.keepEditing") }}
                </UButton>
                <UButton color="error" icon="i-tabler-trash" @click="discard">
                    {{ $t("action.discard") }}
                </UButton>
            </div>
        </template>
    </UModal>
</template>
