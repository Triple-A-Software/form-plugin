<script setup lang="ts">
import { computed, ref } from "vue";
import type { Form } from "../../../bindings";
import { useI18n } from "vue-i18n";

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

const forms = computed<Form[]>(() => [
    {
        id: -1,
        name: "Test-form",
        confirmation_email_id: null,
        confirmation_email_recipient_field_name: null,
        send_confirmation_email: false,
        props: [],
    },
]);

const editingForm = ref<Form | null>(null);
</script>

<template>
    <UTable :columns="columns" :data="forms" v-model:expanded="expanded">
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
            <UModal :title="t('modal.edit.form.title')" fullscreen>
                <UButton color="neutral" variant="ghost" icon="i-tabler-pencil" @click="editingForm = row.original" />
                <template #body v-if="editingForm">
                    <UFormField :label="$t('field.label.formbuilder')">
                        <FormBuilder v-model="editingForm.props" />
                    </UFormField>
                </template>
            </UModal>
            <UButton color="error" variant="ghost" icon="i-tabler-trash" />
        </template>
    </UTable>
</template>
