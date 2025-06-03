-- migrations/<timestamp>_alter_payments_for_rentals.down.sql (DOWN migration)
ALTER TABLE payments
DROP COLUMN property_id;

ALTER TABLE payments
DROP COLUMN due_date;

ALTER TABLE payments
DROP COLUMN period_start;
ALTER TABLE payments
DROP COLUMN period_end;

ALTER TABLE payments
RENAME COLUMN notes TO description;
