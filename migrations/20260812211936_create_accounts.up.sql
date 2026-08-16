CREATE TABLE IF NOT EXISTS public.accounts (
    user_id UUID PRIMARY KEY REFERENCES public.users(id),

    email TEXT NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,

    password_hash TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (email = lower(email))
);
