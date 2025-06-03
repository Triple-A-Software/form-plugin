<script setup lang="ts">
import type { NullToUndefined } from "@/utils/types/utils";
import type { Form } from "../../../bindings";
import { useApiQuery } from "@/utils/api";
import { undefinedToNull } from "@/utils/utils";
import { watch } from "vue";

const props = defineProps<{
    form: NullToUndefined<Form>;
}>();

const rendered = useApiQuery("/api/forms/preview", { method: "POST", body: () => undefinedToNull(props.form) });
watch(props.form, () => rendered.refetch());
</script>
<template>
    <UFormField :label="$t('field.label.preview')" :help="$t('hint.preview')">
        <iframe class="w-full rounded-lg" :srcdoc="rendered.data.value?.html" frameborder="0"></iframe>
    </UFormField>
</template>
