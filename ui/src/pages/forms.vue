<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { Form } from "../../../bindings";
import { useI18n } from "vue-i18n";
import { useApiMutation, useApiQuery } from "@/utils/api";
import clone from "just-clone";
import { useEventListener, useUrlSearchParams } from "@vueuse/core";
import { useForms } from "@/composables/useForms";
import type { TableColumn } from "@nuxt/ui";
import { dataFields, formatDateTime, formatRelative, validateForm } from "@/utils";

const { t, locale } = useI18n();

const columns = computed<TableColumn<Form>[]>(() => [
    {
        accessorKey: "name",
        header: t("table_columns.name"),
    },
    {
        id: "fields",
        header: t("table_columns.fields"),
    },
    {
        id: "submissions",
        header: t("table_columns.submissions"),
    },
    {
        id: "lastSubmission",
        header: t("table_columns.lastSubmission"),
    },
    {
        id: "features",
        header: t("table_columns.features"),
    },
    {
        id: "actions",
        meta: { class: { td: "text-right" } },
    },
]);
const globalFilter = ref("");

const forms = useForms();
const stats = useApiQuery("/api/forms/stats", { method: "GET" });
const statsByForm = computed(() => new Map((stats.data.value ?? []).map((s) => [s.form_id, s])));
function features(form: Form) {
    return [
        form.send_confirmation_email && { icon: "i-tabler-mail-forward", label: t("field.label.sendConfirmationEmail") },
        form.send_submission_as_email_enabled && { icon: "i-tabler-bell", label: t("field.label.sendSubmissionAsEmail") },
        form.activate_captcha && { icon: "i-tabler-shield-check", label: t("field.label.activateCaptcha") },
        form.activate_cleverreach && { icon: "i-tabler-plug", label: t("field.label.activateCleverreach") },
    ].filter((f) => f !== false);
}

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
    email_attach_files: true,
    cleverreach_email_field_name: undefined,
    cleverreach_firstname_field_name: undefined,
    cleverreach_lastname_field_name: undefined,
    cleverreach_salutation: undefined,
    name: "",
    props: [],
    activate_captcha: false,
    captcha_callback: undefined,
    captcha_expired_callback: undefined,
    additional_html: "",
};

