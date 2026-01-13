<script setup lang="ts">
import type { Email } from "../../../bindings";

const model = defineModel<Email>({ required: true });
</script>
<template>
    <div class="grid gap-4">
        <UFormField :label="$t('field.label.subject')" required>
            <UInput v-model="model.subject" class="w-full" />
        </UFormField>
        <UFormField :label="$t('field.label.content')" :help="$t('hint.emailContent')">
            <UTextarea v-model="model.content" class="w-full" autoresize />
        </UFormField>
    </div>
</template>
