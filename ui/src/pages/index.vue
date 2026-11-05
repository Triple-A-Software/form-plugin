<script setup lang="ts">
import { useApiQuery } from "@/utils/api";
import { useForms } from "@/composables/useForms";
import { formatRelative, submissionEntries } from "@/utils";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

const { t, locale } = useI18n();

const forms = useForms();
const emails = useApiQuery("/api/emails", { method: "GET" });
const settings = useApiQuery("/api/settings", { method: "GET" });
const stats = useApiQuery("/api/submissions/stats", { method: "GET" });
const recent = useApiQuery("/api/submissions/recent", { method: "GET" });

const lastWeek = computed(() => {
    const since = Date.now() - 7 * 24 * 60 * 60 * 1000;
    return (stats.data.value?.graph ?? [])
        .filter((day) => new Date(day.date).getTime() >= since)
        .reduce((sum, { date: _, ...perForm }) => sum + Object.values(perForm).reduce((a, b) => a + b, 0), 0);
});
const kpis = computed(() => [
    { label: t("pages.forms.title"), value: forms.data.value?.length ?? 0, icon: "i-tabler-forms", to: "/forms" },
    {
        label: t("pages.home.totalSubmissions"),
        value: stats.data.value?.total ?? 0,
        icon: "i-tabler-inbox",
        to: "/submissions",
    },
    { label: t("pages.home.lastWeek"), value: lastWeek.value, icon: "i-tabler-calendar-week", to: "/submissions" },
]);

const captchaConfigured = computed(() => {
    const s = settings.data.value;
    if (!s?.captchaProvider || !s.captchaSitekey) return false;
    if (s.captchaProvider === "h-captcha") return Boolean(s.hCaptchaSecret);
    if (s.captchaProvider === "turnstile") return Boolean(s.turnstileSecret);
    return Boolean(s.reCaptchaSecret);
});
// steps only appear once they are relevant, e.g. virus scanning when a form accepts uploads
const checklist = computed(() => {
    const all = forms.data.value ?? [];
    const steps = [
        { done: all.length > 0, label: t("checklist.createForm"), to: "/forms?create=true" },
        { done: (emails.data.value?.length ?? 0) > 0, label: t("checklist.createEmail"), to: "/emails" },
        {
            done: all.some((f) => f.send_submission_as_email_enabled),
            label: t("checklist.notify"),
            to: "/forms",
        },
        { done: captchaConfigured.value, label: t("checklist.captcha"), to: "/settings" },
    ];
    if (all.some((f) => f.props.some((p) => p.type === "file"))) {
        steps.push({ done: Boolean(settings.data.value?.clamavAddress), label: t("checklist.virusScan"), to: "/settings" });
    }
    if (all.some((f) => f.activate_cleverreach)) {
        steps.push({
            done: Boolean(settings.data.value?.cleverreachClientId && settings.data.value?.cleverreachSecret),
            label: t("checklist.cleverreach"),
            to: "/settings",
        });
    }
    return steps;
});
const checklistLoaded = computed(() => forms.data.value && emails.data.value && settings.data.value);
const openSteps = computed(() => checklist.value.filter((s) => !s.done).length);

const formsById = computed(() => new Map((forms.data.value ?? []).map((f) => [f.id, f])));
const latest = computed(() =>
    (recent.data.value ?? []).slice(0, 5).map((submission) => ({
        submission,
        summary: submissionEntries(submission.data, formsById.value.get(submission.form_id), locale.value, {
            yes: t("action.yes"),
            no: t("action.no"),
        })
            .filter((e) => e.value !== "–")
            .slice(0, 2)
            .map((e) => e.value)
            .join(" · "),
    })),
);
</script>
<template>
    <div class="grid gap-6 py-4">
        <div class="grid sm:grid-cols-3 gap-4">
            <UCard v-for="kpi in kpis" :key="kpi.label" :ui="{ body: 'flex items-center gap-4' }">
                <UIcon :name="kpi.icon" class="size-8 text-primary shrink-0" />
                <div class="min-w-0">
                    <p class="text-sm text-muted">{{ kpi.label }}</p>
                    <RouterLink :to="kpi.to" class="text-2xl font-semibold tabular-nums hover:underline">
                        {{ kpi.value }}
                    </RouterLink>
                </div>
            </UCard>
        </div>

        <UCard v-if="checklistLoaded && openSteps > 0">
            <template #header>
                <h2 class="font-semibold">{{ $t("checklist.title") }}</h2>
                <p class="text-sm text-muted">
                    {{ $t("checklist.progress", { done: checklist.length - openSteps, total: checklist.length }) }}
                </p>
            </template>
            <ul class="grid gap-2">
                <li v-for="step in checklist" :key="step.label" class="flex items-center gap-2">
                    <UIcon
                        :name="step.done ? 'i-tabler-circle-check-filled' : 'i-tabler-circle-dashed'"
                        :class="step.done ? 'text-success' : 'text-muted'"
                        class="size-5 shrink-0"
                    />
                    <span :class="step.done ? 'text-muted line-through' : ''">{{ step.label }}</span>
                    <UButton v-if="!step.done" :to="step.to" size="xs" variant="link" trailing-icon="i-tabler-arrow-right">
                        {{ $t("checklist.go") }}
                    </UButton>
                </li>
            </ul>
        </UCard>

        <UCard>
            <template #header>
                <div class="flex items-center justify-between">
                    <h2 class="font-semibold">{{ $t("pages.home.recentSubmissions") }}</h2>
                    <UButton to="/submissions" size="sm" variant="link" trailing-icon="i-tabler-arrow-right">
                        {{ $t("action.goto.submissions") }}
                    </UButton>
                </div>
            </template>
            <ul v-if="latest.length" class="divide-y divide-default">
                <li v-for="{ submission, summary } in latest" :key="submission.id" class="py-2 flex flex-wrap items-center gap-x-4">
                    <span class="text-sm text-muted w-32 shrink-0">{{ formatRelative(submission.timestamp, locale) }}</span>
                    <FormLink :id="submission.form_id" />
                    <span class="text-sm text-muted truncate min-w-0 flex-1">{{ summary }}</span>
                </li>
            </ul>
            <p v-else class="text-muted text-sm">{{ $t("empty.submissions") }}</p>
        </UCard>
    </div>
</template>
