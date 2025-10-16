<script setup lang="ts">
import { CalendarDate } from "@internationalized/date";
import dayjs from "dayjs";

defineProps<{
    future?: boolean;
    past?: boolean;
}>();

const model = defineModel<CalendarDate | undefined>({ required: true });
</script>

<template>
    <UPopover :ui="{ content: 'grid grid-cols-[max-content_1fr] gap-2' }">
        <UButton icon="i-tabler-calendar" variant="outline" color="neutral">
            {{ model ? model.toLocaleString() : $t("field.label.noDate") }}
        </UButton>

        <template #content>
            <div class="border-r border-muted p-2 flex flex-col gap-2">
                <UButton color="neutral" variant="ghost" icon="i-tabler-calendar-off" @click="model = undefined">
                    {{ $t("field.label.noDate") }}
                </UButton>
                <UButton
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="model = new CalendarDate(dayjs().year(), dayjs().month() + 1, dayjs().date())"
                >
                    {{ $t("field.label.today") }}
                </UButton>
                <UButton
                    v-if="past"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().subtract(1, 'day').year(),
                            dayjs().subtract(1, 'day').month() + 1,
                            dayjs().subtract(1, 'day').date(),
                        )
                    "
                >
                    {{ $t("field.label.yesterday") }}
                </UButton>
                <UButton
                    v-if="past"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().subtract(1, 'week').year(),
                            dayjs().subtract(1, 'week').month() + 1,
                            dayjs().subtract(1, 'week').date(),
                        )
                    "
                >
                    {{ $t("field.label.oneWeekAgo") }}
                </UButton>
                <UButton
                    v-if="past"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().subtract(1, 'month').year(),
                            dayjs().subtract(1, 'month').month() + 1,
                            dayjs().subtract(1, 'month').date(),
                        )
                    "
                >
                    {{ $t("field.label.oneMonthAgo") }}
                </UButton>
                <UButton
                    v-if="future"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().add(1, 'day').year(),
                            dayjs().add(1, 'day').month() + 1,
                            dayjs().add(1, 'day').date(),
                        )
                    "
                >
                    {{ $t("field.label.tomorrow") }}
                </UButton>
                <UButton
                    v-if="future"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().add(1, 'week').year(),
                            dayjs().add(1, 'week').month() + 1,
                            dayjs().add(1, 'week').date(),
                        )
                    "
                >
                    {{ $t("field.label.oneWeekFromNow") }}
                </UButton>
                <UButton
                    v-if="future"
                    color="neutral"
                    variant="ghost"
                    icon="i-tabler-calendar"
                    @click="
                        model = new CalendarDate(
                            dayjs().add(1, 'month').year(),
                            dayjs().add(1, 'month').month() + 1,
                            dayjs().add(1, 'month').date(),
                        )
                    "
                >
                    {{ $t("field.label.oneMonthFromNow") }}
                </UButton>
            </div>

            <UCalendar v-model="model" />
        </template>
    </UPopover>
</template>
