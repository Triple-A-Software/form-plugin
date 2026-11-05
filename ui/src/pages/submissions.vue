<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
import type { TableColumn } from "@nuxt/ui";
import InputDate from "@/components/input/Date.vue";
import { computed, ref, shallowRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { SubmissionApiResponse } from "../../../bindings";
import { CalendarDate, getLocalTimeZone } from "@internationalized/date";
import { formatDateTime, formatRelative, submissionEntries, submissionFileDownloadLink } from "@/utils";
import { useForms } from "@/composables/useForms";
import { refDebounced, useUrlSearchParams } from "@vueuse/core";

const { t, locale } = useI18n();

const columns = computed<TableColumn<SubmissionApiResponse>[]>(() => [
    { id: "select" },
    { id: "expand" },
    {
        accessorKey: "timestamp",
        header: t("table_columns.timestamp"),
    },
    {
        id: "form",
        header: t("table_columns.form"),
    },
    {
        id: "summary",
        header: t("table_columns.summary"),
    },
    {
        id: "actions",
        meta: { class: { td: "text-right" } },
    },
]);
const expanded = ref({});
const rowSelection = ref<Record<string, boolean>>({});

// the form filter is in the url, so the forms overview can link to the submissions of a form
const search = useUrlSearchParams<{ form: string | undefined }>("hash", {
    removeFalsyValues: true,
    removeNullishValues: true,
});
const filterByForm = computed({
    get: () => (search.form ? Number(search.form) : undefined),
    set: (id) => {
        search.form = id === undefined ? undefined : String(id);
    },
});
const searchText = ref("");
const debouncedSearch = refDebounced(searchText, 300);
const page = ref(1);
const itemsPerPage = ref(50);
// a new filter has fewer pages, so staying on the current page would show an empty list
watch([filterByForm, debouncedSearch], () => {
    page.value = 1;
});
watch([filterByForm, debouncedSearch, page], () => {
    rowSelection.value = {};
});

const forms = useForms();
const formsById = computed(() => new Map((forms.data.value ?? []).map((f) => [f.id, f])));
const onlyExportForm = ref<number | undefined>(undefined);
const startDate = shallowRef<CalendarDate | undefined>(undefined);
const endDate = shallowRef<CalendarDate | undefined>(undefined);
async function exportSubmissions() {
    let _path = "/api/submissions/export";
    if (import.meta.env.MODE === "production") {
        _path = `${__API_BASE__}${_path}`;
    }
    const query = new URLSearchParams();
    if (onlyExportForm.value) query.set("form_id", String(onlyExportForm.value));
    if (startDate.value) query.set("start_date", startDate.value.toDate(getLocalTimeZone()).toISOString());
    if (endDate.value) query.set("end_date", endDate.value.toDate(getLocalTimeZone()).toISOString());
    _path += `?${query.toString()}`;
    window.location.assign(_path);
}
const exportOpen = ref(false);
watch(exportOpen, (open) => {
    // start the export from what is currently shown
    if (open) onlyExportForm.value = filterByForm.value;
});

const listQuery = () => ({
    form_id: filterByForm.value,
    search: debouncedSearch.value || undefined,
    page: page.value - 1,
    items_per_page: itemsPerPage.value,
});
const submissions = useApiQuery("/api/submissions", {
    method: "GET",
    query: listQuery,
});
const submissionCount = useApiQuery("/api/submissions/count", {
    method: "GET",
    query: listQuery,
});

const deleteMutation = useApiMutation(
    "/api/submissions",
    {
        method: "DELETE",
    },
    {
        invalidate: [submissions, submissionCount],
    },
);
const selectedIds = computed(() => Object.keys(rowSelection.value).filter((id) => rowSelection.value[id]));
async function deleteSelected() {
    try {
        await deleteMutation.mutateAsync({ ids: selectedIds.value });
        rowSelection.value = {};
    } catch {
        // the api toolkit already shows an error toast, keep the selection to retry
    }
}

const booleanLabels = computed(() => ({ yes: t("action.yes"), no: t("action.no") }));
function entries(row: SubmissionApiResponse) {
    return submissionEntries(row.data, formsById.value.get(row.form_id), locale.value, booleanLabels.value);
}
function summary(row: SubmissionApiResponse) {
    return entries(row)
        .filter((e) => e.value !== "–")
        .slice(0, 3)
        .map((e) => e.value)
        .join(" · ");
}

// shortcut: "new" is tracked per browser in localStorage, a server side read state needs a migration and per user data
const SEEN_KEY = "form-plugin:submissions-seen";
function readSeen() {
    try {
        return Number(localStorage.getItem(SEEN_KEY)) || undefined;
    } catch {
        return undefined;
    }
}
const lastSeen = readSeen();
try {
    localStorage.setItem(SEEN_KEY, String(Date.now()));
} catch {
    // private mode or blocked storage, everything just isn't marked as new
}
const isNew = (row: SubmissionApiResponse) => lastSeen !== undefined && new Date(row.timestamp).getTime() > lastSeen;
</script>

<template>
    <div class="flex flex-wrap items-center gap-2 py-3">
        <UInput
            v-model="searchText"
            icon="i-tabler-search"
            :placeholder="$t('field.placeholder.searchSubmissions')"
            class="w-full sm:max-w-64"
        />
        <UFieldGroup class="w-full sm:max-w-56">
            <USelectMenu
                v-model="filterByForm"
                :placeholder="$t('field.placeholder.filterByForm')"
                :items="forms.data.value"
                value-key="id"
                label-key="name"
                class="w-full"
            />
            <UButton
                v-if="filterByForm !== undefined"
                icon="i-tabler-filter-off"
                color="neutral"
                variant="subtle"
                :aria-label="$t('action.clear')"
                @click="filterByForm = undefined"
            />
        </UFieldGroup>

        <DeleteModal
            v-if="selectedIds.length > 0"
            :name="$t('deleteName.submission')"
            :id="0"
            :title="$t('modal.deleteSelected.title', selectedIds.length)"
            :description="$t('modal.deleteSelected.description', selectedIds.length)"
            @confirm="deleteSelected"
        >
            <UButton color="error" variant="subtle" icon="i-tabler-trash">
                {{ $t("action.deleteSelected", selectedIds.length) }}
            </UButton>
        </DeleteModal>

        <div class="flex-1" />

        <UModal v-model:open="exportOpen" :title="$t('modal.exportSubmissions.title')" :ui="{ body: 'space-y-4' }">
            <UTooltip :text="$t('field.tooltip.exportSubmissions')">
                <UButton color="neutral" variant="outline" icon="i-tabler-table-export">{{ $t("action.exportSubmissions") }}</UButton>
            </UTooltip>

            <template #body>
                <UFormField :label="$t('field.label.onlyExportForm')">
                    <UFieldGroup class="w-full">
                        <USelectMenu
                            v-model="onlyExportForm"
                            :placeholder="$t('field.placeholder.selectForm')"
                            :items="forms.data.value"
                            value-key="id"
                            label-key="name"
                            class="w-full"
                        />
                        <UButton
                            icon="i-tabler-filter-off"
                            color="neutral"
                            variant="subtle"
                            :aria-label="$t('action.clear')"
                            @click="onlyExportForm = undefined;"
                        />
                    </UFieldGroup>
                </UFormField>

                <UFormField :label="$t('field.label.startDate')">
                    <InputDate v-model="startDate" past />
                </UFormField>

                <UFormField :label="$t('field.label.endDate')">
                    <InputDate v-model="endDate" past />
                </UFormField>
            </template>

            <template #footer>
                <UButton @click="exportSubmissions" icon="i-tabler-table-export">{{ $t("action.exportSubmissions") }}</UButton>
            </template>
        </UModal>
    </div>

    <UTable
        :columns="columns"
        :data="submissions.data.value ?? []"
        v-model:expanded="expanded"
        v-model:row-selection="rowSelection"
        :get-row-id="(row) => row.id"
        :loading="submissions.isLoading.value"
        :ui="{ td: 'whitespace-normal wrap-anywhere' }"
    >
        <template #select-header="{ table }">
            <UCheckbox
                :model-value="table.getIsSomePageRowsSelected() ? 'indeterminate' : table.getIsAllPageRowsSelected()"
                :aria-label="$t('action.selectAll')"
                @update:model-value="(value) => table.toggleAllPageRowsSelected(!!value)"
            />
        </template>
        <template #select-cell="{ row }">
            <UCheckbox
                :model-value="row.getIsSelected()"
                :aria-label="$t('action.select')"
                @update:model-value="(value) => row.toggleSelected(!!value)"
            />
        </template>

        <template #expand-cell="{ row }">
            <UButton
                color="neutral"
                variant="ghost"
                :ui="{
                    leadingIcon: ['transition-transform', row.getIsExpanded() ? 'duration-100 rotate-180' : ''].join(' '),
                }"
                icon="i-tabler-chevron-down"
                :aria-label="$t('action.showDetails')"
                :aria-expanded="row.getIsExpanded()"
                @click="row.toggleExpanded()"
            />
        </template>

        <template #timestamp-cell="{ row }">
            <div class="flex items-center gap-2">
                <UTooltip :text="formatRelative(row.original.timestamp, locale)">
                    <span>{{ formatDateTime(row.original.timestamp, locale) }}</span>
                </UTooltip>
                <UBadge v-if="isNew(row.original)" size="sm" variant="subtle">{{ $t("badge.new") }}</UBadge>
            </div>
        </template>

        <template #form-cell="{ row }">
            <FormLink :id="row.original.form_id" />
        </template>

        <template #summary-cell="{ row }">
            <span class="text-muted line-clamp-1">{{ summary(row.original) }}</span>
        </template>

        <template #expanded="{ row }">
            <dl class="grid grid-cols-[max-content_1fr] gap-x-6 gap-y-1 px-2 text-sm">
                <template v-for="entry in entries(row.original)" :key="entry.name">
                    <dt class="font-medium">{{ entry.label }}</dt>
                    <dd class="whitespace-pre-wrap wrap-anywhere">{{ entry.value }}</dd>
                </template>
                <template v-if="row.original.file_count > 0">
                    <dt class="font-medium">{{ $t("table_columns.files") }}</dt>
                    <dd>
                        <UButton
                            variant="link"
                            color="neutral"
                            class="p-0"
                            icon="i-tabler-download"
                            @click="submissionFileDownloadLink(row.original)"
                        >
                            {{ $t("action.downloadFiles", Number(row.original.file_count)) }}
                        </UButton>
                    </dd>
                </template>
            </dl>
        </template>

        <template #actions-cell="{ row }">
            <div class="flex flex-row gap-1 items-center justify-end">
                <UTooltip v-if="row.original.file_count > 0" :text="$t('field.tooltip.downloadAttachments')">
                    <UButton
                        variant="ghost"
                        color="neutral"
                        icon="i-tabler-download"
                        :aria-label="$t('field.tooltip.downloadAttachments')"
                        @click="submissionFileDownloadLink(row.original)"
                    />
                </UTooltip>
                <SendEmailModal :submission="row.original" />
                <DeleteModal
                    :name="$t('deleteName.submission')"
                    :id="row.original.id"
                    @confirm="deleteMutation.mutate({ ids: [row.original.id] })"
                >
                    <UButton color="error" variant="ghost" icon="i-tabler-trash" :aria-label="$t('action.delete')" />
                </DeleteModal>
            </div>
        </template>

        <template #empty>
            <div class="flex flex-col items-center gap-3 py-8">
                <UIcon name="i-tabler-inbox" class="size-10 text-muted" />
                <p class="text-muted">{{ $t("empty.submissions") }}</p>
            </div>
        </template>
    </UTable>

    <div class="flex justify-center py-4" v-if="(submissionCount.data.value ?? 0) > itemsPerPage">
        <UPagination v-model:page="page" :total="submissionCount.data.value" :items-per-page="itemsPerPage" />
    </div>
</template>
