<script setup lang="ts">
import { computed, ref } from "vue";
import type { Email } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { useApiMutation, useApiQuery } from "@/utils/api";
import clone from "just-clone";

const { t } = useI18n();

const columns = computed(() => [
    {
        id: "expand",
    },
    {
        accessorKey: "subject",
        header: t("table_columns.subject"),
    },
    {
        id: "actions",
    },
]);
const expanded = ref({});

const emails = useApiQuery("/api/emails", {
    method: "GET",
});

const editingEmail = ref<Email | undefined>(undefined);
const emptyEmail: Email = {
    id: -1,
    subject: "",
    content: "",
};
function create() {
    editingEmail.value = clone(emptyEmail);
}

const saveCreateMutation = useApiMutation(
    "/api/emails",
    { method: "POST" },
    {
        invalidate: [emails],
    },
);
const saveEditMutation = useApiMutation(
    "/api/emails",
    { method: "PUT" },
    {
        invalidate: [emails],
    },
);
async function saveEmail() {
    if (!editingEmail.value) {
        return;
    }
    if (editingEmail.value.id !== -1) {
        saveEditMutation.mutate(editingEmail.value);
    } else {
        const result = await saveCreateMutation.mutateAsync(editingEmail.value);
        editingEmail.value = result;
    }
}

const deleteMutation = useApiMutation(
    "/api/emails",
    {
        method: "DELETE",
    },
    {
        invalidate: [emails],
    },
);

const editModalOpen = ref(false);
const createModalOpen = ref(false);
</script>
<template>
    <div class="flex flex-row">
        <div class="flex-1" />
        <UModal :title="$t('modal.create.email')" fullscreen v-model:open="createModalOpen">
            <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.email") }}</UButton>
            <template #body v-if="editingEmail">
                <EmailForm v-model="editingEmail" />
            </template>
            <template #footer>
                <ModalSaveButtons @save="saveEmail()" @close="createModalOpen = false" />
            </template>
        </UModal>
    </div>
    <UTable :columns="columns" :data="emails.data.value" v-model:expanded="expanded" :ui="{ td: 'whitespace-normal wrap-anywhere' }">
        <template #expanded="{ row }">
            <div v-for="(value, key) in row.original">
                <p class="pl-2 -indent-2 whitespace-normal wrap-anywhere"><span class="font-bold">{{ key }}:</span> {{ value }}</p>
            </div>
        </template>
        <template #expand-cell="{ row }">
            <UButton
                color="neutral"
                variant="ghost"
                :ui="{
                    leadingIcon: ['transition-transform', row.getIsExpanded() ? 'duration-100 rotate-180' : ''].join(' '),
                }"
                icon="i-tabler-chevron-down"
                @click="row.toggleExpanded()"
            />
        </template>
        <template #actions-cell="{ row }">
            <UModal :title="t('modal.edit.email')" fullscreen v-model:open="editModalOpen">
                <UButton color="neutral" variant="ghost" icon="i-tabler-pencil" @click="editingEmail = row.original" />
                <template #body v-if="editingEmail">
                    <EmailForm v-model="editingEmail" />
                </template>
                <template #footer>
                    <ModalSaveButtons @save="saveEmail()" @close="editModalOpen = false" />
                </template>
            </UModal>
            <DeleteModal
                :name="$t('deleteName.email')"
                :id="row.original.id"
                @confirm="deleteMutation.mutate({ id: row.original.id })"
            >
                <UButton color="error" variant="ghost" icon="i-tabler-trash" />
            </DeleteModal>
        </template>
    </UTable>
</template>
