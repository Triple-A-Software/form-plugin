<script setup lang="ts">
import { computed, ref } from "vue";
import type { Form } from "../../../bindings";
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
        accessorKey: "name",
        header: t("table_columns.name"),
    },
    {
        id: "actions",
    },
]);
const expanded = ref({});

const forms = useApiQuery("/api/forms", {
    method: "GET",
});

const editingForm = ref<NullToUndefined<Form> | undefined>(undefined);
const emptyForm: NullToUndefined<Form> = {
    id: -1,
    send_confirmation_email: false,
    confirmation_email_id: undefined,
    confirmation_email_recipient_field_name: undefined,
    name: "",
    props: [],
};
function create() {
    editingForm.value = clone(emptyForm);
}

const saveCreateMutation = useApiMutation(
    "/api/forms",
    { method: "POST" },
    {
        invalidate: [forms],
    },
);
function saveCreateForm() {
    if (!editingForm.value) {
        return;
    }
    saveCreateMutation.mutate(editingForm.value);
}

const saveEditMutation = useApiMutation(
    "/api/forms",
    { method: "PUT" },
    {
        invalidate: [forms],
    },
);
function saveEditForm() {
    if (!editingForm.value) {
        return;
    }
    saveEditMutation.mutate(editingForm.value);
}

async function deleteForm(id: number) {
    await fetchApi("/api/forms", {
        method: "DELETE",
        query: {
            id,
        },
    });
    forms.invalidate();
}

const editModalOpen = ref(false);
const createModalOpen = ref(false);
</script>

<template>
    <div class="flex flex-row">
        <div class="flex-1" />
        <UModal :title="$t('modal.create.form')" fullscreen v-model:open="createModalOpen">
            <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.form") }}</UButton>
            <template #body v-if="editingForm">
                <FormForm v-model="editingForm" />
            </template>
            <template #footer>
                <ModalSaveButtons @save="saveCreateForm()" @close="createModalOpen = false" />
            </template>
        </UModal>
    </div>
    <UTable :columns="columns" :data="forms.data.value" v-model:expanded="expanded">
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
            <UModal :title="t('modal.edit.form')" fullscreen v-model:open="editModalOpen">
                <UButton color="neutral" variant="ghost" icon="i-tabler-pencil" @click="editingForm = row.original" />
                <template #body v-if="editingForm">
                    <div class="space-y-4">
                        <FormForm v-model="editingForm" />
                        <FormPreview :form="editingForm" />
                    </div>
                </template>
                <template #footer>
                    <ModalSaveButtons @save="saveEditForm()" @close="editModalOpen = false" />
                </template>
            </UModal>
            <DeleteModal :name="$t('deleteName.form')" :id="row.original.id" @confirm="deleteForm">
                <UButton color="error" variant="ghost" icon="i-tabler-trash" />
            </DeleteModal>
        </template>
    </UTable>
</template>
