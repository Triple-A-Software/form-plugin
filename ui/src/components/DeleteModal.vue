<template>
    <UModal v-model:open="open">
        <template #title>
            <div class="flex flex-row gap-2 items-center">
                <UIcon name="i-tabler-trash" />
                {{ title ?? t("modal.delete.title", { name }) }}
            </div>
        </template>
        <template #body>
            <p>
                {{ description ?? t("modal.delete.description", { name }) }}
            </p>
            <slot name="extra" />
        </template>
        <slot />
        <template #footer>
            <div class="flex flex-row items-center justify-end gap-2 w-full">
                <UButton color="neutral" variant="subtle" icon="i-tabler-x" @click="open = false;">
                    {{ t("action.cancel") }}
                </UButton>
                <UButton color="error" icon="i-tabler-trash" :disabled="blocked" @click="confirm">
                    {{ t("action.delete") }}
                </UButton>
            </div>
        </template>
    </UModal>
</template>
<script setup lang="ts" generic="TId">
import { useI18n } from "vue-i18n";

const props = defineProps<{
    name: string;
    title?: string;
    description?: string;
    id: TId | null;
    // something still depends on the item, the slot "extra" explains what
    blocked?: boolean;
}>();
const emit = defineEmits<{
    (e: "confirm", id: TId): void;
}>();
const open = defineModel<boolean>("open", {
    default: false,
});
const { t } = useI18n();

async function confirm() {
    if (props.id !== null) {
        emit("confirm", props.id);
    }
    open.value = false;
}
</script>
