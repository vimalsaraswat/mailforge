# Mailforge API

High-performance Rust backend for Mailforge, built with **Axum**, **Tokio**, **SQLx**, and **PostgreSQL**. It provides secure Google OAuth 2.0 authentication, session management, and template handling.

---

## Architecture & Layout

The project follows a clean, modular repository layout:

```text
src/
  clients/google/       Google OAuth client and provider models
  controllers/          HTTP handlers (health, auth, email templates)
  db/                   PostgreSQL connection pool & auto-migrations
  dto/                  Request/Response DTOs
  models/               SQLx domain models
  repositories/         Database access layer (users, sessions, accounts, templates)
  routes/               Modularized Axum route definitions
  services/             Application business logic & orchestration
  config.rs             Environment variables & configuration loader
  state.rs              Shared application state (AppState)
migrations/             Embedded PostgreSQL SQLx migrations
```

---

## Core Features

- **Robust Routing & State:** Built with Axum, utilizing modular sub-routers (`/auth`, `/templates`) and centralized AppState.
- **Database & Migrations:** Automated PostgreSQL connection pooling and embedded SQLx migrations executed on application startup.
- **Google OAuth 2.0 & PKCE:** Secure Google login flow handling token exchange, token refresh, and account linking.
- **Session & Cookie Management:** Secure session storage with HTTP-only cookies and automatic logout invalidation.
- **CORS Support:** Integrated `tower-http` CORS configuration supporting secure cross-origin requests from the Nuxt frontend.
- **Template Management:** Full CRUD operations for email templates.

---

## Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
2. Configure your local environment variables in `.env`:
   ```dotenv
   HOST=127.0.0.1
   PORT=3000
   DATABASE_URL=postgres://user:password@localhost:5432/mailforge
   GOOGLE_CLIENT_ID=your_client_id
   GOOGLE_CLIENT_SECRET=your_client_secret
   GOOGLE_REDIRECT_URI=http://localhost:3000/auth/google/callback
   FRONTEND_URL=http://localhost:8000
   ```

---

## Running Locally

To run the API development server from the `apps/api` directory:

```bash
cargo run
```

Verify the health check endpoint:

```bash
curl http://127.0.0.1:3000/health
```

To run compilation check without blocking target directories:

```bash
CARGO_TARGET_DIR=/tmp/mailforge-api-target cargo check
```

---

## Database Entities

- **`users`**: Stores user profiles and provider identity (`provider`, `provider_user_id`), ensuring unique and secure lookups.
- **`sessions`**: Manages opaque session tokens, ownership links, and expiration times with cascading deletes.
- **`mail_accounts`**: Stores provider mail credentials and OAuth token refresh data per user.
- **`email_templates`**: Stores user-created custom templates for outreach campaigns.

---

## Development Conventions

- **Controllers:** Handle HTTP inputs, cookies, headers, status codes, and DTO mappings.
- **Routes:** Modular sub-routers located in `src/routes/` are consolidated into the main router.
- **Repositories:** Isolate raw SQL queries and use `sqlx::FromRow` models.
- **Services:** Handle multi-step business transactions and orchestration across repositories.
- **Migrations:** Add new `.sql` migration files under `migrations/` for automatic execution on startup.
