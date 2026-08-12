CREATE TABLE IF NOT EXISTS public.sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    refresh_selector_hash BYTEA NOT NULL UNIQUE,
    refresh_validator_hash BYTEA NOT NULL,
    -- Counter of token refreshes.
    refresh_generation INTEGER NOT NULL DEFAULT 0,

    ip_address INET,
    country TEXT,
    region TEXT,
    city TEXT,

    user_agent TEXT,
    operating_system TEXT,
    platform TEXT,

    idle_expires_at TIMESTAMPTZ NOT NULL,
    absolute_expires_at TIMESTAMPTZ NOT NULL,

    revoked_at TIMESTAMPTZ,
    revocation_reason TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
