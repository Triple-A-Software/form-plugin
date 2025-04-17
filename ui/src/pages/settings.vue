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
        <div>
            <UButton icon="i-tabler-device-floppy" type="submit">{{ $t("action.save") }}</UButton>
        </div>
    </UForm>
</template>
