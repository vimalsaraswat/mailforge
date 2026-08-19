<script setup lang="ts">
import type { CurrentUser } from '~/types/auth'

defineProps<{
  user: CurrentUser
  loggingOut: boolean
  gmailConnectUrl: string
}>()

const emit = defineEmits<{ logout: [] }>()
</script>

<template>
  <UPageCard class="w-full max-w-96" variant="subtle">
    <div class="flex items-center gap-4">
      <UAvatar
        :as="{ img: 'img' }"
        :src="user.picture ?? undefined"
        :alt="user.name"
        :text="user.name.charAt(0).toUpperCase()"
        size="xl"
        color="primary"
        loading="eager"
        referrerpolicy="no-referrer"
      />
      <div class="min-w-0">
        <p class="m-0 text-xs text-muted">Welcome back</p>
        <h2 class="truncate text-[1.65rem] font-bold tracking-[-0.04em] text-highlighted">
          {{ user.name }}
        </h2>
        <p class="truncate text-xs text-muted">{{ user.email }}</p>
      </div>
    </div>
    <USeparator class="my-8" />
    <div class="space-y-3">
      <UButton
        :label="user.gmail_connected ? 'Gmail Connected' : 'Connect Gmail Account'"
        :color="user.gmail_connected ? 'success' : 'primary'"
        variant="solid"
        icon="i-lucide-mail"
        block
        size="lg"
        :to="user.gmail_connected ? undefined : gmailConnectUrl"
        :disabled="!!user.gmail_connected"
      />
      <UButton
        label="Log out"
        color="neutral"
        variant="outline"
        icon="i-lucide-log-out"
        block
        size="lg"
        :loading="loggingOut"
        @click="emit('logout')"
      />
    </div>
  </UPageCard>
</template>
