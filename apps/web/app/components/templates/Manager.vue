<script setup lang="ts">
import type { EmailTemplate, EmailTemplateInput } from "~/types/email-template";

const { templates, loading, saving, deleting, error, refresh, save, remove } = useEmailTemplates();

const isModalOpen = ref(false);
const activeTemplate = ref<EmailTemplate | null>(null);

function openCreateModal() {
  activeTemplate.value = null;
  isModalOpen.value = true;
}

function openEditModal(template: EmailTemplate) {
  activeTemplate.value = template;
  isModalOpen.value = true;
}

async function handleSave(input: EmailTemplateInput, id: string | null) {
  const template = await save(id, input);
  if (template) {
    isModalOpen.value = false;
    activeTemplate.value = null;
  }
}

async function handleDelete(template: EmailTemplate) {
  if (!window.confirm(`Delete "${template.name}"? This cannot be undone.`)) return;
  await remove(template.id);
}

onMounted(refresh);
</script>

<template>
  <div class="p-6 md:p-8 max-w-7xl mx-auto w-full space-y-8">
    <!-- Header Actions -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
      <div>
        <h1 class="text-2xl font-bold tracking-tight text-highlighted">Templates</h1>
        <p class="text-sm text-muted mt-1">
          Create and manage your reusable email templates for outreach campaigns.
        </p>
      </div>
      <div class="flex items-center gap-3">
        <UButton
          icon="i-lucide-refresh-cw"
          variant="ghost"
          color="neutral"
          square
          :loading="loading"
          @click="() => refresh()"
        />
        <UButton
          icon="i-lucide-plus"
          label="New Template"
          color="primary"
          @click="openCreateModal"
        />
      </div>
    </div>

    <!-- Error State -->
    <UAlert
      v-if="error"
      color="error"
      variant="subtle"
      icon="i-lucide-circle-alert"
      :title="error.message"
    />

    <!-- Main Content Area -->
    <div>
      <!-- Loading State -->
      <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <UCard v-for="item in 6" :key="item" class="space-y-4">
          <USkeleton class="h-6 w-3/4" />
          <USkeleton class="h-4 w-1/2" />
          <USkeleton class="h-16 w-full" />
        </UCard>
      </div>

      <!-- Empty State -->
      <div
        v-else-if="!templates?.length"
        class="text-center py-16 border border-dashed border-default rounded-xl bg-default"
      >
        <UIcon name="i-lucide-file-text" class="size-12 mx-auto text-muted mb-4" />
        <h3 class="text-lg font-medium text-highlighted">No templates yet</h3>
        <p class="text-sm text-muted mt-1 mb-6">
          Create your first reusable email template to start sending messages.
        </p>
        <UButton icon="i-lucide-plus" label="Create Template" @click="openCreateModal" />
      </div>

      <!-- Templates Grid List -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <TemplatesCard
          v-for="template in templates"
          :key="template.id"
          :template="template"
          :deleting="deleting"
          @edit="openEditModal"
          @delete="handleDelete"
        />
      </div>
    </div>

    <!-- Form Modal Component -->
    <TemplatesFormModal
      v-model:open="isModalOpen"
      :template="activeTemplate"
      :saving="saving"
      @submit="handleSave"
    />
  </div>
</template>
