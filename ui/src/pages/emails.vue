<script setup lang="ts">
import { computed, ref } from "vue";
import type { Email } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { useApiMutation, useApiQuery } from "@/utils/api";
import { useForms } from "@/composables/useForms";
import type { TableColumn } from "@nuxt/ui";
import clone from "just-clone";
import { useEventListener } from "@vueuse/core";

const { t } = useI18n();

const columns = computed<TableColumn<Email>[]>(() => [
    {
        accessorKey: "subject",
        header: t("table_columns.subject"),
    },
    {
        id: "usedBy",
        header: t("table_columns.usedBy"),
    },
    {
        id: "actions",
        meta: { class: { td: "text-right" } },
    },
]);

const forms = useForms();
// a form referencing a deleted email would stop sending it without any error, so deleting is blocked
function usedBy(email: Email) {
    return (forms.data.value ?? []).filter(
        (f) => f.confirmation_email_id === email.id || f.submissions_email_id === email.id,
    );
}

const emails = useApiQuery("/api/emails", {
    method: "GET",
});

const editingEmail = ref<Email | undefined>(undefined);
const emptyEmail: Email = {
    id: -1,
    subject: "",
    content: "",
};
const savedJson = ref<string>();
const dirty = computed(() => editingEmail.value !== undefined && JSON.stringify(editingEmail.value) !== savedJson.value);
function load(email: Email) {
    // edit a copy, so cancelling doesn't leave unsaved text in the cached list
    editingEmail.value = clone(email);
    savedJson.value = JSON.stringify(editingEmail.value);
}
function closeEditor() {
    editingEmail.value = undefined;
    attemptedSave.value = false;
}
const discardOpen = ref(false);
const editorOpen = computed({
    get: () => editingEmail.value !== undefined,
    set(v) {
        if (v) return;
        if (dirty.value) discardOpen.value = true;
        else closeEditor();
    },
});
useEventListener(window, "beforeunload", (event) => {
    if (dirty.value) event.preventDefault();
});
function create() {
    load(emptyEmail);
}
function edit(email: Email) {
    load(email);
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
const attemptedSave = ref(false);
const subjectError = computed(() =>
    attemptedSave.value && !editingEmail.value?.subject.trim() ? t("validation.required") : undefined,
);
const saving = computed(() => saveCreateMutation.isLoading.value || saveEditMutation.isLoading.value);
async function saveEmail(close: boolean) {
    const email = editingEmail.value;
    if (!email || saving.value) {
        return;
    }
    attemptedSave.value = true;
    if (!email.subject.trim()) {
        return;
    }
    const json = JSON.stringify(email);
    try {
        if (email.id !== -1) {
            await saveEditMutation.mutateAsync(email);
            savedJson.value = json;
        } else {
            const result = await saveCreateMutation.mutateAsync(email);
            if (result) load(result);
        }
        if (close) editorOpen.value = false;
    } catch {
        // the api toolkit already shows an error toast, keep the editor open so nothing is lost
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
</script>
<template>
    <div class="flex flex-row py-3">
        <div class="flex-1" />
        <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.email") }}</UButton>
    </div>
    <UModal
        :title="editingEmail?.id === -1 ? $t('modal.create.email') : $t('modal.edit.email')"
        fullscreen
        v-model:open="editorOpen"
    >
        <template #body>
            <EmailForm v-if="editingEmail" v-model="editingEmail" :subject-error="subjectError" />
        </template>
        <template #footer>
            <ModalSaveButtons :loading="saving" @save="saveEmail" />
            <UBadge v-if="dirty" color="warning" variant="subtle" icon="i-tabler-point-filled">
                {{ $t("hint.unsavedChanges") }}
            </UBadge>
        </template>
    </UModal>
    <DiscardChangesModal v-model:open="discardOpen" @discard="closeEditor" />
    <UTable :columns="columns" :data="emails.data.value ?? []" :loading="emails.isLoading.value" :ui="{ td: 'whitespace-normal wrap-anywhere' }">
        <template #subject-cell="{ row }">
            <button type="button" class="text-left font-medium hover:underline cursor-pointer" @click="edit(row.original)">
                {{ row.original.subject || "–" }}
            </button>
        </template>
        <template #usedBy-cell="{ row }">
            <div class="flex flex-wrap gap-1">
                <UBadge
                    v-for="form in usedBy(row.original)"
                    :key="form.id"
                    color="neutral"
                    variant="subtle"
                    :label="form.name"
                />
                <span v-if="usedBy(row.original).length === 0" class="text-muted">–</span>
            </div>
        </template>
        <template #actions-cell="{ row }">
            <div class="flex justify-end gap-1">
                <UTooltip :text="$t('action.edit')">
                    <UButton
                        color="neutral"
                        variant="ghost"
                        icon="i-tabler-pencil"
                        :aria-label="$t('action.edit')"
                        @click="edit(row.original)"
                    />
                </UTooltip>
                <DeleteModal
                    :name="$t('deleteName.email')"
                    :id="row.original.id"
                    :blocked="usedBy(row.original).length > 0"
                    @confirm="deleteMutation.mutate({ id: row.original.id })"
                >
                    <UButton color="error" variant="ghost" icon="i-tabler-trash" :aria-label="$t('action.delete')" />
                    <template #extra>
                        <UAlert
                            v-if="usedBy(row.original).length > 0"
                            class="mt-4"
                            color="warning"
                            variant="subtle"
                            icon="i-tabler-alert-triangle"
                            :description="
                                $t('alert.emailInUse', { forms: usedBy(row.original).map((f) => f.name).join(', ') })
                            "
                        />
                    </template>
                </DeleteModal>
            </div>
        </template>
        <template #empty>
            <div class="flex flex-col items-center gap-3 py-8">
                <UIcon name="i-tabler-mail" class="size-10 text-muted" />
                <p class="text-muted">{{ $t("empty.emails") }}</p>
                <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.email") }}</UButton>
            </div>
        </template>
    </UTable>
</template>
