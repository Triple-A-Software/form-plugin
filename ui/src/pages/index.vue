<script setup lang="ts"></script>
<template>
    <div class="flex flex-row gap-4 w-full">
        <UCard class="w-full" :ui="{ body: 'flex flex-col gap-4 justify-between h-full' }">
            <div class="prose prose-invert">
                <h2>{{ $t("pages.forms.title") }}</h2>
                <p>{{ $t("pages.forms.description") }}</p>
            </div>
            <UButton to="/forms" class="w-max" icon="i-tabler-forms">{{ $t("action.goto.forms") }}</UButton>
        </UCard>
        <UCard class="w-full" :ui="{ body: 'flex flex-col gap-4 justify-between h-full' }">
            <div class="prose prose-invert">
                <h2>{{ $t("pages.emails.title") }}</h2>
                <p>{{ $t("pages.emails.description") }}</p>
            </div>
            <UButton to="/emails" icon="i-tabler-mail" class="w-max" color="neutral">{{ $t("action.goto.emails") }}</UButton>
        </UCard>
        <UCard class="w-full" :ui="{ body: 'flex flex-col gap-4 justify-between h-full' }">
            <div class="prose prose-invert">
                <h2>{{ $t("pages.submissions.title") }}</h2>
                <p>{{ $t("pages.submissions.description") }}</p>
            </div>
            <UButton to="/submissions" icon="i-tabler-send" class="w-max" color="neutral">
                {{ $t("action.goto.submissions") }}
            </UButton>
        </UCard>
    </div>
</template>
