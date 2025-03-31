<template>
    <UCard
        class="hover:ring-primary-500 hover:dark:ring-primary-400 hover:ring-2 transition-all duration-150 select-none text-center"
        @click="emit('click', data)"
    >
        <div :class="`!w-10 !h-10 ${data.icon ?? fallbackIcon}`" />
        <template #footer>
            <p>
                <slot name="label">
                    {{ "form" in data ? data.form.label[locale] : data.name }}
                </slot>
            </p>
        </template>
    </UCard>
</template>
<script setup lang="ts">
import { useI18n } from "vue-i18n";

type Data = {
    icon?: string | null | undefined;
} & (
    | {
          form: {
              label: Record<string, string>;
          };
      }
    | {
          name: string;
      }
);
const { locale } = useI18n();
defineProps<{
    data: Data;
    fallbackIcon?: string;
}>();
const emit = defineEmits<{
    (e: "click", data: Data): void;
}>();
</script>
