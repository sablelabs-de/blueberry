CREATE TABLE IF NOT EXISTS public.sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    refresh_token TEXT NOT NULL,

    ip_address TEXT,
    country TEXT,
    region TEXT,
    city TEXT,

    user_agent TEXT,
    operating_system TEXT,
    platform TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days'),

    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
