<script setup lang="ts">
const { user, loading, signInWithGoogle } = useAuth()

const providers = [
  {
    label: 'Google',
    icon: 'i-simple-icons-google',
    onClick: signInWithGoogle,
  },
]
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center gap-4 p-4">
    <UPageCard v-if="loading" class="w-full max-w-md">
      <div class="flex flex-col items-center gap-3 text-center">
        <USkeleton class="size-16 rounded-full" />
        <div class="space-y-2">
          <USkeleton class="h-4 w-50" />
          <USkeleton class="h-4 w-36" />
        </div>
      </div>
    </UPageCard>
    <UPageCard v-if="!loading && user" class="w-full max-w-md">
      <div class="flex flex-col items-center gap-3 text-center">
        <img
          v-if="user.picture"
          :src="user.picture"
          :alt="user.name"
          referrerpolicy="no-referrer"
          class="size-16 rounded-full"
        />
        <div
          v-else
          class="bg-primary text-primary-foreground flex size-16 items-center justify-center rounded-full text-xl font-semibold"
          aria-hidden="true"
        >
          {{ user.name.charAt(0).toUpperCase() }}
        </div>
        <div>
          <h1 class="text-xl font-semibold">Welcome, {{ user.name }}</h1>
          <p class="text-muted">{{ user.email }}</p>
        </div>
      </div>
    </UPageCard>
    <UPageCard v-if="!loading && !user" class="w-full max-w-md">
      <UAuthForm title="Lets get you started" :providers="providers" />
    </UPageCard>
  </div>
</template>
