<script setup lang="ts">
import { fetchApi, useApiQuery } from "@/utils/api";
import type { TableColumn } from "@nuxt/ui";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import type { Submission } from "../../../bindings";

const { t, locale } = useI18n();

const submissions = useApiQuery("/api/submissions", {
    method: "GET",
});

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
        accessorKey: "form_id",
        header: t("table_columns.form_id"),
    },
    {
        id: "actions",
    },
]);
const expanded = ref({});

async function deleteSubmission(id: number) {
    await fetchApi("/api/submissions", {
        method: "DELETE",
        query: {
            id,
        },
    });
    submissions.invalidate();
}
</script>

<template>
    <UTable :columns="columns" :data="submissions.data.value" v-model:expanded="expanded">
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
            <DeleteModal :name="$t('deleteName.email')" :id="row.original.id" @confirm="deleteSubmission">
                <UButton color="error" variant="ghost" icon="i-tabler-trash" />
            </DeleteModal>
        </template>
    </UTable>
</template>
