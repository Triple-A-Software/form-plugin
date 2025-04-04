<script setup lang="ts">
import type { NullToUndefined } from "@/utils/types/utils";
import type { Email } from "../../../bindings";

const model = defineModel<NullToUndefined<Email>>({ required: true });
</script>
<template>
    <div class="grid gap-4">
        <UFormField :label="$t('field.label.subject')" required>
            <UInput v-model="model.subject" class="w-full" />
        </UFormField>
        <UFormField :label="$t('field.label.content')">
            <UTextarea v-model="model.content" class="w-full" />
        </UFormField>
    </div>
</template>
