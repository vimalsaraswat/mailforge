import { templateRoutes } from "~/api/email-template";
import type { EmailTemplate, EmailTemplateInput } from "~/types/email-template";

export function useEmailTemplates() {
  const {
    data: templates,
    refresh,
    pending: loading,
    error,
  } = useAPI<EmailTemplate[]>(templateRoutes.list);

  const saving = ref(false);
  const deleting = ref(false);

  async function save(id: string | null, input: EmailTemplateInput): Promise<EmailTemplate | null> {
    saving.value = true;
    try {
      const response = id
        ? await $api<EmailTemplate>(templateRoutes.update(id), {
            method: "PUT",
            body: input,
          })
        : await $api<EmailTemplate>(templateRoutes.create, {
            method: "POST",
            body: input,
          });

      await refresh();
      return response;
    } finally {
      saving.value = false;
    }
  }

  async function remove(id: string): Promise<boolean> {
    deleting.value = true;
    try {
      await $api(templateRoutes.delete(id), { method: "DELETE" });
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
