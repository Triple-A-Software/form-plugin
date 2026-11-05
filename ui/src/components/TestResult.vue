<script setup lang="ts">
import type { TestConnectionResponse } from "../../../bindings";

defineProps<{ result?: TestConnectionResponse & { testing?: boolean } }>();
</script>
<template>
    <p v-if="result && !result.testing" role="status" class="text-sm flex items-start gap-1" :class="result.ok ? 'text-success' : 'text-error'">
        <UIcon :name="result.ok ? 'i-tabler-circle-check' : 'i-tabler-alert-circle'" class="size-4 shrink-0 mt-0.5" />
        <span class="wrap-anywhere">{{ result.ok ? $t("alert.connectionOk") : result.error }}</span>
    </p>
</template>
