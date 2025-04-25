<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
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
        id: "form",
        header: t("table_columns.form"),
    },
    {
        id: "actions",
    },
]);
const expanded = ref({});

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
    <UTable :columns="columns" :data="submissions.data.value" v-model:expanded="expanded">
        <template #expanded="{ row }">
            <pre>{{ row.original }}</pre>
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
