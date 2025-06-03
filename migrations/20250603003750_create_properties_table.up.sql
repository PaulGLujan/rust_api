CREATE TABLE properties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address TEXT NOT NULL,
    unit_number TEXT, -- Nullable, for apartments/units within an address
    current_rent_amount DECIMAL(10, 2) NOT NULL, -- Current monthly rent
    current_tenant_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX properties_address_idx ON properties (address);
CREATE INDEX properties_current_tenant_id_idx ON properties (current_tenant_id);
