-- Add a foreign key to the properties table
ALTER TABLE payments
ADD COLUMN property_id UUID REFERENCES properties(id);

-- Add a due date for the payment
ALTER TABLE payments
ADD COLUMN due_date DATE NULL; 

-- Add the period the rent payment covers
ALTER TABLE payments
ADD COLUMN period_start DATE;
ALTER TABLE payments
ADD COLUMN period_end DATE;

ALTER TYPE payment_status ADD VALUE 'overdue';
ALTER TYPE payment_status ADD VALUE 'partially_paid';

-- Rename description to notes for clarity
ALTER TABLE payments
RENAME COLUMN description TO notes;

-- Add an index for looking up payments by property
CREATE INDEX payments_property_id_idx ON payments (property_id);
