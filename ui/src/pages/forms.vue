<script setup lang="ts">
import { computed, ref } from "vue";
import type { Form } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { useApiMutation, useApiQuery } from "@/utils/api";
import clone from "just-clone";
import type { NullToUndefined } from "@/utils/types/utils";
import { undefinedToNull, nullToUndefined } from "@/utils/utils";

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

const saveCreateMutation = useApiMutation("/api/forms", { method: "POST" });
function saveCreateForm() {
    if (!editingForm.value) {
        return;
    }
    const body = undefinedToNull(editingForm.value);
    saveCreateMutation.mutate(body);
}

const saveEditMutation = useApiMutation("/api/forms", { method: "PUT" });
function saveEditForm() {
    if (!editingForm.value) {
        return;
    }
    const body = undefinedToNull(editingForm.value);
    saveEditMutation.mutate(body);
}

const editModalOpen = ref(false);
const createModalOpen = ref(false);
</script>

<template>
    <div class="flex flex-row">
        <div class="flex-1" />
        <UModal :title="$t('modal.create.form')" fullscreen v-model="createModalOpen">
            <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.form") }}</UButton>
            <template #body v-if="editingForm">
                <FormForm v-model="editingForm" />
            </template>
            <template #footer>
                <UButton icon="i-tabler-device-floppy" @click="saveCreateForm()">{{ $t("action.save") }}</UButton>
                <UButton
                    icon="i-tabler-device-floppy"
                    @click="
                        saveCreateForm();
                        createModalOpen = false;
                    "
                    >{{ $t("action.saveAndClose") }}</UButton
                >
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
            <UModal :title="t('modal.edit.form')" fullscreen v-model="editModalOpen">
                <UButton
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-pencil"
                    @click="editingForm = nullToUndefined(row.original)"
                />
                <template #body v-if="editingForm">
                    <FormForm v-model="editingForm" />
                </template>
                <template #footer>
                    <UButton icon="i-tabler-device-floppy" @click="saveEditForm()">{{ $t("action.save") }}</UButton>
                    <UButton
                        icon="i-tabler-device-floppy"
                        @click="
                            saveEditForm();
                            editModalOpen = false;
                        "
                        >{{ $t("action.saveAndClose") }}</UButton
                    >
                </template>
            </UModal>
            <UButton color="error" variant="ghost" icon="i-tabler-trash" />
        </template>
    </UTable>
</template>
