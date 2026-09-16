import { templateRoutes } from "~/api/email-template";
import type { EmailTemplate, EmailTemplateInput } from "~/types/email-template";

export function useEmailTemplates() {
  const {
    data: templates,
    execute: refresh,
    pending: loading,
    error: loadError,
  } = useAPI<EmailTemplate[]>(templateRoutes.list, { immediate: false });

  const saving = ref(false);
  const deleting = ref(false);
  const mutationError = shallowRef<Error | null>(null);
  const error = computed(() => mutationError.value ?? loadError.value);

  function setMutationError(cause: unknown): void {
    mutationError.value = cause instanceof Error ? cause : new Error("Unable to update template.");
  }

  async function save(id: string | null, input: EmailTemplateInput): Promise<EmailTemplate | null> {
    saving.value = true;
    mutationError.value = null;
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
    } catch (cause) {
      setMutationError(cause);
      return null;
    } finally {
      saving.value = false;
    }
  }

  async function remove(id: string): Promise<boolean> {
    deleting.value = true;
    mutationError.value = null;
    try {
      await $api(templateRoutes.delete(id), { method: "DELETE" });
      await refresh();
      return true;
    } catch (cause) {
      setMutationError(cause);
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
