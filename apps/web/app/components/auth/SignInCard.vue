<script setup lang="ts">
const emit = defineEmits<{ signIn: [] }>()
</script>

<template>
  <div class="w-full max-w-96 text-center">
    <p class="mb-2 text-[0.7rem] font-extrabold uppercase tracking-[0.15em] text-primary">
      Welcome
    </p>
    <h2 class="text-[1.65rem] font-bold tracking-[-0.04em] text-highlighted">
      Sign in to Mailforge
    </h2>
    <p class="mx-auto mt-3 max-w-80 text-sm leading-6 text-muted">
      Use your Google account to access your Mailforge account.
    </p>
    <UButton
      label="Continue with Google"
      icon="i-simple-icons-google"
      color="neutral"
      variant="solid"
      block
      size="lg"
      class="mt-8 shadow-lg"
      @click="emit('signIn')"
    />
    <p class="mx-auto mt-6 max-w-72 text-[0.7rem] leading-5 text-dimmed">
      By continuing, you agree to sign in securely with Google.
    </p>
  </div>
</template>
