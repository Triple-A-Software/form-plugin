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
                        color="gray"
                        icon="i-tabler-edit"
                        @click="
                            showFormElementConfig = true;
                            currentIndex = index;
                        "
                    />
                    <UButton color="red" icon="i-tabler-trash" @click="deleteElementFromForm(index)" />
                </div>
            </div>
        </Draggable>
        <UPopover overlay>
            <UButton variant="ghost" icon="i-tabler-plus" class="w-full">
                {{ t().action.create.any() }}
            </UButton>
            <template #panel="{ close }">
                <div class="grid grid-cols-4 overflow-y-scroll p-2 gap-2">
                    <ComponentCard
                        v-for="element in builderItems"
                        :data="element"
                        fallback-icon="i-tabler-code"
                        @click="
                            addElementToForm(element);
                            close();
                        "
                    >
                        <template #label>
                            {{ element.label[locale] }}
                        </template>
                    </ComponentCard>
                </div>
            </template>
        </UPopover>
        <USlideover v-model="showFormElementConfig">
            <SlideoverCard>
                <template #header>
                    <ModalHeader @close="showFormElementConfig = false">
                        {{ t().modal.edit.title(t().table.field()) }}
                    </ModalHeader>
                </template>
                <div class="space-y-4">
                    <InputFromForm
                        v-for="prop in model[currentIndex].props"
                        v-model="model[currentIndex].data[prop.id]"
                        :field="prop"
                    />
                </div>
            </SlideoverCard>
        </USlideover>
    </div>
</template>
<script setup lang="ts">
import { VueDraggableNext as Draggable } from "vue-draggable-next";
import { ref } from "vue";
import { useI18n } from "~/locales";
// import type { BuilderItem } from "shared-interfaces/bindings";

const { t, locale } = useI18n();

const showFormElementConfig = ref(false);
const currentIndex = ref(-1);

defineProps<{
    builderItems: Array<Item>;
}>();

const model = defineModel<Array<Item>>({
    default: [],
});

function addElementToForm(element: Item) {
    const newElement = { ...element, data: { ...element.data }, props: [...element.props] };
    model.value?.push(newElement);
}

function deleteElementFromForm(index: number) {
    model.value = model.value?.filter((_, _index) => index !== _index);
}
</script>
