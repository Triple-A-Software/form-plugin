<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Form } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { useApiMutation } from "@/utils/api";
import clone from "just-clone";
import { useUrlSearchParams } from "@vueuse/core";
import { useForms } from "@/composables/useForms";

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

const forms = useForms();

const editingForm = ref<Form | undefined>(undefined);
const emptyForm: Form = {
    id: -1,
    send_confirmation_email: false,
    confirmation_email_id: undefined,
    confirmation_email_recipient_field_name: undefined,
    activate_cleverreach: false,
    cleverreach_group_id: undefined,
    cleverreach_condition: {
        type: "OnSubmit",
    },
    send_submission_as_email_enabled: false,
    send_submission_as_email: undefined,
    submissions_email_id: undefined,
    cleverreach_email_field_name: undefined,
    cleverreach_firstname_field_name: undefined,
    cleverreach_lastname_field_name: undefined,
    cleverreach_salutation: undefined,
    name: "",
    props: [],
    activate_captcha: false,
    captcha_type: "h-captcha",
    captcha_callback: undefined,
    captcha_expired_callback: undefined,
    additional_html: "",
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
const saveEditMutation = useApiMutation(
    "/api/forms",
    { method: "PUT" },
    {
        invalidate: [forms],
    },
);
async function saveForm() {
    if (!editingForm.value) {
        return;
    }
    if (editingForm.value.id !== -1) {
        saveEditMutation.mutate(editingForm.value);
    } else {
        const result = await saveCreateMutation.mutateAsync(editingForm.value);
        editingForm.value = result;
    }
}

const deleteMutation = useApiMutation(
    "/api/forms",
    {
        method: "DELETE",
    },
    {
        invalidate: [forms],
    },
);

const search = useUrlSearchParams<{ id: string | undefined; edit: boolean | undefined; create: boolean | undefined }>("hash", {
    removeFalsyValues: true,
    removeNullishValues: true,
});
const editModalOpen = computed({
    get() {
        return Boolean(search.edit);
    },
    set(v) {
        if (v) {
            search.edit = true;
        } else {
            search.edit = undefined;
            search.id = undefined;
        }
    },
});
const createModalOpen = computed({
    get() {
        return Boolean(search.create);
    },
    set(v) {
        if (v) {
            search.create = true;
        } else {
            search.create = undefined;
        }
    },
});

watch(
    () => [search.id, forms.data.value] as const,
    ([id, data]) => {
        if (id && data) {
            const form = data.find((f) => f.id === Number(id));
            if (form) {
                editingForm.value = clone(form);
            }
        }
    },
    {
        immediate: true,
    },
);
function editForm(id: number) {
    search.id = String(id);
}
</script>

<template>
    <div class="flex flex-row">
        <div class="flex-1" />
        <UModal :title="$t('modal.create.form')" fullscreen v-model:open="createModalOpen">
            <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.form") }}</UButton>
            <template #body v-if="editingForm">
                <FormForm v-model="editingForm" />
                <FormPreview :form="editingForm" />
            </template>
            <template #footer>
                <ModalSaveButtons @save="saveForm()" @close="createModalOpen = false" />
            </template>
        </UModal>
    </div>

    <UTable :columns="columns" :data="forms.data.value" v-model:expanded="expanded" :ui="{ td: 'wrap-break-word' }">
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
                <UButton color="neutral" variant="ghost" icon="i-tabler-pencil" @click="editForm(row.original.id)" />
                <template #body v-if="editingForm">
                    <div class="space-y-4">
                        <FormForm v-model="editingForm" />
                        <FormPreview :form="editingForm" />
                    </div>
                </template>
                <template #footer>
                    <ModalSaveButtons @save="saveForm()" @close="editModalOpen = false" />
                </template>
            </UModal>
            <DeleteModal
                :name="$t('deleteName.form')"
                :id="row.original.id"
                @confirm="deleteMutation.mutate({ id: row.original.id })"
            >
                <UButton color="error" variant="ghost" icon="i-tabler-trash" />
            </DeleteModal>
        </template>
    </UTable>
</template>
