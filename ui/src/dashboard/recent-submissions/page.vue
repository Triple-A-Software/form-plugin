<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
import type { TableColumn } from "@nuxt/ui";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import type { Submission } from "../../../../bindings";
import { useForms } from "@/composables/useForms";
import { formatDateTime, submissionEntries } from "@/utils";

const { t, locale } = useI18n();

const submissions = useApiQuery("/api/submissions/recent", {
    method: "GET",
});

const columns = computed<TableColumn<Submission>[]>(() => [
    {
        id: "expand",
    },
    {
        accessorKey: "timestamp",
        header: t("table_columns.timestamp"),
        cell: ({ row }) => formatDateTime(row.getValue("timestamp"), locale.value),
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
const forms = useForms();
</script>

<template>
    <UTable :columns="columns" :data="submissions.data.value" v-model:expanded="expanded" :ui="{ td: 'whitespace-normal wrap-anywhere' }">
        <template #expanded="{ row }">
            <dl class="grid grid-cols-[max-content_1fr] gap-x-6 gap-y-1 px-2 text-sm">
                <template
                    v-for="entry in submissionEntries(
                        row.original.data,
                        forms.data.value?.find((f) => f.id === row.original.form_id),
                        locale,
                        { yes: $t('action.yes'), no: $t('action.no') },
                    )"
                    :key="entry.name"
                >
                    <dt class="font-medium">{{ entry.label }}</dt>
                    <dd class="whitespace-pre-wrap wrap-anywhere">{{ entry.value }}</dd>
                </template>
            </dl>
        </template>
        <template #form-cell="{ row }">{{ forms.data.value?.find((f) => f.id === row.original.form_id)?.name }}</template>
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
                    :name="$t('deleteName.submission')"
                    :id="row.original.id"
                    @confirm="deleteMutation.mutate({ ids: [row.original.id] })"
                >
                    <UButton color="error" variant="ghost" icon="i-tabler-trash" :aria-label="$t('action.delete')" />
                </DeleteModal>
            </div>
        </template>
    </UTable>
</template>
