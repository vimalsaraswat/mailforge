<script setup lang="ts">
const { user, loading, loggingOut, logout, signInWithGoogle } = useAuth()
</script>

<template>
  <AuthShell>
    <div v-if="loading" class="flex w-full max-w-96 flex-col items-center gap-6">
      <USkeleton class="size-14 rounded-2xl" />
      <div class="w-full space-y-3">
        <USkeleton class="h-7 w-3/5" />
        <USkeleton class="h-4 w-full" />
        <USkeleton class="h-12 w-full rounded-xl" />
      </div>
    </div>
    <AuthAccountCard v-else-if="user" :user="user" :logging-out="loggingOut" @logout="logout" />
    <AuthSignInCard v-else @sign-in="signInWithGoogle" />
  </AuthShell>
</template>
