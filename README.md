# Mailforge

Mailforge is a modern full-stack mail outreach application designed to send personalized emails at scale directly from your own Google account. It features a high-performance **Rust (Axum)** backend and a reactive **Nuxt 4 / Nuxt UI** frontend.

---

## Project Structure

This project is organized as a monorepo containing two main packages:

```text
mailforge/
  apps/
    api/     # Rust backend (Axum, Tokio, SQLx, PostgreSQL)
    web/     # Nuxt 4 frontend (Nuxt UI, Tailwind CSS, TypeScript)
  docs/      # Architecture decisions and implementation context
```

---

## Tech Stack

### Backend (`apps/api`)
- **Language:** Rust (Edition 2021)
- **Framework:** Axum & Tokio
- **Database:** PostgreSQL with SQLx (automated startup migrations)
- **Authentication:** Google OAuth 2.0 with PKCE & session cookies

### Frontend (`apps/web`)
- **Framework:** Nuxt 4
- **UI Library:** Nuxt UI & Tailwind CSS
- **Language:** TypeScript
- **State & Data Fetching:** Nuxt Composables & `$fetch`

---

## Prerequisites

Ensure you have the following installed on your machine:
- **Rust & Cargo** (latest stable)
- **Node.js** (v18+) & **pnpm**
- **PostgreSQL** (running locally or via Docker)

---

## Getting Started

### 1. Database Setup
Create a PostgreSQL database named `mailforge`:
```sql
CREATE DATABASE mailforge;
```

### 2. Configure Environment Variables
Set up environment configurations for both apps:

- **Backend (`apps/api/.env`):**
  ```dotenv
  HOST=127.0.0.1
  PORT=3000
  DATABASE_URL=postgres://user:password@localhost:5432/mailforge
  GOOGLE_CLIENT_ID=your_google_client_id
  GOOGLE_CLIENT_SECRET=your_google_client_secret
  GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback
  FRONTEND_URL=http://localhost:8000
  ```

- **Frontend (`apps/web/.env`):**
  ```dotenv
  NUXT_PUBLIC_API_BASE=http://localhost:3000
  ```

---

## Running the Application

### Start the Backend API
Navigate to `apps/api` and run the Rust server:
```bash
cd apps/api
cargo run
```
*The API will start at `http://localhost:3000`, automatically connect to PostgreSQL, and run embedded database migrations.*

### Start the Web Frontend
In a separate terminal, navigate to `apps/web`, install dependencies, and start the development server:
```bash
cd apps/web
pnpm install
pnpm run dev
```
*The web frontend will start at `http://localhost:8000`.*

---

## Documentation

For deeper technical context, implementation notes, and architectural decisions, refer to the package-specific documentation:
- [API README](apps/api/README.md)
- [Web README](apps/web/README.md)
- [Implementation Docs](apps/api/docs/README.md)
