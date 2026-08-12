import { createApiClient } from '~/services/api'
import { createEmailTemplateService } from '~/services/email-template'
import type { EmailTemplate, EmailTemplateInput } from '~/types/email-template'

function messageFor(error: unknown) {
  if (error && typeof error === 'object' && 'data' in error) {
    const data = error.data
    if (typeof data === 'string') return data
  }

  return 'Something went wrong while saving your templates. Please try again.'
}

export function useEmailTemplates() {
  const config = useRuntimeConfig()
  const service = createEmailTemplateService(createApiClient(config.public.apiBase))

  const templates = ref<EmailTemplate[]>([])
  const loading = ref(true)
  const saving = ref(false)
  const deleting = ref(false)
  const error = ref<string | null>(null)

  async function refresh() {
    loading.value = true
    error.value = null

    try {
      templates.value = await service.list()
    } catch (caught) {
      error.value = messageFor(caught)
    } finally {
      loading.value = false
    }
  }

  async function save(id: string | null, input: EmailTemplateInput) {
    saving.value = true
    error.value = null

    try {
      const template = id ? await service.update(id, input) : await service.create(input)
      const index = templates.value.findIndex(({ id }) => id === template.id)

      if (index === -1) templates.value.unshift(template)
      else templates.value.splice(index, 1, template)

      return template
    } catch (caught) {
      error.value = messageFor(caught)
      return null
    } finally {
      saving.value = false
    }
  }

  async function remove(id: string) {
    deleting.value = true
    error.value = null

    try {
      await service.delete(id)
      templates.value = templates.value.filter((template) => template.id !== id)
      return true
    } catch (caught) {
      error.value = messageFor(caught)
      return false
    } finally {
      deleting.value = false
    }
  }

  return { templates, loading, saving, deleting, error, refresh, save, remove }
}
