<template>
    <div class="ring-1 ring-gray-300 dark:ring-gray-700 rounded-md p-2">
        <Draggable :list="model">
            <div
                v-if="model?.length"
                v-for="(formElement, index) in model"
                :key="index"
                class="dark:hover:bg-gray-800 hover:bg-gray-200 rounded-md p-1"
            >
                <div class="flex gap-2 w-full items-center">
                    <UIcon name="i-tabler-grip-vertical" class="cursor-grab" />
                    <label v-if="formElement.data.label">
                        {{ formElement.data.label }}
                        <span class="text-red-600" v-if="formElement.data.required"> * </span>
                    </label>
                    <p class="italic text-gray-600 dark:text-gray-400 text-sm"><{{ formElement.label[locale] }}></p>
                    <div class="flex-1"></div>
                    <UButton
                        color="neutral"
                        icon="i-tabler-edit"
                        @click="
                            showFormElementConfig = true;
                            currentIndex = index;
                        "
                    />
                    <UButton color="error" icon="i-tabler-trash" @click="deleteElementFromForm(index)" />
                </div>
            </div>
        </Draggable>
        <UPopover overlay>
            <UButton variant="ghost" icon="i-tabler-plus">
                {{ t("action.create.field") }}
            </UButton>
            <template #content>
                <div class="grid grid-cols-4 overflow-y-scroll p-2 gap-2">
                    <BuilderItemSelectCard
                        v-for="element in builderItems"
                        :data="element"
                        fallback-icon="i-tabler-code"
                        @click="addElementToForm(element)"
                    >
                        <template #label>
                            {{ element.label[locale] }}
                        </template>
                    </BuilderItemSelectCard>
                </div>
            </template>
        </UPopover>
        <USlideover v-model="showFormElementConfig" :title="t('modal.edit.field.title')">
            <template #body>
                <div class="space-y-4">
                    <FromFormInput
                        v-for="prop in model[currentIndex]?.props ?? []"
                        v-model="model[currentIndex].data[prop.id]"
                        :field="prop"
                    />
                </div>
            </template>
        </USlideover>
    </div>
</template>
<script setup lang="ts">
import { VueDraggableNext as Draggable } from "vue-draggable-next";
import { ref } from "vue";
import type { Property } from "../../../bindings";
import type { JsonValue } from "../../../bindings/serde_json/JsonValue";
import { useI18n } from "vue-i18n";
import clone from "just-clone";

const showFormElementConfig = ref(false);
const currentIndex = ref(-1);

const { t, locale } = useI18n();

defineProps<{
    properties: Array<Property>;
    data: JsonValue;
}>();

const model = defineModel<Array<Property>>({
    default: [],
});

function addElementToForm(element: Property) {
    const newElement = clone(element);
    model.value?.push(newElement);
}

function deleteElementFromForm(index: number) {
    model.value = model.value?.filter((_, _index) => index !== _index);
}
</script>
