<script setup lang="ts">
const { user, logout, loggingOut } = useAuth();
</script>

<template>
  <div class="flex min-h-screen bg-muted">
    <!-- Sidebar Navigation -->
    <aside
      class="hidden lg:flex w-64 flex-col border-r border-default bg-default p-6 justify-between"
    >
      <div class="space-y-6">
        <!-- Logo/Brand Header -->
        <div class="flex items-center gap-2 font-bold text-highlighted px-2">
          <span class="grid size-8 place-items-center rounded-lg bg-primary text-primary-contrast">
            <UIcon name="i-lucide-mail" class="size-4" />
          </span>
          Mailforge
        </div>

        <!-- Vertical Navigation Menu -->
        <UNavigationMenu
          :items="[
            { label: 'Templates', icon: 'i-lucide-layout-template', to: '/dashboard' },
            { label: 'Settings', icon: 'i-lucide-settings', to: '/dashboard' },
          ]"
          orientation="vertical"
          class="w-full"
        />
      </div>

      <!-- User Profile / Footer / Logout -->
      <div class="border-t border-default pt-4 space-y-4">
        <div v-if="user" class="flex items-center justify-between gap-3 px-2">
          <div class="flex items-center gap-3 overflow-hidden">
            <UAvatar
              :src="user.picture ?? undefined"
              :alt="user.name"
              :text="user.name.charAt(0).toUpperCase()"
              size="sm"
            />
            <div class="truncate text-xs">
              <p class="font-medium text-highlighted truncate">{{ user.name }}</p>
              <p class="text-muted truncate">{{ user.email }}</p>
            </div>
          </div>
          <UButton
            icon="i-lucide-log-out"
            variant="ghost"
            color="neutral"
            size="xs"
            :loading="loggingOut"
            @click="logout"
          />
        </div>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col min-w-0">
      <header
        class="h-16 border-b border-default bg-default px-6 flex items-center justify-between"
      >
        <div class="flex items-center gap-2 font-bold text-highlighted lg:hidden">
          <span class="grid size-8 place-items-center rounded-lg bg-primary text-primary-contrast">
            <UIcon name="i-lucide-mail" class="size-4" />
          </span>
          Mailforge
        </div>
        <div class="flex items-center gap-4 ml-auto">
          <UButton
            v-if="user"
            icon="i-lucide-log-out"
            variant="ghost"
            color="neutral"
            size="sm"
            label="Logout"
            class="lg:hidden"
            :loading="loggingOut"
            @click="logout"
          />
        </div>
      </header>

      <!-- Templates Dashboard Content -->
      <div class="flex-1 w-full">
        <TemplatesManager />
      </div>
    </main>
  </div>
</template>
