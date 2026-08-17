import { createApiClient, getApiErrorMessage } from '~/services/api'
import { createEmailTemplateService } from '~/services/email-template'
import type { EmailTemplate, EmailTemplateInput } from '~/types/email-template'

function messageFor(error: unknown, fallback: string): string {
  return getApiErrorMessage(error) ?? fallback
}

export function useEmailTemplates() {
  const config = useRuntimeConfig()
  const service = createEmailTemplateService(createApiClient(config.public.apiBase))

  const templates = ref<EmailTemplate[]>([])
  const loading = ref(true)
  const saving = ref(false)
  const deleting = ref(false)
  const error = ref<string | null>(null)

  async function refresh(): Promise<EmailTemplate[]> {
    loading.value = true
    error.value = null

    try {
      templates.value = await service.list()
      return templates.value
    } catch (caught) {
      error.value = messageFor(
        caught,
        'Something went wrong while loading your templates. Please try again.',
      )
      return []
    } finally {
      loading.value = false
    }
  }

  async function save(id: string | null, input: EmailTemplateInput): Promise<EmailTemplate | null> {
    saving.value = true
    error.value = null

    try {
      const template = id ? await service.update(id, input) : await service.create(input)
      const index = templates.value.findIndex(({ id }) => id === template.id)

      if (index === -1) templates.value.unshift(template)
      else templates.value.splice(index, 1, template)

      return template
    } catch (caught) {
      error.value = messageFor(
        caught,
        'Something went wrong while saving your template. Please try again.',
      )
      return null
    } finally {
      saving.value = false
    }
  }

  async function remove(id: string): Promise<boolean> {
    deleting.value = true
    error.value = null

    try {
      await service.delete(id)
      templates.value = templates.value.filter((template) => template.id !== id)
      return true
    } catch (caught) {
      error.value = messageFor(
        caught,
        'Something went wrong while deleting your template. Please try again.',
      )
      return false
    } finally {
      deleting.value = false
    }
  }

  return {
    templates: readonly(templates),
    loading: readonly(loading),
    saving: readonly(saving),
    deleting: readonly(deleting),
    error: readonly(error),
    refresh,
    save,
    remove,
  }
}