const search = useUrlSearchParams<{ id: string | undefined; edit: boolean | undefined; create: boolean | undefined }>("hash", {
    removeFalsyValues: true,
    removeNullishValues: true,
});
// the last saved (or loaded) state, to warn before closing the editor with unsaved changes
const savedJson = ref<string>();
const dirty = computed(() => editingForm.value !== undefined && JSON.stringify(editingForm.value) !== savedJson.value);
function load(form: Form) {
    editingForm.value = clone(form);
    savedJson.value = JSON.stringify(editingForm.value);
}
function closeEditor() {
    search.edit = undefined;
    search.create = undefined;
    search.id = undefined;
    editingForm.value = undefined;
    attemptedSave.value = false;
}
const discardOpen = ref(false);
const editorOpen = computed({
    get() {
        return Boolean(search.edit || search.create);
    },
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
    load(emptyForm);
    search.create = true;
}
function editForm(id: number) {
    search.id = String(id);
    search.edit = true;
}

watch(
    () => [search.id, search.create, forms.data.value] as const,
    ([id, creating, data]) => {
        if (creating && !editingForm.value) {
            load(emptyForm);
        }
        // only load when switching forms, so a refetch after saving doesn't overwrite ongoing edits
        if (id && data && editingForm.value?.id !== Number(id)) {
            const form = data.find((f) => f.id === Number(id));
            if (form) {
                load(form);
            }
        }
    },
    {
        immediate: true,
    },
);

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
const issues = computed(() => (editingForm.value ? validateForm(editingForm.value) : []));
// only show problems after the first save attempt, a new form would start out full of errors otherwise
const attemptedSave = ref(false);
const shownIssues = computed(() => (attemptedSave.value ? issues.value : []));
const saving = computed(() => saveCreateMutation.isLoading.value || saveEditMutation.isLoading.value);
async function saveForm(close: boolean) {
    const form = editingForm.value;
    if (!form || saving.value) {
        return;
    }
    attemptedSave.value = true;
    if (issues.value.length > 0) {
        return;
    }
    // snapshot before the request, edits made while saving still count as unsaved
    const json = JSON.stringify(form);
    try {
        if (form.id !== -1) {
            await saveEditMutation.mutateAsync(form);
            savedJson.value = json;
        } else {
            const result = await saveCreateMutation.mutateAsync(form);
            if (result) {
                load(result);
                search.create = undefined;
                search.id = String(result.id);
                search.edit = true;
            }
        }
        if (close) {
            editorOpen.value = false;
        }
    } catch {
        // the api toolkit already shows an error toast, keep the editor open so nothing is lost
    }
}

const deleteMutation = useApiMutation(
    "/api/forms",
    {
        method: "DELETE",
    },
    {
        invalidate: [forms, stats],
    },
);

async function duplicateForm(form: Form) {
    try {
        const copy = await saveCreateMutation.mutateAsync({ ...clone(form), id: -1, name: t("copyOf", { name: form.name }) });
        if (copy) editForm(copy.id);
    } catch {
        // the api toolkit already shows an error toast
    }
}
</script>

<template>
    <div class="flex flex-row items-center gap-2 py-3">
        <UInput
            v-model="globalFilter"
            icon="i-tabler-search"
            :placeholder="$t('field.placeholder.searchForms')"
            class="max-w-xs"
        />
        <div class="flex-1" />
        <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.form") }}</UButton>
    </div>

    <UModal
        :title="editingForm?.id === -1 ? $t('modal.create.form') : $t('modal.edit.form')"
        fullscreen
        v-model:open="editorOpen"
    >
        <template #body>
            <div v-if="editingForm" class="grid lg:grid-cols-2 gap-6 items-start">
                <div class="space-y-4 min-w-0">
                    <UAlert
                        v-if="shownIssues.length > 0"
                        color="error"
                        variant="subtle"
                        icon="i-tabler-alert-circle"
                        :title="$t('validation.title')"
                        :description="$t('validation.count', shownIssues.length)"
                    />
                    <FormForm v-model="editingForm" :issues="shownIssues" />
                </div>
                <div class="lg:sticky lg:top-0 min-w-0">
                    <FormPreview :form="editingForm" />
                </div>
            </div>
        </template>
        <template #footer>
            <ModalSaveButtons :loading="saving" @save="saveForm" />
            <UBadge v-if="dirty" color="warning" variant="subtle" icon="i-tabler-point-filled">
                {{ $t("hint.unsavedChanges") }}
            </UBadge>
        </template>
    </UModal>
    <DiscardChangesModal v-model:open="discardOpen" @discard="closeEditor" />

    <UTable
        :columns="columns"
        :data="forms.data.value ?? []"
        v-model:global-filter="globalFilter"
        :loading="forms.isLoading.value"
        :ui="{ td: 'wrap-break-word' }"
    >
        <template #name-cell="{ row }">
            <button type="button" class="text-left font-medium hover:underline cursor-pointer" @click="editForm(row.original.id)">
                {{ row.original.name }}
            </button>
            <span class="ms-2 text-muted text-xs font-mono">#{{ row.original.id }}</span>
        </template>

        <template #fields-cell="{ row }">
            {{ dataFields(row.original.props).length }}
        </template>

        <template #submissions-cell="{ row }">
            <UButton
                variant="link"
                color="neutral"
                :to="`/submissions?form=${row.original.id}`"
                :label="String(statsByForm.get(row.original.id)?.submission_count ?? 0)"
                class="p-0"
            />
        </template>

        <template #lastSubmission-cell="{ row }">
            <UTooltip
                v-if="statsByForm.get(row.original.id)"
                :text="formatDateTime(statsByForm.get(row.original.id)!.last_submission, locale)"
            >
                <span>{{ formatRelative(statsByForm.get(row.original.id)!.last_submission, locale) }}</span>
            </UTooltip>
            <span v-else class="text-muted">–</span>
        </template>

        <template #features-cell="{ row }">
            <div class="flex gap-1">
                <UTooltip v-for="feature in features(row.original)" :key="feature.icon" :text="feature.label">
                    <UIcon :name="feature.icon" class="size-5 text-muted" :aria-label="feature.label" />
                </UTooltip>
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
                        @click="editForm(row.original.id)"
                    />
                </UTooltip>
                <UTooltip :text="$t('action.duplicate')">
                    <UButton
                        color="neutral"
                        variant="ghost"
                        icon="i-tabler-copy"
                        :aria-label="$t('action.duplicate')"
                        @click="duplicateForm(row.original)"
                    />
                </UTooltip>
                <DeleteModal
                    :name="$t('deleteName.form')"
                    :id="row.original.id"
                    :blocked="Boolean(statsByForm.get(row.original.id))"
                    @confirm="deleteMutation.mutate({ id: row.original.id })"
                >
                    <UButton color="error" variant="ghost" icon="i-tabler-trash" :aria-label="$t('action.delete')" />
                    <template #extra>
                        <UAlert
                            v-if="statsByForm.get(row.original.id)"
                            class="mt-4"
                            color="warning"
                            variant="subtle"
                            icon="i-tabler-alert-triangle"
                            :description="
                                $t('alert.formHasSubmissions', statsByForm.get(row.original.id)!.submission_count)
                            "
                            :actions="[
                                {
                                    label: $t('action.goto.submissions'),
                                    to: `/submissions?form=${row.original.id}`,
                                },
                            ]"
                        />
                    </template>
                </DeleteModal>
            </div>
        </template>

        <template #empty>
            <div class="flex flex-col items-center gap-3 py-8">
                <UIcon name="i-tabler-forms" class="size-10 text-muted" />
                <p class="text-muted">{{ $t("empty.forms") }}</p>
                <UButton icon="i-tabler-plus" @click="create">{{ t("action.create.form") }}</UButton>
            </div>
        </template>
    </UTable>
</template>
