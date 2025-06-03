<script setup lang="ts" generic="T extends object">
import clone from "just-clone";

const model = defineModel<Array<T>>({
    required: true,
});
const props = defineProps<{
    empty: T;
}>();
function create() {
    const new_empty = clone(props.empty);
    model.value.push(new_empty);
}
function remove(index: number) {
    model.value.splice(index, 1);
}
</script>
<template>
    <div class="flex flex-col gap-2">
        <div v-for="(item, index) in model" class="flex flex-row gap-4">
            <div>
                <slot name="edit-item" :item="item"></slot>
            </div>
            <UButton color="error" variant="ghost" @click="remove(index)" icon="i-tabler-trash" />
        </div>
        <UButton @click="create" color="neutral" icon="i-tabler-plus">{{ $t("action.create.option") }}</UButton>
    </div>
</template>
