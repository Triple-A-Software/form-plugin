<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { Settings } from "../../../bindings";
import { useApiMutation, useApiQuery } from "@/utils/api";

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
</script>
<template>
    <UForm
        :state="form"
        @submit="form && updateMutation.mutate(form)"
        class="flex flex-col gap-6"
        v-if="form"
    >
        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.cleverreach") }}</h2>
            <UFormField :label="$t('field.label.cleverreachClientId')" :help="$t('hint.cleverreachClientId')">
                <UInput type="text" v-model="form.cleverreachClientId" />
            </UFormField>
            <UFormField :label="$t('field.label.cleverreachSecret')" :help="$t('hint.cleverreachSecret')">
                <UInput type="password" v-model="form.cleverreachSecret" />
            </UFormField>
        </section>

        <USeparator />

        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.captcha") }}</h2>
            <UFormField :label="$t('field.label.captchaProvider')" :help="$t('hint.captchaProvider')">
                <USelect
                    v-model="form.captchaProvider"
                    :items="captchaProviderOptions"
                    class="w-full"
                />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 'h-captcha'"
                :label="$t('field.label.hCaptchaSecret')"
                :help="$t('hint.hCaptchaSecret')"
            >
                <UInput type="password" v-model="form.hCaptchaSecret" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 'turnstile'"
                :label="$t('field.label.turnstileSecret')"
                :help="$t('hint.turnstileSecret')"
            >
                <UInput type="password" v-model="form.turnstileSecret" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider === 're-captcha'"
                :label="$t('field.label.reCaptchaSecret')"
                :help="$t('hint.reCaptchaSecret')"
            >
                <UInput type="password" v-model="form.reCaptchaSecret" />
            </UFormField>
            <UFormField
                v-if="form.captchaProvider"
                :label="$t('field.label.captchaSitekey')"
                :help="$t('hint.captchaSitekey')"
            >
                <UInput v-model="form.captchaSitekey" />
            </UFormField>
        </section>

        <USeparator />

        <section class="flex flex-col gap-2">
            <h2 class="font-bold text-xl">{{ $t("pages.settings.virusScan") }}</h2>
            <UFormField :label="$t('field.label.clamavAddress')" :help="$t('hint.clamavAddress')">
                <UInput v-model="form.clamavAddress" placeholder="clamav:3310" />
            </UFormField>
        </section>

        <div>
            <UButton icon="i-tabler-device-floppy" type="submit">{{ $t("action.save") }}</UButton>
        </div>
    </UForm>
</template>
