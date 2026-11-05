<script setup lang="ts">
import { ref } from "vue";

const model = defineModel<string | undefined>();
const visible = ref(false);
</script>
<template>
    <UInput v-model="model" :type="visible ? 'text' : 'password'" autocomplete="off" class="w-full" :ui="{ trailing: 'pe-1' }">
        <template #trailing>
            <UButton
                color="neutral"
                variant="link"
                size="sm"
                :icon="visible ? 'i-tabler-eye-off' : 'i-tabler-eye'"
                :aria-label="visible ? $t('action.hide') : $t('action.show')"
                :aria-pressed="visible"
                @click="visible = !visible;"
            />
        </template>
    </UInput>
</template>
