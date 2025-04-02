<script setup lang="ts">
import { computed } from "vue";
import type { NavigationMenuItem } from "@nuxt/ui";

const links = computed<NavigationMenuItem[]>(() => [
    { label: "Home", to: "/" },
    { label: "Forms", to: "/forms" },
]);
</script>
<template>
    <UContainer>
        <UNavigationMenu :items="links" />
        <RouterView />
    </UContainer>
</template>
