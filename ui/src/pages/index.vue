<script setup lang="ts">
import { ref } from "vue";

const builder = ref([]);
</script>
<template>
    <UFormField :label="$t('field.label.formbuilder')">
        <FormBuilder v-model="builder" />
    </UFormField>
</template>
