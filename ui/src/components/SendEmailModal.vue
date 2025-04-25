<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";
import { ref } from "vue";
import type { Submission } from "../../../bindings";

const props = defineProps<{
    submission: Submission;
}>();

const emails = useApiQuery("/api/emails", {
    method: "GET",
});
const sendingEmailId = ref<number | undefined>(undefined);
const open = ref(false);
const mutation = useApiMutation("/api/submissions/send-email", {
    method: "POST",
});
const emailFieldName = ref("email");
function sendEmail() {
    if (sendingEmailId.value) {
        mutation.mutate({
            email_id: sendingEmailId.value,
            submission_id: props.submission.id,
            email_field_name: emailFieldName.value,
        });
        open.value = false;
    }
}
</script>
<template>
    <UModal :title="$t('modal.sendEmail.title')" v-model:open="open">
        <UTooltip :text="$t('field.tooltip.sendEmail')">
            <UButton icon="i-tabler-send" variant="ghost" color="neutral" />
        </UTooltip>
        <template #body>
            <UFormField :label="$t('field.label.confirmationEmail')" required>
                <USelect
                    v-model="sendingEmailId"
                    :items="emails.data.value"
                    value-key="id"
                    label-key="subject"
                    :placeholder="$t('field.placeholder.selectEmail')"
                />
            </UFormField>
            <UFormField :label="$t('field.label.emailFieldName')">
                <UInput v-model="emailFieldName" />
            </UFormField>
        </template>
        <template #footer>
            <UButton :disabled="sendingEmailId == null" icon="i-tabler-send" @click="sendEmail">
                {{ $t("action.sendEmail") }}
            </UButton>
        </template>
    </UModal>
</template>
