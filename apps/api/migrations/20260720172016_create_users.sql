-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY,

    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,

    email TEXT NOT NULL UNIQUE,

    name TEXT NOT NULL,
    picture TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(provider, provider_user_id)
);
