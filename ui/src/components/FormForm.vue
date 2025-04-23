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
                <UCard class="m-1">
                    <div class="grid grid-cols-2 gap-2 w-max">
                        <UFormField :label="$t('field.label.cleverreach_condition')" required>
                            <UButtonGroup>
                                <UButton
                                    @click="model.cleverreach_condition = { type: 'OnSubmit' }"
                                    color="neutral"
                                    :variant="model.cleverreach_condition?.type === 'OnSubmit' ? 'solid' : 'subtle'"
                                >
                                    {{ $t("field.option.cleverreach_condition.on_submit") }}
                                </UButton>
                                <UButton
                                    @click="model.cleverreach_condition = { type: 'WhenFieldIsSet', field: '' }"
                                    color="neutral"
                                    :variant="model.cleverreach_condition?.type === 'WhenFieldIsSet' ? 'solid' : 'subtle'"
                                >
                                    {{ $t("field.option.cleverreach_condition.when_field_is_set") }}
                                </UButton>
                            </UButtonGroup>
                            <UFormField
                                v-if="model.cleverreach_condition?.type === 'WhenFieldIsSet'"
                                :label="$t('field.label.cleverreach_condition_field')"
                            >
                                <UInput v-model="model.cleverreach_condition.field" />
                            </UFormField>
                        </UFormField>
                        <UFormField :label="$t('field.label.cleverreachGroupId')" required class="col-start-1">
                            <UInput v-model="model.cleverreach_group_id" />
                        </UFormField>
                        <UFormField :label="$t('field.label.cleverreachEmailField')" required>
                            <UInput v-model="model.cleverreach_email_field_name" />
                        </UFormField>
                        <UFormField :label="$t('field.label.cleverreachFirstNameField')" class="col-start-1">
                            <UInput v-model="model.cleverreach_firstname_field_name" />
                        </UFormField>
                        <UFormField :label="$t('field.label.cleverreachLastNameField')">
                            <UInput v-model="model.cleverreach_lastname_field_name" />
                        </UFormField>
                        <UFormField :label="$t('field.label.cleverreachSalutation')" required class="col-span-2">
                            <UInput v-model="model.cleverreach_salutation" class="w-full" />
                        </UFormField>
                    </div>
                </UCard>
            </template>
        </UCollapsible>
        <UFormField :label="$t('field.label.formbuilder')">
            <FormBuilder v-model="model.props" />
        </UFormField>
    </div>
</template>
