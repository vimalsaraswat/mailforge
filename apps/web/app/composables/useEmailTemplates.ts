import { templateRoutes } from "~/services/email-template";
import type { EmailTemplate, EmailTemplateInput } from "~/types/email-template";

export function useEmailTemplates() {
  const {
    data: templates,
    refresh,
    pending: loading,
    error,
  } = useAPI<EmailTemplate[]>(templateRoutes.list);
  const baseURL = useRuntimeConfig().public.apiBase;

  const saving = ref(false);
  const deleting = ref(false);

  async function save(id: string | null, input: EmailTemplateInput): Promise<EmailTemplate | null> {
    saving.value = true;
    try {
      if (id) {
        const response = await $fetch<EmailTemplate>(templateRoutes.update(id), {
          method: "PUT",
          baseURL,
          body: input,
          credentials: "include",
        });
        await refresh();
        return response;
      } else {
        const response = await $fetch<EmailTemplate>(templateRoutes.create, {
          method: "POST",
          baseURL,
          body: input,
          credentials: "include",
        });
        await refresh();
        return response;
      }
    } finally {
      saving.value = false;
    }
  }

  async function remove(id: string): Promise<boolean> {
    deleting.value = true;
    try {
      await $fetch(templateRoutes.delete(id), {
        method: "DELETE",
        baseURL,
        credentials: "include",
      });
      await refresh();
      return true;
    } catch {
      return false;
    } finally {
      deleting.value = false;
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
  };
}
