<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
import type { TableColumn } from "@nuxt/ui";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Submission } from "../../../bindings";
import { useForms } from "@/store/forms";

const { t, locale } = useI18n();

const columns = computed<TableColumn<Submission>[]>(() => [
    {
        id: "expand",
    },
    {
        accessorKey: "timestamp",
        header: t("table_columns.timestamp"),
        cell: ({ row }) =>
            new Date(row.getValue("timestamp")).toLocaleString(locale.value, {
                day: "numeric",
                month: "short",
                year: "numeric",
                hour: "2-digit",
                minute: "2-digit",
                hour12: false,
            }),
    },
    {
        id: "form",
        header: t("table_columns.form"),
    },
    {
        id: "actions",
    },
]);
const expanded = ref({});

const filterByForm = ref<number | undefined>(undefined);
const page = ref(1);
const itemsPerPage = ref(100);
watch(
    () => [filterByForm.value, page.value, itemsPerPage.value],
    () => {
        submissions.refetch();
        submissionCount.refetch();
    },
);
const forms = useForms();
async function exportSubmissions() {
    let _path = "/api/submissions/export";
    if (import.meta.env.MODE === "production") {
        _path = `${__API_BASE__}${_path}`;
    }
    const query = new URLSearchParams();
    if (filterByForm.value) query.set("form_id", String(filterByForm.value));
    // these 2 are not used in the endpoint on the server, so it doesn't matter what is in here
    query.set("page", "1");
    query.set("items_per_page", "1");
    _path += `?${query.toString()}`;
    const response = await fetch(_path, { method: "GET" });
    const blob = await response.blob();
    const file = window.URL.createObjectURL(blob);
    window.location.assign(file);
}

const submissions = useApiQuery("/api/submissions", {
    method: "GET",
    query: () => {
        return {
            form_id: filterByForm.value,
            page: page.value - 1,
            items_per_page: itemsPerPage.value,
        };
    },
});
const submissionCount = useApiQuery("/api/submissions/count", {
    method: "GET",
    query: () => ({
        form_id: filterByForm.value,
        page: page.value - 1,
        items_per_page: itemsPerPage.value,
    }),
});

const deleteMutation = useApiMutation(
    "/api/submissions",
    {
        method: "DELETE",
    },
    {
        invalidate: [submissions],
    },
);
</script>

<template>
    <div class="flex items-center justify-between gap-2 px-4 py-3.5 overflow-x-auto">
        <UButtonGroup class="w-full max-w-48">
            <USelectMenu
                v-model="filterByForm"
                :placeholder="$t('field.placeholder.filterByForm')"
                :items="forms.forms.data"
                value-key="id"
                label-key="name"
                class="w-full"
            />
            <UButton icon="i-tabler-filter-off" color="neutral" variant="subtle" @click="filterByForm = undefined" />
        </UButtonGroup>
        <UTooltip :text="$t('field.tooltip.exportSubmissions')">
            <UButton color="neutral" target="_blank" icon="i-tabler-table-export" @click="exportSubmissions">
                {{ $t("action.exportSubmissions") }}
            </UButton>
        </UTooltip>
        <UPagination v-model:page="page" :total="submissionCount.data.value" :items-per-page="itemsPerPage" />
    </div>
    <UTable :columns="columns" :data="submissions.data.value" v-model:expanded="expanded">
        <template #expanded="{ row }">
            <div v-for="(value, key) in row.original.data">
                <p class="pl-2 -indent-2">
                    <span class="font-bold">{{ key }}:</span> {{ value }}
                </p>
            </div>
        </template>
        <template #form-cell="{ row }">
            <FormLink :id="row.original.form_id" />
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
            <div class="flex flex-row gap-2 items-center">
                <SendEmailModal :submission="row.original" />
                <DeleteModal
                    :name="$t('deleteName.email')"
                    :id="row.original.id"
                    @confirm="deleteMutation.mutate({ id: row.original.id })"
                >
                    <UButton color="error" variant="ghost" icon="i-tabler-trash" />
                </DeleteModal>
            </div>
        </template>
    </UTable>
</template>
