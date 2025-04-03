<script setup lang="ts">
import { useApiQuery } from "@/utils/api";
import type { Form } from "../../../bindings";
import type { NullToUndefined } from "@/utils/types/utils";

const model = defineModel<NullToUndefined<Form>>({ required: true });

const emails = useApiQuery("/api/emails", {
    method: "GET",
});
</script>
<template>
    <div class="grid gap-4">
        <UFormField :label="$t('field.label.name')" required>
            <UInput v-model="model.name" />
        </UFormField>
        <UCollapsible v-model:open="model.send_confirmation_email">
            <UButton
                color="neutral"
                variant="ghost"
                :icon="model.send_confirmation_email ? 'i-tabler-square-check' : 'i-tabler-square'"
            >
                {{ $t("field.label.sendConfirmationEmail") }}
            </UButton>
            <template #content v-if="model.send_confirmation_email && emails.data.value">
                <UFormField :label="$t('field.label.confirmationEmail')" required>
                    <USelect
                        v-model="model.confirmation_email_id"
                        :items="emails.data.value"
                        value-key="id"
                        :placeholder="$t('field.placeholder.confirmationEmail')"
                    />
                </UFormField>
                <UFormField :label="$t('field.label.confirmationEmailRecipientFieldName')">
                    <UInput v-model="model.confirmation_email_recipient_field_name" />
                </UFormField>
            </template>
        </UCollapsible>
        <UFormField :label="$t('field.label.formbuilder')">
            <FormBuilder v-model="model.props" />
        </UFormField>
    </div>
</template>
