<script setup lang="ts">
import { dataFields } from "@/utils";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { Property } from "../../../../bindings";

const model = defineModel<string | undefined>();
const props = defineProps<{
    fields: Property[];
    clearable?: boolean;
}>();

const { t, locale } = useI18n();
const items = computed(() => {
    const options = dataFields(props.fields)
        .filter((field) => field.name)
        .map((field) => ({
            label: field.label[locale.value] || field.name,
            description: `${field.name} · ${t(`builder.item.label.${field.type}`)}`,
            value: field.name,
        }));
    // keep a reference to a renamed or deleted field visible, instead of showing an empty select
    if (model.value && !options.some((o) => o.value === model.value)) {
        options.unshift({ label: model.value, description: t("field.placeholder.missingField"), value: model.value });
    }
    return options;
});
</script>
<template>
    <UFieldGroup class="w-full">
        <USelectMenu
            v-model="model"
            :items="items"
            value-key="value"
            :placeholder="$t('field.placeholder.selectField')"
            class="w-full"
        />
        <UButton
            v-if="clearable && model"
            icon="i-tabler-x"
            color="neutral"
            variant="subtle"
            :aria-label="$t('action.clear')"
            @click="model = undefined"
        />
    </UFieldGroup>
</template>
