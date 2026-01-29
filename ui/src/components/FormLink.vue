<script setup lang="ts">
import { useForms } from "@/composables/useForms";
import { computed } from "vue";

const props = defineProps<{
    id: number;
}>();

const forms = useForms();
const form = computed(() => forms.data.value?.find((f) => f.id === props.id));
</script>
<template>
    <UButton v-if="form" variant="link" color="neutral" :to="`/forms?edit=true&id=${form.id}`">{{ form.name }}</UButton>
</template>
