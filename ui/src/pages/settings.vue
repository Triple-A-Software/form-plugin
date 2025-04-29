<script setup lang="ts">
import { useApiMutation, useApiQuery } from "@/utils/api";

const settings = useApiQuery("/api/settings", {
    method: "GET",
});

const updateMutation = useApiMutation(
    "/api/settings",
    {
        method: "PUT",
    },
    {
        invalidate: [settings],
    },
);
</script>
<template>
    <UForm
        :state="settings.data.value"
        @submit="updateMutation.mutate(settings.data.value)"
        class="flex flex-col gap-2"
        v-if="settings.data.value"
    >
        <h2 class="font-bold text-xl">{{ $t("pages.settings.cleverreach") }}</h2>
        <UFormField :label="$t('field.label.cleverreachClientId')" :help="$t('hint.cleverreachClientId')">
            <UInput type="text" v-model="settings.data.value.cleverreachClientId" />
        </UFormField>
        <UFormField :label="$t('field.label.cleverreachSecret')" :help="$t('hint.cleverreachSecret')">
            <UInput type="password" v-model="settings.data.value.cleverreachSecret" />
        </UFormField>
        <UFormField :label="$t('field.label.hCaptchaSecret')" :help="$t('hint.hCaptchaSecret')">
            <UInput type="password" v-model="settings.data.value.hCaptchaSecret" />
        </UFormField>
        <UFormField :label="$t('field.label.reCaptchaSecret')" :help="$t('hint.reCaptchaSecret')">
            <UInput type="password" v-model="settings.data.value.reCaptchaSecret" />
        </UFormField>
        <UFormField :label="$t('field.label.captchaSitekey')" :help="$t('hint.captchaSitekey')">
            <UInput v-model="settings.data.value.captchaSitekey" />
        </UFormField>
        <div>
            <UButton icon="i-tabler-device-floppy" type="submit">{{ $t("action.save") }}</UButton>
        </div>
    </UForm>
</template>
