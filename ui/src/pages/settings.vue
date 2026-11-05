<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Settings, TestConnectionBody, TestConnectionResponse } from "../../../bindings";
import { useApiMutation, useApiQuery } from "@/utils/api";
import { onBeforeRouteLeave, useRouter, type RouteLocationRaw } from "vue-router";

const { t } = useI18n();

const settings = useApiQuery("/api/settings", {
    method: "GET",
});

// Local, fully reactive copy of the settings for the form. Binding the form directly to the
// pinia-colada query data breaks controlled inputs like USelect: nested mutations on the cached
// data object don't trigger reactivity, so the select snaps back to its previous value. We sync
// this copy from the query and submit it; after saving, the query is invalidated and re-synced.
const form = ref<Settings>();
watch(
    () => settings.data.value,
    (data) => {
        if (data) form.value = { ...data };
    },
    { immediate: true },
);
// a cleared input holds "" where the stored value is unset, which is no change
const normalized = (value: unknown) => JSON.stringify(value, (_, v) => (v === "" ? undefined : v));
const dirty = computed(() => form.value !== undefined && normalized(form.value) !== normalized(settings.data.value));

const updateMutation = useApiMutation(
    "/api/settings",
    {
        method: "PUT",
    },
    {
        invalidate: [settings],
    },
);

const captchaProviderOptions = computed(() => [
    { label: t("field.option.captchaType.hCaptcha"), value: "h-captcha" },
    { label: t("field.option.captchaType.turnstile"), value: "turnstile" },
    { label: t("field.option.captchaType.reCaptcha"), value: "re-captcha" },
]);

// tests what is typed in, so a typo shows up before saving and not with the first submission
const testMutation = useApiMutation("/api/settings/test-connection", { method: "POST" });
const testResults = ref<Partial<Record<TestConnectionBody["type"], TestConnectionResponse & { testing?: boolean }>>>({});
async function test(body: TestConnectionBody) {
    testResults.value[body.type] = { ok: false, error: undefined, testing: true };
    try {
        const result = await testMutation.mutateAsync(body);
        if (result) testResults.value[body.type] = result;
    } catch {
        testResults.value[body.type] = { ok: false, error: t("alert.testRequestFailed") };
    }
}
function testClamav() {
    if (form.value?.clamavAddress) test({ type: "Clamav", address: form.value.clamavAddress });
}
function testCleverreach() {
    const { cleverreachClientId: client_id, cleverreachSecret: client_secret } = form.value ?? {};
    if (client_id && client_secret) test({ type: "Cleverreach", client_id, client_secret });
}
// a result only belongs to the values it was tested with
watch(
    () => form.value?.clamavAddress,
    () => {
        testResults.value.Clamav = undefined;
    },
);
watch(
    () => `${form.value?.cleverreachClientId}\n${form.value?.cleverreachSecret}`,
    () => {
        testResults.value.Cleverreach = undefined;
    },
);

const router = useRouter();
const discardOpen = ref(false);
let pendingRoute: RouteLocationRaw | undefined;
let allowLeave = false;
onBeforeRouteLeave((to) => {
    if (!dirty.value || allowLeave) return true;
    pendingRoute = to.fullPath;
    discardOpen.value = true;
    return false;
});
function discard() {
    allowLeave = true;
    if (pendingRoute) router.push(pendingRoute);
}
</script>
<template>
    <UForm
        :state="form"
        @submit="form && updateMutation.mutate(form)"
        class="flex flex-col gap-6 py-4 max-w-2xl"
        v-if="form"
    >
        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.captcha") }}</h2>
            <UFormField :label="$t('field.label.captchaProvider')" :help="$t('hint.captchaProvider')">
                <USelect
                    v-model="form.captchaProvider"
                    :items="captchaProviderOptions"
                    :placeholder="$t('field.placeholder.selectCaptchaProvider')"
                    class="w-full"
                />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider"
                :label="$t('field.label.captchaSitekey')"
                :help="$t('hint.captchaSitekey')"
            >
                <UInput v-model="form.captchaSitekey" class="w-full" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 'h-captcha'"
                :label="$t('field.label.hCaptchaSecret')"
                :help="$t('hint.hCaptchaSecret')"
            >
                <SecretInput v-model="form.hCaptchaSecret" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 'turnstile'"
                :label="$t('field.label.turnstileSecret')"
                :help="$t('hint.turnstileSecret')"
            >
                <SecretInput v-model="form.turnstileSecret" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 're-captcha'"
                :label="$t('field.label.reCaptchaSecret')"
                :help="$t('hint.reCaptchaSecret')"
            >
                <SecretInput v-model="form.reCaptchaSecret" />
            </UFormField>
        </section>

        <USeparator />

        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.virusScan") }}</h2>
            <UFormField :label="$t('field.label.clamavAddress')" :help="$t('hint.clamavAddress')">
                <UInput v-model="form.clamavAddress" placeholder="clamav:3310" class="w-full" />
            </UFormField>
            <div class="flex flex-wrap items-center gap-3">
                <UButton
                    color="neutral"
                    variant="subtle"
                    icon="i-tabler-plug-connected"
                    :disabled="!form.clamavAddress"
                    :loading="testResults.Clamav?.testing"
                    @click="testClamav"
                >
                    {{ $t("action.testConnection") }}
                </UButton>
                <TestResult :result="testResults.Clamav" />
            </div>
        </section>

        <USeparator />

        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.cleverreach") }}</h2>
            <UFormField :label="$t('field.label.cleverreachClientId')" :help="$t('hint.cleverreachClientId')">
                <UInput type="text" v-model="form.cleverreachClientId" class="w-full" />
            </UFormField>
            <UFormField :label="$t('field.label.cleverreachSecret')" :help="$t('hint.cleverreachSecret')">
                <SecretInput v-model="form.cleverreachSecret" />
            </UFormField>
            <div class="flex flex-wrap items-center gap-3">
                <UButton
                    color="neutral"
                    variant="subtle"
                    icon="i-tabler-plug-connected"
                    :disabled="!form.cleverreachClientId || !form.cleverreachSecret"
                    :loading="testResults.Cleverreach?.testing"
                    @click="testCleverreach"
                >
                    {{ $t("action.testConnection") }}
                </UButton>
                <TestResult :result="testResults.Cleverreach" />
            </div>
        </section>

        <div class="sticky bottom-0 py-3 bg-default/90 backdrop-blur flex items-center gap-3 border-t border-default">
            <UButton icon="i-tabler-device-floppy" type="submit" :loading="updateMutation.isLoading.value" :disabled="!dirty">
                {{ $t("action.save") }}
            </UButton>
            <UBadge v-if="dirty" color="warning" variant="subtle" icon="i-tabler-point-filled">
                {{ $t("hint.unsavedChanges") }}
            </UBadge>
        </div>
    </UForm>
    <DiscardChangesModal v-model:open="discardOpen" @discard="discard" />
</template>
