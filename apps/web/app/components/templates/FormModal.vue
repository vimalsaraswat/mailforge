<script setup lang="ts">
import type { EmailTemplate, EmailTemplateInput } from "~/types/email-template";

const props = defineProps<{
  template?: EmailTemplate | null;
  saving?: boolean;
}>();

const open = defineModel<boolean>("open");

const emit = defineEmits<{
  submit: [input: EmailTemplateInput, id: string | null];
}>();

const form = reactive<EmailTemplateInput>({ name: "", subject: "", body: "" });

watch(
  () => props.template,
  (tmpl) => {
    if (tmpl) {
      form.name = tmpl.name;
      form.subject = tmpl.subject;
      form.body = tmpl.body;
    } else {
      form.name = "";
      form.subject = "";
      form.body = "";
    }
  },
  { immediate: true },
);

function handleSubmit() {
  if (!form.name.trim() || !form.subject.trim() || !form.body.trim()) return;
  emit(
    "submit",
    { name: form.name.trim(), subject: form.subject.trim(), body: form.body.trim() },
    props.template?.id ?? null,
  );
}
</script>

<template>
  <UModal v-model:open="open">
    <template #content>
      <div class="p-6 space-y-6">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-bold text-highlighted">
            {{ template ? "Edit Template" : "New Template" }}
          </h3>
          <UButton
            icon="i-lucide-x"
            color="neutral"
            variant="ghost"
            size="xs"
            square
            @click="open = false"
          />
        </div>

        <form class="space-y-4" @submit.prevent="handleSubmit">
          <UFormField label="Template Name" required>
            <UInput
              v-model="form.name"
              placeholder="Follow-up email"
              autocomplete="off"
              class="w-full"
            />
          </UFormField>

          <UFormField label="Subject" required>
            <UInput
              v-model="form.subject"
              placeholder="Quick follow-up"
              autocomplete="off"
              class="w-full"
            />
          </UFormField>

          <UFormField label="Message" required>
            <UTextarea
              v-model="form.body"
              :rows="6"
              placeholder="Hi {{firstName}},"
              autoresize
              class="w-full"
            />
          </UFormField>

          <div class="flex items-center justify-end gap-3 pt-4">
            <UButton label="Cancel" color="neutral" variant="ghost" @click="open = false" />
            <UButton
              type="submit"
              color="primary"
              :label="template ? 'Save Changes' : 'Create Template'"
              :loading="saving"
              :disabled="!form.name.trim() || !form.subject.trim() || !form.body.trim()"
            />
          </div>
        </form>
      </div>
    </template>
  </UModal>
</template>
