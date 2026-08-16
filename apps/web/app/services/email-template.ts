import type { ApiClient } from '~/services/api'
import type { EmailTemplate, EmailTemplateInput } from '~/types/email-template'

const templateRoutes = {
  list: '/templates',
  byId: (id: string) => `/templates/${id}`,
} as const

export interface EmailTemplateService {
  list(): Promise<EmailTemplate[]>
  create(input: EmailTemplateInput): Promise<EmailTemplate>
  update(id: string, input: EmailTemplateInput): Promise<EmailTemplate>
  delete(id: string): Promise<void>
}

export function createEmailTemplateService(api: ApiClient): EmailTemplateService {
  return {
    list() {
      return api.get<EmailTemplate[]>(templateRoutes.list)
    },

    create(input: EmailTemplateInput) {
      return api.post<EmailTemplate>(templateRoutes.list, input)
    },

    update(id: string, input: EmailTemplateInput) {
      return api.put<EmailTemplate>(templateRoutes.byId(id), input)
    },

    delete(id: string) {
      return api.delete(templateRoutes.byId(id))
    },
  }
}
