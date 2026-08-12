CREATE TABLE IF NOT EXISTS public.users (
    id UUID PRIMARY KEY REFERENCES public.accounts(id),

    username TEXT NOT NULL UNIQUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT username_length CHECK (char_length(username) BETWEEN 2 AND 32),
    CHECK (username = lower(username))
);
