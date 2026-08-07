<script setup lang="ts">
const currentYear = new Date().getFullYear()
</script>

<template>
  <main
    class="relative flex min-h-svh items-center justify-center overflow-hidden bg-default px-5 py-8"
  >
    <div
      aria-hidden="true"
      class="pointer-events-none absolute -right-64 -top-76 size-136 rounded-full border border-primary/10"
    />
    <div
      aria-hidden="true"
      class="pointer-events-none absolute -bottom-60 -left-64 size-104 rounded-full border border-primary/10"
    />
    <div
      class="relative grid min-h-160 w-full max-w-268 overflow-hidden rounded-3xl border border-default bg-default shadow-2xl lg:grid-cols-[1fr_0.92fr]"
    >
      <section
        aria-labelledby="brand-title"
        class="flex flex-col justify-between bg-primary-950 p-8 sm:p-12 lg:p-18"
      >
        <div class="flex items-center gap-3">
          <div class="grid size-10 place-items-center rounded-xl bg-primary-200 text-primary-950">
            <UIcon name="i-lucide-mail" class="size-5" />
          </div>
          <span class="text-[1.05rem] font-bold tracking-tight">Mailforge</span>
        </div>
        <div class="my-14 max-w-120 lg:my-20">
          <p class="mb-3 text-[0.7rem] font-extrabold uppercase tracking-[0.15em] text-primary-200">
            Smart outreach, sent as you
          </p>
          <h1
            id="brand-title"
            class="max-w-108 text-[clamp(2.4rem,4.5vw,4rem)] font-semibold leading-[1.04] tracking-[-0.055em]"
          >
            Personalized emails, delivered at scale.
          </h1>
          <p class="mt-6 max-w-100 text-base leading-7 text-primary-100">
            Connect via Google, add your recipients, and use custom templates to send individual,
            professional emails directly from your own account.
          </p>
        </div>
        <p class="m-0 text-xs text-primary-300">© {{ currentYear }} Mailforge</p>
      </section>
      <section class="grid place-items-center bg-muted p-5 sm:p-12 lg:p-16" aria-live="polite">
        <slot />
      </section>
    </div>
  </main>
</template>
