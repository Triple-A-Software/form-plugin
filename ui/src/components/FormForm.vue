<script setup lang="ts">
import { useApiQuery } from "@/utils/api";
import type { Form } from "../../../bindings";
import { computed, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { TabsItem } from "@nuxt/ui";
import { createElement, findEmailField, type FormIssue } from "@/utils";

const model = defineModel<Form>({ required: true });
const props = withDefaults(defineProps<{ issues?: FormIssue[] }>(), { issues: () => [] });

const { t } = useI18n();
function errorFor(path: string) {
    const issue = props.issues.find((i) => i.path === path);
    return issue ? t(issue.key, issue.params ?? {}) : undefined;
}
const fieldErrors = computed(() => {
    const errors: Record<number, string> = {};
    for (const issue of props.issues) {
        const match = /^props\.(\d+)$/.exec(issue.path);
        if (match) errors[Number(match[1])] = t(issue.key, issue.params ?? {});
    }
    return errors;
});

type Tab = "fields" | "notifications" | "integrations" | "advanced";
function tabOf(path: string): Tab | undefined {
    if (path === "props" || path.startsWith("props.")) return "fields";
    if (path.startsWith("confirmation_email") || path.startsWith("send_submission") || path === "submissions_email_id") {
        return "notifications";
    }
    if (path.startsWith("cleverreach") || path === "activate_captcha") return "integrations";
}
const tabs = computed<TabsItem[]>(() =>
    (
        [
            { value: "fields", label: t("tab.fields"), icon: "i-tabler-forms" },
            { value: "notifications", label: t("tab.notifications"), icon: "i-tabler-bell" },
            { value: "integrations", label: t("tab.integrations"), icon: "i-tabler-plug" },
            { value: "advanced", label: t("tab.advanced"), icon: "i-tabler-code" },
        ] as const
    ).map((tab) => {
        const count = props.issues.filter((i) => tabOf(i.path) === tab.value).length;
        return { ...tab, slot: tab.value, badge: count ? { label: String(count), color: "error" as const } : undefined };
    }),
);

// the recipient is almost always the email input, so preselect it
watch(
    () => model.value.send_confirmation_email,
    (enabled) => {
        if (!enabled || model.value.confirmation_email_recipient_field_name) return;
        const emailField = findEmailField(model.value.props);
        if (emailField?.name) model.value.confirmation_email_recipient_field_name = emailField.name;
    },
);

// activate_captcha makes the server verify the captcha, the element renders the widget. One without the
// other either rejects every submission or shows an unchecked captcha, so the switch manages both.
const hasCaptchaElement = computed(() => model.value.props.some((p) => p.type === "captcha"));
function setCaptcha(enabled: boolean) {
    model.value.activate_captcha = enabled;
    if (enabled && !hasCaptchaElement.value) {
        const captcha = createElement("captcha");
        if (!captcha) return;
        // place it above the submit button, which is usually the last button
        const props = model.value.props;
        const buttonIndex = props.map((p) => p.type).lastIndexOf("button");
        props.splice(buttonIndex === -1 ? props.length : buttonIndex, 0, captcha);
    } else if (!enabled) {
        model.value.props = model.value.props.filter((p) => p.type !== "captcha");
    }
}
// adding or deleting the element in the builder toggles the verification along with it
watch(hasCaptchaElement, (has) => {
    model.value.activate_captcha = has;
});

const settings = useApiQuery("/api/settings", {
    method: "GET",
});
const showCaptchaAlert = computed(() => {
    if (!settings.data.value) return false;
    const provider = settings.data.value.captchaProvider;
    if (!provider) return true;
    if (!settings.data.value.captchaSitekey) return true;
    if (provider === "h-captcha" && !settings.data.value.hCaptchaSecret) return true;
    if (provider === "turnstile" && !settings.data.value.turnstileSecret) return true;
    if (provider === "re-captcha" && !settings.data.value.reCaptchaSecret) return true;
    return false;
});
const showCleverreachAlert = computed(
    () => settings.data.value && (!settings.data.value.cleverreachClientId || !settings.data.value.cleverreachSecret),
);

const emails = useApiQuery("/api/emails", {
    method: "GET",
});
const noEmails = computed(() => emails.data.value?.length === 0);
</script>

<template>
    <div class="grid gap-4">
        <UFormField :label="$t('field.label.name')" required :error="errorFor('name')">
            <UInput v-model="model.name" class="w-full" />
        </UFormField>

        <UTabs :items="tabs" variant="link" :unmount-on-hide="false" class="w-full">
            <template #fields>
                <UFormField :error="errorFor('props')" class="pt-2">
                    <FormBuilder v-model="model.props" :errors="fieldErrors" />
                </UFormField>
            </template>

            <template #notifications>
                <div class="grid gap-4 pt-2">
                    <UAlert
                        v-if="noEmails"
                        color="warning"
                        variant="subtle"
                        icon="i-tabler-mail-off"
                        :title="$t('alert.noEmails.title')"
                        :description="$t('alert.noEmails.description')"
                        :actions="[{ label: $t('action.create.email'), icon: 'i-tabler-plus', to: '/emails' }]"
                    />

                    <USwitch
                        v-model="model.send_confirmation_email"
                        :label="$t('field.label.sendConfirmationEmail')"
                        :description="$t('hint.sendConfirmationEmail')"
                    />
                    <UCard v-if="model.send_confirmation_email" :ui="{ body: 'space-y-2' }">
                        <UFormField
                            :label="$t('field.label.confirmationEmail')"
                            required
                            :error="errorFor('confirmation_email_id')"
                        >
                            <USelect
                                v-model="model.confirmation_email_id"
                                :items="emails.data.value"
                                value-key="id"
                                label-key="subject"
                                :placeholder="$t('field.placeholder.confirmationEmail')"
                                class="w-full"
                            />
                        </UFormField>
                        <UFormField
                            :label="$t('field.label.confirmationEmailRecipientFieldName')"
                            required
                            :error="errorFor('confirmation_email_recipient_field_name')"
                        >
                            <FieldSelect v-model="model.confirmation_email_recipient_field_name" :fields="model.props" />
                        </UFormField>
                    </UCard>

                    <USwitch
                        v-model="model.send_submission_as_email_enabled"
                        :label="$t('field.label.sendSubmissionAsEmail')"
                        :description="$t('hint.sendSubmissionAsEmailSwitch')"
                    />
                    <UCard v-if="model.send_submission_as_email_enabled" :ui="{ body: 'space-y-2' }">
                        <UFormField
                            :label="$t('field.label.submissionEmail')"
                            required
                            :help="$t('hint.submissionEmail')"
                            :error="errorFor('submissions_email_id')"
                        >
                            <USelect
                                v-model="model.submissions_email_id"
                                :items="emails.data.value"
                                value-key="id"
                                label-key="subject"
                                :placeholder="$t('field.label.submissionEmail')"
                                class="w-full"
                            />
                        </UFormField>
                        <UFormField
                            :label="$t('field.label.submissionEmailRecipient')"
                            required
                            :help="$t('hint.submissionEmailRecipient')"
                            :error="errorFor('send_submission_as_email')"
                        >
                            <UInput v-model="model.send_submission_as_email" type="email" class="w-full" />
                        </UFormField>
                        <USwitch
                            v-model="model.email_attach_files"
                            :label="$t('field.label.emailAttachFiles')"
                            :description="$t('hint.emailAttachFiles')"
                        />
                    </UCard>
                </div>
            </template>

            <template #integrations>
                <div class="grid gap-4 pt-2">
                    <UFormField :error="errorFor('activate_captcha')">
                        <USwitch
                            :model-value="model.activate_captcha"
                            :label="$t('field.label.activateCaptcha')"
                            :description="$t('hint.activateCaptcha')"
                            @update:model-value="setCaptcha"
                        />
                    </UFormField>
                    <UCard v-if="model.activate_captcha" :ui="{ body: 'space-y-2' }">
                        <UAlert
                            v-if="showCaptchaAlert"
                            color="error"
                            :title="$t('alert.missingCaptchaSettings.title')"
                            :description="$t('alert.missingCaptchaSettings.description')"
                            variant="subtle"
                            :actions="[
                                {
                                    label: $t('action.goto.settings'),
                                    icon: 'i-tabler-settings',
                                    to: '/settings',
                                },
                            ]"
                        />
                        <UFormField :label="$t('field.label.captchaCallback')" :help="$t('hint.captchaCallback')">
                            <UInput v-model="model.captcha_callback" class="w-full" />
                        </UFormField>
                        <UFormField :label="$t('field.label.captchaExpiredCallback')">
                            <UInput v-model="model.captcha_expired_callback" class="w-full" />
                        </UFormField>
                    </UCard>

                    <USwitch
                        v-model="model.activate_cleverreach"
                        :label="$t('field.label.activateCleverreach')"
                        :description="$t('hint.activateCleverreach')"
                    />
                    <UCard v-if="model.activate_cleverreach">
                        <UAlert
                            v-if="showCleverreachAlert"
                            class="mb-4"
                            color="error"
                            variant="subtle"
                            :title="$t('alert.missingCleverreachSettings.title')"
                            :description="$t('alert.missingCleverreachSettings.description')"
                            :actions="[{ label: $t('action.goto.settings'), icon: 'i-tabler-settings', to: '/settings' }]"
                        />
                        <div class="grid sm:grid-cols-2 gap-4">
                            <UFormField
                                :label="$t('field.label.cleverreach_condition')"
                                :help="$t('hint.cleverreach_condition')"
                                required
                                class="sm:col-span-2"
                            >
                                <UFieldGroup>
                                    <UButton
                                        @click="model.cleverreach_condition = { type: 'OnSubmit' };"
                                        color="neutral"
                                        :variant="model.cleverreach_condition?.type === 'OnSubmit' ? 'solid' : 'subtle'"
                                    >
                                        {{ $t("field.option.cleverreach_condition.on_submit") }}
                                    </UButton>
                                    <UButton
                                        @click="model.cleverreach_condition = { type: 'WhenFieldIsSet', field: '' };"
                                        color="neutral"
                                        :variant="model.cleverreach_condition?.type === 'WhenFieldIsSet' ? 'solid' : 'subtle'"
                                    >
                                        {{ $t("field.option.cleverreach_condition.when_field_is_set") }}
                                    </UButton>
                                </UFieldGroup>
                            </UFormField>
                            <UFormField
                                v-if="model.cleverreach_condition?.type === 'WhenFieldIsSet'"
                                :label="$t('field.label.cleverreach_condition_field')"
                                :error="errorFor('cleverreach_condition')"
                                class="sm:col-span-2"
                            >
                                <FieldSelect v-model="model.cleverreach_condition.field" :fields="model.props" />
                            </UFormField>
                            <UFormField
                                :label="$t('field.label.cleverreachGroupId')"
                                required
                                :error="errorFor('cleverreach_group_id')"
                            >
                                <UInput v-model="model.cleverreach_group_id" class="w-full" />
                            </UFormField>
                            <UFormField
                                :label="$t('field.label.cleverreachEmailField')"
                                required
                                :error="errorFor('cleverreach_email_field_name')"
                            >
                                <FieldSelect v-model="model.cleverreach_email_field_name" :fields="model.props" />
                            </UFormField>
                            <UFormField
                                :label="$t('field.label.cleverreachFirstNameField')"
                                :error="errorFor('cleverreach_firstname_field_name')"
                            >
                                <FieldSelect v-model="model.cleverreach_firstname_field_name" :fields="model.props" clearable />
                            </UFormField>
                            <UFormField
                                :label="$t('field.label.cleverreachLastNameField')"
                                :error="errorFor('cleverreach_lastname_field_name')"
                            >
                                <FieldSelect v-model="model.cleverreach_lastname_field_name" :fields="model.props" clearable />
                            </UFormField>
                            <UFormField
                                :label="$t('field.label.cleverreachSalutation')"
                                required
                                class="sm:col-span-2"
                                :error="errorFor('cleverreach_salutation')"
                            >
                                <UInput v-model="model.cleverreach_salutation" class="w-full" />
                            </UFormField>
                        </div>
                    </UCard>
                </div>
            </template>

            <template #advanced>
                <div class="pt-2">
                    <UFormField :label="$t('field.label.additionalHtml')" :help="$t('hint.additionalHtml')">
                        <UTextarea v-model="model.additional_html" class="w-full font-mono" autoresize :rows="6" />
                    </UFormField>
                </div>
            </template>
        </UTabs>
    </div>
</template>
