DROP TABLE IF EXISTS expense_details CASCADE;
DROP TABLE IF EXISTS invoice_details CASCADE;

DROP TABLE IF EXISTS expenses CASCADE;
DROP TABLE IF EXISTS invoices CASCADE;

DROP TABLE IF EXISTS vendors CASCADE;
DROP TABLE IF EXISTS buyers CASCADE;
DROP TABLE IF EXISTS expense_categories CASCADE;

DROP TABLE IF EXISTS sellers CASCADE;

DROP TABLE IF EXISTS taxes CASCADE;
DROP TABLE IF EXISTS currencies CASCADE;
DROP TABLE IF EXISTS countries CASCADE;

DROP INDEX IF EXISTS idx_taxes_tax_rate;
DROP INDEX IF EXISTS idx_invoices_number_seller;
DROP INDEX IF EXISTS idx_invoice_details_invoice_seller;
DROP INDEX IF EXISTS idx_expense_details_expense_seller;
DROP INDEX IF EXISTS idx_expenses_vendor_seller;
DROP INDEX IF EXISTS idx_expenses_category_seller;

-- DROP EXTENSION IF EXISTS "uuid-ossp";
