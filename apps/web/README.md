# Mailforge Web Client

The frontend web application for Mailforge, built with **Nuxt 4**, **Nuxt UI**, **Tailwind CSS**, and **TypeScript**. It provides a sleek dashboard, Google OAuth authentication flow, and template management tools.

---

## Architecture & Layout

The project follows the standard Nuxt 4 directory structure:

```text
app/
  assets/css/           Global styles and Tailwind setup
  components/           Vue components (auth cards, layout shells, etc.)
  composables/          Auto-imported Vue composables (useAuth, useAPI, useEmailTemplates)
  pages/                File-based routing pages (index.vue, dashboard.vue)
  services/             API route path mappings and helpers
  types/                TypeScript interfaces and types
nuxt.config.ts          Nuxt application and runtime configurations
```

---

## Core Features

- **Nuxt 4 & Nuxt UI:** Modern component library integration with customized theme tokens and responsive navigation menus.
- **Authentication:** Secure Google OAuth sign-in flow (`useAuth`) backed by credentials-included session handling.
- **API Integration:** Custom `useAPI` composable and `$api` utility wrapping fetch requests with automatic `baseURL` and cookie credentials management.
- **Dashboard & Sidebar Layout:** Responsive sidebar navigation with user profile management and logout actions.
- **Template Management:** Interactive email template manager component.

---

## Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
2. Configure your environment variables in `.env`:
   ```dotenv
   NUXT_PUBLIC_API_BASE=http://localhost:3000
   ```

---

## Running Locally

From the `apps/web` directory, install dependencies and start the development server:

```bash
# Install dependencies (if not already done at the root workspace level)
pnpm install

# Run the Nuxt dev server (runs on port 8000 by default)
pnpm run dev
```

Open your browser and navigate to `http://localhost:8000`.

---

## Development Conventions

- **Composables (`app/composables/`):** Keep reusable stateful logic here; they are auto-imported across the project.
- **Pages (`app/pages/`):** File-based routes (`index.vue` acts as the auth/landing gate, `dashboard.vue` provides the main app interface).
- **API Endpoints (`app/api/` & `app/services/`):** Centralize route definitions and request mappings to keep components decoupled from raw URLs.
- **Styling:** Use Tailwind CSS classes combined with Nuxt UI semantic components (`UCard`, `USkeleton`, `UNavigationMenu`, `UAvatar`, etc.).
