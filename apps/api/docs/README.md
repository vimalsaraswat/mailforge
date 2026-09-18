# API implementation context

This document is a snapshot of the API as explored on 2026-07-24. Treat it as working context for future implementation sessions; verify it against the source when behavior changes.

## Intended authentication flow

The requested product flow is:

1. The Nuxt frontend sends the user to an API login route.
2. The API creates a Google authorization URL with CSRF state and PKCE, then redirects to Google.
3. Google redirects to an API callback with an authorization code and state.
4. The API validates state, exchanges the code with PKCE, and obtains Google tokens.
5. The API retrieves the Google profile.
6. The API creates or updates the `User` record.
7. The API creates or updates the `MailAccount` record.
8. The API creates a `Session` record.
9. The API sets an opaque, preferably `HttpOnly` and `Secure`, session cookie.
10. The API redirects to the Nuxt frontend.
11. Protected routes resolve the cookie to a non-expired session and load its user.

Steps 1–11 are now wired. The implementation still needs production hardening, transactional persistence, and integration tests.

## Existing modules

### Application startup

`src/main.rs` loads `Config`, connects to PostgreSQL, runs embedded migrations, constructs `AppState`, builds the Axum router, and binds the listener. `AppState` currently contains only `Config` and `PgPool`.

`src/config.rs` loads `.env` with `dotenvy` and requires `HOST`, `PORT`, and `DATABASE_URL`. It does not yet contain Google credentials, a frontend URL, cookie settings, or session lifetime.

### Routing and HTTP

`src/router.rs` currently registers only:

```text
GET /health -> controllers::health::health
```

`src/controllers/auth.rs` implements Google login/callback, `/auth/me`, and logout as thin HTTP handlers. `src/services/auth.rs` owns Google token/profile orchestration, user and mail-account upserts, session creation, current-user resolution, and logout operations. `src/dto/auth.rs` provides the serialized `MeResponse` returned by `/auth/me`.

### Google OAuth

`src/clients/google/oauth.rs` contains `GoogleOAuthClient` with:

- Google authorization and token endpoints.
- OpenID, email, profile, and `gmail.send` scopes.
- Random CSRF token generation.
- SHA-256 PKCE challenge/verifier generation.
- Authorization-code exchange.
- Refresh-token exchange.

`GoogleToken` contains an access token, optional refresh token, and optional expiry. `GoogleUserInfo` is only a Rust data shape; there is no HTTP call to Google’s user-info endpoint yet, and it does not currently derive `Deserialize`.

The current implementation carries the CSRF state and PKCE verifier in a short-lived `HttpOnly` OAuth flow cookie, then clears it after the callback. This avoids shared mutex state for now. Before production, consider a signed/encrypted or server-side flow-state mechanism so the verifier is not client-readable metadata and so cookie handling can be centrally revoked.

### Persistence

The repositories are:

- `UserRepository`: find by UUID, find by provider identity, create, and update profile fields.
- `MailAccountRepository`: find by provider/account, list by user, create, and update tokens.
- `SessionRepository`: find, create, delete one, and delete all for a user.

Controllers must not call repositories directly. New authentication or account behavior belongs in a service, with repositories limited to persistence queries and commands.

## Current layer boundaries

The authentication flow is currently organized as follows:

```text
controllers/auth.rs
  -> controllers/cookies.rs + controllers/errors.rs + dto/auth.rs
  -> services/auth.rs
  -> services/ports.rs
  -> repositories/* and clients/google/*
```

`main.rs` is the composition root. It constructs the concrete PostgreSQL repositories and Google provider, then injects them into the generic `AuthService`. The service no longer constructs infrastructure dependencies, and controllers no longer call repositories directly.

Application errors live in `services/errors.rs`; these are not HTTP errors. HTTP mapping lives in `controllers/errors.rs`. HTTP DTOs live under `dto/` and are mapped to/from application results at the controller boundary.

The infrastructure implementations of the repository/provider ports are in `infrastructure/adapters.rs`. The next boundary improvement is to split SQLx row models from domain entities and replace the current concrete `models/*` types, which still derive `sqlx::FromRow`.

The SQL migrations create:

- `users` with provider identity uniqueness and unique email.
- `sessions` linked to users with `ON DELETE CASCADE`.
- `mail_accounts` linked to users with `ON DELETE CASCADE` and provider/account uniqueness.

## Important implementation checks

These are observations, not completed fixes:

- `MailAccountRepository::find_by_id` accepts `i64`, but `MailAccount.id` and the migration column are UUIDs. Change the repository parameter to `Uuid` before using this method.
- `MailAccount.refresh_token` is required in both the model and SQL schema, while Google may return no refresh token on a token exchange. Decide whether account creation should require one or whether the schema/model should allow it.
- The OAuth client currently requests `gmail.send`, which is a sensitive Google scope. Confirm the minimum required scopes and production verification requirements before launch.
- Token values are stored as plain text. Add encryption or an equivalent secret-management strategy before production.
- `/auth/me` checks `expires_at > NOW()` before returning the user; other future session consumers should apply the same rule.
- The session cookie should contain only an opaque session ID, not user data or provider tokens. Configure `HttpOnly`, `Secure` in production, `SameSite`, `Path`, and `Max-Age` explicitly.
- The callback must validate the OAuth state before exchanging the code and must handle Google errors and missing parameters without leaking secrets.
- The auth flow reads Google client ID/secret, OAuth callback URL, frontend redirect URL, session duration, and cookie security from configuration.
- If the Nuxt app runs on another origin, configure narrowly scoped CORS and credentialed requests; do not use a permissive production wildcard with cookies.

## Suggested next implementation boundary

Remaining hardening work:

1. Move auth persistence into a service and use a database transaction where practical.
2. Add an auth extractor/middleware for additional protected routes.
3. Replace the development OAuth flow cookie with signed/encrypted or server-side state.
4. Add integration tests covering callback failures, expired sessions, logout, and cookie attributes.
