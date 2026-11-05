<script setup lang="ts">
import type { Email } from "../../../bindings";
import { useApiQuery } from "@/utils/api";
import { useForms } from "@/composables/useForms";
import { dataFields } from "@/utils";
import { computed, nextTick, ref, useTemplateRef } from "vue";
import { useI18n } from "vue-i18n";
import { refDebounced } from "@vueuse/core";

const model = defineModel<Email>({ required: true });
defineProps<{ subjectError?: string }>();

const { locale } = useI18n();
const forms = useForms();
const usedBy = computed(() =>
    (forms.data.value ?? []).filter(
        (f) =>
            (f.send_confirmation_email && f.confirmation_email_id === model.value.id) ||
            (f.send_submission_as_email_enabled && f.submissions_email_id === model.value.id),
    ),
);
// offer the fields of the forms sending this email, or of all forms for a new email
const variables = computed(() => {
    const source = usedBy.value.length ? usedBy.value : (forms.data.value ?? []);
    const byName = new Map<string, string>();
    for (const form of source) {
        for (const field of dataFields(form.props)) {
            if (field.name && !byName.has(field.name)) byName.set(field.name, field.label[locale.value] || field.name);
        }
    }
    return [...byName].map(([name, label]) => ({ name, label }));
});

const textarea = useTemplateRef<{ textareaRef: HTMLTextAreaElement | undefined }>("textarea");
async function insertVariable(name: string) {
    const snippet = `{{ submission.${name} }}`;
    const element = textarea.value?.textareaRef;
    const content = model.value.content;
    const start = element?.selectionStart ?? content.length;
    const end = element?.selectionEnd ?? content.length;
    model.value.content = content.slice(0, start) + snippet + content.slice(end);
    await nextTick();
    element?.focus();
    element?.setSelectionRange(start + snippet.length, start + snippet.length);
}

const debouncedContent = refDebounced(
    computed(() => model.value.content),
    400,
);
const preview = useApiQuery(
    "/api/emails/preview",
    {
        method: "POST",
        body: () => ({
            content: debouncedContent.value,
            // placeholders that show which value ends up where
            submission: Object.fromEntries(variables.value.map((v) => [v.name, `[${v.label}]`])),
            form_id: usedBy.value[0]?.id,
        }),
    },
    { placeholderData: (previous) => previous },
);

const iframe = ref<HTMLIFrameElement>();
const height = ref(200);
function onLoad() {
    const doc = iframe.value?.contentDocument;
    if (doc) height.value = Math.max(200, doc.documentElement.scrollHeight + 2);
}
</script>
<template>
    <div class="grid lg:grid-cols-2 gap-6 items-start">
        <div class="grid gap-4 min-w-0">
            <UFormField :label="$t('field.label.subject')" required :error="subjectError">
                <UInput v-model="model.subject" class="w-full" />
            </UFormField>
            <p v-if="usedBy.length" class="text-sm text-muted">
                {{ $t("table_columns.usedBy") }}: {{ usedBy.map((f) => f.name).join(", ") }}
            </p>
            <UFormField :label="$t('field.label.content')" :help="$t('hint.emailContent')">
                <UTextarea ref="textarea" v-model="model.content" class="w-full font-mono" autoresize :rows="12" />
            </UFormField>
            <div v-if="variables.length" class="space-y-2">
                <p class="text-sm font-medium">{{ $t("field.label.insertVariable") }}</p>
                <div class="flex flex-wrap gap-1">
                    <UButton
                        v-for="variable in variables"
                        :key="variable.name"
                        size="xs"
                        color="neutral"
                        variant="subtle"
                        icon="i-tabler-braces"
                        @click="insertVariable(variable.name)"
                    >
                        {{ variable.label }}
                    </UButton>
                </div>
            </div>
        </div>
        <div class="lg:sticky lg:top-0 min-w-0 space-y-2">
            <p class="text-sm font-medium">{{ $t("field.label.preview") }}</p>
            <UAlert
                v-if="preview.data.value?.error"
                color="error"
                variant="subtle"
                icon="i-tabler-alert-circle"
                :title="$t('alert.templateError')"
                :description="preview.data.value.error"
                :ui="{ description: 'font-mono whitespace-pre-wrap' }"
            />
            <!-- no scripts, same origin only so the height can be measured -->
            <iframe
                v-else
                ref="iframe"
                sandbox="allow-same-origin"
                class="w-full rounded-lg ring-1 ring-default bg-white"
                :style="{ height: `${height}px` }"
                :srcdoc="preview.data.value?.html"
                :title="$t('field.label.preview')"
                @load="onLoad"
            ></iframe>
        </div>
    </div>
</template>
