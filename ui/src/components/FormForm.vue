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
                <UCard class="m-1" :ui="{ body: 'space-y-2' }">
                    <UFormField :label="$t('field.label.confirmationEmail')" required>
                        <USelect
                            v-model="model.confirmation_email_id"
                            :items="emails.data.value"
                            value-key="id"
                            label-key="subject"
                            :placeholder="$t('field.placeholder.confirmationEmail')"
                        />
                    </UFormField>
                    <UFormField :label="$t('field.label.confirmationEmailRecipientFieldName')">
                        <UInput v-model="model.confirmation_email_recipient_field_name" />
                    </UFormField>
                </UCard>
            </template>
        </UCollapsible>
        <UCollapsible v-model:open="model.activate_cleverreach">
            <UButton
                color="neutral"
                variant="ghost"
                :icon="model.activate_cleverreach ? 'i-tabler-square-check' : 'i-tabler-square'"
            >
                {{ $t("field.label.activateCleverreach") }}
            </UButton>
            <template #content v-if="model.activate_cleverreach">
                <UCard class="m-1" :ui="{ body: 'space-y-2' }">
                    <UFormField :label="$t('field.label.cleverreachGroupId')" required>
                        <UInput v-model="model.cleverreach_group_id" />
                    </UFormField>
                    <UFormField :label="$t('field.label.cleverreachEmailField')" required>
                        <UInput v-model="model.cleverreach_email_field_name" />
                    </UFormField>
                    <UFormField :label="$t('field.label.cleverreachFirstNameField')">
                        <UInput v-model="model.cleverreach_first_name_field_name" />
                    </UFormField>
                    <UFormField :label="$t('field.label.cleverreachLastNameField')">
                        <UInput v-model="model.cleverreach_last_name_field_name" />
                    </UFormField>
                    <UFormField :label="$t('field.label.cleverreachSalutation')" required>
                        <UInput v-model="model.cleverreach_salutation" />
                    </UFormField>
                </UCard>
            </template>
        </UCollapsible>
        <UFormField :label="$t('field.label.formbuilder')">
            <FormBuilder v-model="model.props" />
        </UFormField>
    </div>
</template>
