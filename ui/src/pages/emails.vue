<script setup lang="ts">
import { computed, ref } from "vue";
import type { Email } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { fetchApi, useApiMutation, useApiQuery } from "@/utils/api";
import clone from "just-clone";
import type { NullToUndefined } from "@/utils/types/utils";

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

const editingEmail = ref<NullToUndefined<Email> | undefined>(undefined);
const emptyEmail: NullToUndefined<Email> = {
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
function saveCreateEmail() {
    if (!editingEmail.value) {
        return;
    }
    saveCreateMutation.mutate(editingEmail.value);
}

const saveEditMutation = useApiMutation(
    "/api/emails",
    { method: "PUT" },
    {
        invalidate: [emails],
    },
);
function saveEditEmail() {
    if (!editingEmail.value) {
        return;
    }
    saveEditMutation.mutate(editingEmail.value);
}

async function deleteEmail(id: number) {
    await fetchApi("/api/emails", {
        method: "DELETE",
        query: {
            id,
        },
    });
    emails.invalidate();
}

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
                <ModalSaveButtons @save="saveCreateEmail()" @close="createModalOpen = false" />
            </template>
        </UModal>
    </div>
    <UTable :columns="columns" :data="emails.data.value" v-model:expanded="expanded">
        <template #expanded="{ row }">
            <pre>{{ row.original }}</pre>
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
                    <ModalSaveButtons @save="saveEditEmail()" @close="editModalOpen = false" />
                </template>
            </UModal>
            <DeleteModal :name="$t('deleteName.email')" :id="row.original.id" @confirm="deleteEmail">
                <UButton color="error" variant="ghost" icon="i-tabler-trash" />
            </DeleteModal>
        </template>
    </UTable>
</template>
