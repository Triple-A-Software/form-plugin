<script setup lang="ts">
import LocalizedTextInput from "@/components/input/LocalizedTextInput.vue";
import { createButtongroup, type PropertySelector } from "@/utils";
import { useI18n } from "vue-i18n";

const { locale } = useI18n();

const model = defineModel<PropertySelector<"buttongroup">>({
    required: true,
    default: createButtongroup(),
});
</script>
<template>
    <CommonElementForm v-model="model" />
    <UFormField :label="$t('field.label.default_value')">
        <UFieldGroup class="w-full">
            <UButton
                :variant="!model.default_value ? 'solid' : 'subtle'"
                :color="!model.default_value ? 'primary' : 'neutral'"
                @click="model.default_value = undefined"
            >
                {{ $t("field.option.default_value.nothing") }}
            </UButton>
            <UButton
                v-for="option in model.options"
                :variant="model.default_value === option.value ? 'solid' : 'subtle'"
                :color="model.default_value === option.value ? 'primary' : 'neutral'"
                @click="model.default_value = option.value"
            >
                {{ option.label[locale] }}
            </UButton>
        </UFieldGroup>
    </UFormField>
    <UFormField :label="$t('field.label.options')">
        <ArrayInput
            v-model="model.options"
            :empty="{
                label: {},
                value: '',
            }"
            class="w-full"
        >
            <template #edit-item="{ item }">
                <UFormField :label="$t('field.label.label')">
                    <LocalizedTextInput v-model="item.label" />
                </UFormField>
                <UFormField :label="$t('field.label.value')">
                    <UInput v-model="item.value" />
                </UFormField>
            </template>
        </ArrayInput>
    </UFormField>
    <UFormField :label="$t('field.label.button_class')">
        <UInput v-model="model.button_class" class="w-full" />
    </UFormField>
</template>
