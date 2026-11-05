<template>
    <div class="ring-1 ring-default rounded-md p-2">
        <p v-if="!model.length" class="text-muted text-sm p-2">{{ t("builder.empty") }}</p>
        <Draggable :list="model" handle=".drag-handle">
            <div v-for="(formElement, index) in model" :key="keyOf(formElement)" class="rounded-md p-1">
                <Element
                    :element="formElement"
                    :error="errors?.[index]"
                    :open="openKey === keyOf(formElement)"
                    @edit="toggleEdit(formElement)"
                    @duplicate="duplicateElement(index)"
                    @delete="deleteElementFromForm(index)"
                />
                <UCard
                    v-if="openKey === keyOf(formElement)"
                    class="mt-2 ms-6"
                    :ui="{ body: 'flex flex-col gap-4' }"
                >
                    <ElementForm v-model="model[index]" />
                    <div>
                        <UButton color="neutral" variant="subtle" icon="i-tabler-check" @click="openKey = undefined">
                            {{ t("action.done") }}
                        </UButton>
                    </div>
                </UCard>
            </div>
        </Draggable>
        <UPopover v-model:open="elementSelectorOpen">
            <UButton variant="ghost" icon="i-tabler-plus">
                {{ t("action.create.field") }}
            </UButton>
            <template #content>
                <div class="grid grid-cols-3 sm:grid-cols-4 p-2 gap-2 max-h-96 overflow-y-auto">
                    <BuilderItemSelectCard
                        v-for="element in builderItems"
                        :key="element.type"
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
    </div>
</template>

<script setup lang="ts">
import { VueDraggableNext as Draggable } from "vue-draggable-next";
import { computed, ref } from "vue";
import type { Property } from "../../../bindings";
import { useI18n } from "vue-i18n";
import type { Item } from "./BuilderItemSelectCard.vue";
import { createElement } from "@/utils";
import clone from "just-clone";

const elementSelectorOpen = ref(false);

const { t } = useI18n();

const model = defineModel<Array<Property>>({
    required: true,
});
defineProps<{
    errors?: Record<number, string>;
}>();

// stable keys per element object, index keys break the drag and drop and the open editor when reordering
const keys = new WeakMap<object, number>();
let nextKey = 0;
function keyOf(element: Property) {
    let key = keys.get(element);
    if (key === undefined) {
        key = nextKey++;
        keys.set(element, key);
    }
    return key;
}
const openKey = ref<number>();
function toggleEdit(element: Property) {
    const key = keyOf(element);
    openKey.value = openKey.value === key ? undefined : key;
}

function addElementToForm(element_type: Property["type"]) {
    const newElement = createElement(element_type);
    if (!newElement) return;
    model.value.push(newElement);
    // open the new field right away, it needs at least a label and a name
    if (element_type !== "captcha") openKey.value = keyOf(model.value[model.value.length - 1]);
}

function duplicateElement(index: number) {
    const copy = clone(model.value[index]);
    if (copy.type !== "captcha" && copy.name) copy.name = `${copy.name}_copy`;
    model.value.splice(index + 1, 0, copy);
    openKey.value = keyOf(model.value[index + 1]);
}

function deleteElementFromForm(index: number) {
    model.value.splice(index, 1);
}

const builderItems = computed<Array<Item>>(() =>
    (
        [
            { icon: "i-tabler-text-size", type: "text" },
            { icon: "i-tabler-text-plus", type: "textarea" },
            { icon: "i-tabler-number", type: "number" },
            { icon: "i-tabler-calendar", type: "date" },
            { icon: "i-tabler-select", type: "select" },
            { icon: "i-tabler-check", type: "checkbox" },
            { icon: "i-tabler-adjustments-horizontal", type: "range" },
            { icon: "i-tabler-columns", type: "buttongroup" },
            { icon: "i-tabler-file", type: "file" },
            { icon: "i-tabler-click", type: "button" },
            { icon: "i-tabler-shield-check", type: "captcha" },
        ] satisfies Omit<Item, "label">[]
    )
        // a form can only have one captcha
        .filter((item) => item.type !== "captcha" || !model.value.some((p) => p.type === "captcha"))
        .map((item) => ({ ...item, label: t(`builder.item.label.${item.type}`) })),
);
</script>
