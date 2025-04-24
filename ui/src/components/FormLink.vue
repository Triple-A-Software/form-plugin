<script setup lang="ts">
import { useForms } from "@/store/forms";
import { computed } from "vue";

const props = defineProps<{
    id: number;
}>();

const formStore = useForms();
const form = computed(() => formStore.forms.data.find((f) => f.id === props.id));
</script>
<template>
    <UButton v-if="form" variant="link" color="neutral" :to="`/forms?edit=true&id=${form.id}`">
        {{ form.name }}
    </UButton>
</template>
