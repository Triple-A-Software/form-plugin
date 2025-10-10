<script setup lang="ts">
import { useApiQuery } from "@/utils/api";
import dayjs from "dayjs";
import { computed } from "vue";
import { BarChart, LegendPosition } from "vue-chrts";

const stats = useApiQuery("/api/submissions/stats", {
    method: "GET",
});
const forms = useApiQuery("/api/forms", {
    method: "GET",
});

// these are the tailwindcss colors converted to hex, because vue-chrts/@unovis
// doesn't support oklch colors and the tailwind colors are oklch
const colors = [
    "#ff8904",
    "#00c951",
    "#ff2056",
    "#2b7fff",
    "#efb100",
    "#00bba7",
    "#615fff",
    "#00b8db",
    "#f6339a",
    "#7ccf00",
    "#fd9a00",
    "#8e51ff",
    "#00bc7d",
    "#e12afb",
    "#fb2c36",
    "#ad46ff",
    "#00a6f4",
];
const categories = computed(() =>
    Object.fromEntries(
        forms.data.value?.map((f, i) => [
            `${f.id}`,
            {
                name: f.name,
                color: colors.at(i % colors.length),
            },
        ]) ?? [],
    ),
);

const xFormatter = (i: number): string => `${dayjs.utc(stats.data.value?.graph.at(i)?.date).format("YYYY-MM-DD")}`;
</script>

<template>
    <BarChart
        v-if="stats.data.value?.graph.length && forms.data.value?.length"
        :data="stats.data.value.graph"
        :height="300"
        :categories
        :y-axis="forms.data.value.map((f) => `${f.id}`)"
        :group-padding="0"
        :bar-padding="0.2"
        :x-num-ticks="stats.data.value.graph.length - 1 /* i don't know why -1 but otherwise theres always some empty columns */"
        :radius="4"
        :x-formatter="xFormatter"
        :legend-position="LegendPosition.Top"
        y-grid-line
    />
</template>
