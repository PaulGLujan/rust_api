CREATE TYPE payment_status AS ENUM ('pending', 'completed', 'failed');

CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    amount DECIMAL(10, 2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'USD',
    status payment_status NOT NULL DEFAULT 'pending',
    description TEXT,
    transaction_id TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMestamptz NOT NULL DEFAULT NOW()
);

CREATE INDEX payments_user_id_status_idx ON payments (user_id, status);
