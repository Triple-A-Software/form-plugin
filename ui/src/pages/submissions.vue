<script setup lang="ts">
import { fetchApi, useApiQuery } from "@/utils/api";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const submissions = useApiQuery("/api/submissions", {
    method: "GET",
});

const columns = computed(() => [
    {
        id: "expand",
    },
    {
        accessorKey: "subject",
        header: t("table_columns.subject"),
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
