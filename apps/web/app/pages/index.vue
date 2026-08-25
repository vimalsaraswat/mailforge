<script setup lang="ts">
const { user, loading, signInWithGoogle } = useAuth();

const isHydrated = ref(false);

onMounted(() => {
  isHydrated.value = true;
});
</script>

<template>
  <AuthShell v-if="!isHydrated || loading || !user">
    <div v-if="!isHydrated || loading" class="w-full max-w-96 text-center space-y-6">
      <div class="space-y-2">
        <USkeleton class="h-3 w-16 mx-auto bg-neutral-400/20" />
        <USkeleton class="h-8 w-56 mx-auto bg-neutral-400/20" />
      </div>
      <div class="space-y-2">
        <USkeleton class="h-4 w-64 mx-auto bg-neutral-400/20" />
        <USkeleton class="h-4 w-48 mx-auto bg-neutral-400/20" />
      </div>
      <USkeleton class="h-12 w-full rounded-lg bg-neutral-400/20" />
      <div class="space-y-2">
        <USkeleton class="h-3 w-40 mx-auto bg-neutral-400/20" />
        <USkeleton class="h-3 w-32 mx-auto bg-neutral-400/20" />
      </div>
    </div>
    <AuthSignInCard v-else @sign-in="signInWithGoogle" />
  </AuthShell>
</template>
