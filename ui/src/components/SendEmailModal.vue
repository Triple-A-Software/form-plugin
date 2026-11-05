<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
import { useForms } from "@/composables/useForms";
import { findEmailField } from "@/utils";
import { computed, ref, watch } from "vue";
import type { Submission } from "../../../bindings";

const props = defineProps<{
    submission: Submission;
}>();

const emails = useApiQuery("/api/emails", {
    method: "GET",
});
const forms = useForms();
const form = computed(() => forms.data.value?.find((f) => f.id === props.submission.form_id));

const sendingEmailId = ref<number | undefined>(undefined);
const open = ref(false);
const mutation = useApiMutation("/api/submissions/send-email", {
    method: "POST",
});

const defaultFieldName = computed(
    () =>
        form.value?.confirmation_email_recipient_field_name ??
        (form.value && findEmailField(form.value.props)?.name) ??
        "email",
);
const emailFieldName = ref<string | undefined>(defaultFieldName.value);
watch(open, (isOpen) => {
    if (isOpen) emailFieldName.value = defaultFieldName.value;
});
const recipient = computed(() => {
    const data = props.submission.data;
    if (!emailFieldName.value || !data || typeof data !== "object" || Array.isArray(data)) return undefined;
    const value = data[emailFieldName.value];
    return typeof value === "string" ? value : undefined;
});

async function sendEmail() {
    if (!sendingEmailId.value) return;
    try {
        await mutation.mutateAsync({
            email_id: sendingEmailId.value,
            submission_id: props.submission.id,
            email_field_name: emailFieldName.value,
        });
        open.value = false;
    } catch {
        // the api toolkit already shows an error toast
    }
}
</script>
<template>
    <UModal :title="$t('modal.sendEmail.title')" v-model:open="open" :ui="{ body: 'space-y-4' }">
        <UTooltip :text="$t('field.tooltip.sendEmail')">
            <UButton icon="i-tabler-send" variant="ghost" color="neutral" :aria-label="$t('field.tooltip.sendEmail')" />
        </UTooltip>
        <template #body>
            <UFormField :label="$t('field.label.confirmationEmail')" required>
                <USelect
                    v-model="sendingEmailId"
                    :items="emails.data.value"
                    value-key="id"
                    label-key="subject"
                    :placeholder="$t('field.placeholder.selectEmail')"
                    class="w-full"
                />
            </UFormField>
            <UFormField
                :label="$t('field.label.emailFieldName')"
                :help="recipient ? $t('hint.sendEmailRecipient', { recipient }) : undefined"
                :error="emailFieldName && !recipient ? $t('validation.noRecipient') : undefined"
            >
                <FieldSelect v-if="form" v-model="emailFieldName" :fields="form.props" />
                <UInput v-else v-model="emailFieldName" class="w-full" />
            </UFormField>
        </template>
        <template #footer>
            <UButton
                :disabled="sendingEmailId == null || !recipient"
                :loading="mutation.isLoading.value"
                icon="i-tabler-send"
                @click="sendEmail"
            >
                {{ $t("action.sendEmail") }}
            </UButton>
        </template>
    </UModal>
</template>
