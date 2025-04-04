<template>
    <div class="ring-1 ring-gray-200 dark:ring-gray-800 rounded-md p-2">
        <Draggable :list="model">
            <div v-if="model?.length" v-for="(formElement, index) in model" :key="index" class="rounded-md p-1">
                <Element
                    :element="formElement"
                    @delete="deleteElementFromForm(index)"
                    @edit="
                        elementEditOpen = true;
                        currentlySelectedElement = index;
                    "
                />
            </div>
        </Draggable>
        <UPopover overlay v-model:open="elementSelectorOpen">
            <UButton variant="ghost" icon="i-tabler-plus">
                {{ t("action.create.field") }}
            </UButton>
            <template #content>
                <div class="grid grid-cols-4 overflow-y-scroll p-2 gap-2">
                    <BuilderItemSelectCard
                        v-for="element in builderItems"
                        :element="element"
                        fallback-icon="i-tabler-code"
                        @click="
                            addElementToForm($event.type);
                            elementSelectorOpen = false;
                        "
                    />
                </div>
            </template>
        </UPopover>
        <USlideover
            v-model:open="elementEditOpen"
            v-if="currentlySelectedElement != null"
            :title="t('modal.edit.field', { type: t(`builder.item.label.${model[currentlySelectedElement].type}`) })"
        >
            <template #body>
                <div class="flex flex-col gap-4">
                    <ElementForm v-model="model[currentlySelectedElement]" />
                </div>
            </template>
        </USlideover>
    </div>
</template>
<script setup lang="ts">
import { VueDraggableNext as Draggable } from "vue-draggable-next";
import { computed, ref, watch } from "vue";
import type { Property } from "../../../bindings";
import { useI18n } from "vue-i18n";
import type { Item } from "./BuilderItemSelectCard.vue";
import { createElement } from "@/utils";
import type { NullToUndefined } from "@/utils/types/utils";

const elementEditOpen = ref(false);
const elementSelectorOpen = ref(false);

const { t } = useI18n();

const model = defineModel<Array<NullToUndefined<Property>>>({
    required: true,
});

function addElementToForm(element_type: Property["type"]) {
    const newElement = createElement(element_type);
    if (!newElement) return;
    model.value?.push(newElement);
}

function deleteElementFromForm(index: number) {
    model.value.splice(index, 1);
}

const builderItems = computed<Array<Item>>(() => [
    {
        icon: "i-tabler-text-plus",
        label: t("builder.item.label.textarea"),
        type: "textarea",
    },
    {
        icon: "i-tabler-text-size",
        label: t("builder.item.label.text"),
        type: "text",
    },
    // {
    //     icon: "i-tabler-text-grammar",
    //     label: t("builder.item.label.richtext"),
    //     type: "richtext",
    // },
    {
        icon: "i-tabler-check",
        label: t("builder.item.label.checkbox"),
        type: "checkbox",
    },
    {
        icon: "i-tabler-number",
        label: t("builder.item.label.number"),
        type: "number",
    },
    {
        icon: "i-tabler-select",
        label: t("builder.item.label.select"),
        type: "select",
    },
    {
        icon: "i-tabler-number-123",
        label: t("builder.item.label.range"),
        type: "range",
    },
    {
        icon: "i-tabler-hand-click",
        label: t("builder.item.label.buttongroup"),
        type: "buttongroup",
    },
    {
        icon: "i-tabler-hand-click",
        label: t("builder.item.label.button"),
        type: "button",
    },
]);
const currentlySelectedElement = ref<number | null>(null);
watch(elementEditOpen, (open) => {
    if (!open) {
        currentlySelectedElement.value = null;
    }
});
</script>
