<script setup lang="ts">
import type { Form } from "../../../bindings";
import { useApiQuery } from "@/utils/api";
import { computed, onBeforeUnmount, ref } from "vue";
import { refDebounced } from "@vueuse/core";

const props = defineProps<{
    form: Form;
}>();

// stringify tracks the form deeply; debounce so typing doesn't send a request per keystroke
const formJson = computed(() => JSON.stringify(props.form));
const debouncedJson = refDebounced(formJson, 400);
const rendered = useApiQuery(
    "/api/forms/preview",
    { method: "POST", body: () => JSON.parse(debouncedJson.value) as Form },
    { placeholderData: (previous) => previous },
);

const iframe = ref<HTMLIFrameElement>();
const height = ref(150);
let observer: ResizeObserver | undefined;
function onLoad() {
    observer?.disconnect();
    const body = iframe.value?.contentDocument?.body;
    if (!body) return;
    // tailwind is compiled in the browser after load, so the height changes later
    observer = new ResizeObserver(() => {
        height.value = (iframe.value?.contentDocument?.documentElement.scrollHeight ?? 150) + 2;
    });
    observer.observe(body);
}
onBeforeUnmount(() => observer?.disconnect());
</script>
<template>
    <UFormField :label="$t('field.label.preview')" :help="$t('hint.preview')">
        <div class="relative">
            <iframe
                ref="iframe"
                class="w-full rounded-lg ring-1 ring-default bg-white"
                :style="{ height: `${height}px` }"
                :srcdoc="rendered.data.value?.html"
                :title="$t('field.label.preview')"
                frameborder="0"
                @load="onLoad"
            ></iframe>
            <UIcon
                v-if="rendered.isLoading.value || debouncedJson !== formJson"
                name="i-tabler-loader-2"
                class="absolute top-2 right-2 animate-spin text-muted"
            />
        </div>
    </UFormField>
</template>
