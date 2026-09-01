<script setup lang="ts">
import type { EmailTemplate } from "~/types/email-template";

defineProps<{
  template: EmailTemplate;
  deleting?: boolean;
}>();

const emit = defineEmits<{
  edit: [template: EmailTemplate];
  delete: [template: EmailTemplate];
}>();
</script>

<template>
  <UCard class="flex flex-col justify-between">
    <div class="space-y-3">
      <div class="flex items-start justify-between gap-2">
        <h3 class="font-semibold text-lg text-highlighted truncate">{{ template.name }}</h3>
      </div>
      <p class="text-xs font-medium text-muted truncate">Subject: {{ template.subject }}</p>
      <div
        class="bg-muted p-3 rounded-lg text-xs leading-relaxed text-dimmed line-clamp-4 whitespace-pre-wrap"
      >
        {{ template.body }}
      </div>
    </div>

    <template #footer>
      <div class="flex items-center justify-end gap-2">
        <UButton
          icon="i-lucide-pencil"
          color="neutral"
          variant="ghost"
          size="xs"
          label="Edit"
          @click="emit('edit', template)"
        />
        <UButton
          icon="i-lucide-trash-2"
          color="error"
          variant="ghost"
          size="xs"
          label="Delete"
          :loading="deleting"
          @click="emit('delete', template)"
        />
      </div>
    </template>
  </UCard>
</template>
