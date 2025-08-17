CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE countries (
    country VARCHAR(60) PRIMARY KEY
);

CREATE TABLE currencies (
    currency VARCHAR(3) PRIMARY KEY,
    symbol VARCHAR(5) NOT NULL
);

CREATE TABLE taxes (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    rate DECIMAL(4,2) NOT NULL,
    tax VARCHAR(20) NOT NULL
);
CREATE UNIQUE INDEX idx_taxes_tax_rate ON taxes (tax, rate);

CREATE TABLE sellers (
    address VARCHAR(255) NOT NULL,
    bank_account VARCHAR(50) NOT NULL,
    city VARCHAR(60) NOT NULL,
    country VARCHAR(60) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(128) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    name VARCHAR(255) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    vat_number VARCHAR(20) NOT NULL
);
ALTER TABLE sellers 
    ADD CONSTRAINT fk_sellers_country 
    FOREIGN KEY (country) REFERENCES countries(country);

CREATE TABLE buyers (
    address VARCHAR(255) NOT NULL,
    city VARCHAR(60) NOT NULL,
    country VARCHAR(60) NOT NULL,
    email VARCHAR(255) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    name VARCHAR(255) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    seller_id UUID NOT NULL,
    vat_number VARCHAR(20) NOT NULL
);
ALTER TABLE buyers 
    ADD CONSTRAINT fk_buyers_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE buyers 
    ADD CONSTRAINT fk_buyers_country 
    FOREIGN KEY (country) REFERENCES countries(country);
ALTER TABLE buyers
    ADD CONSTRAINT uq_buyers_id_seller UNIQUE (id, seller_id);

CREATE TABLE vendors (
    address VARCHAR(255) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    name VARCHAR(255) NOT NULL,
    seller_id UUID NOT NULL,
    vat_number VARCHAR(20) NOT NULL
);
ALTER TABLE vendors 
    ADD CONSTRAINT fk_vendors_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE vendors
    ADD CONSTRAINT uq_vendors_id_seller UNIQUE (id, seller_id);

CREATE TABLE expense_categories (
    category VARCHAR(100) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    seller_id UUID NOT NULL
);
ALTER TABLE expense_categories 
    ADD CONSTRAINT fk_expense_categories_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE expense_categories
    ADD CONSTRAINT uq_expense_categories_id_seller UNIQUE (id, seller_id);

CREATE TABLE invoices (
    buyer_id UUID NOT NULL,
    currency VARCHAR(3) NOT NULL,
    due_date TIMESTAMP NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    issue_date TIMESTAMP NOT NULL,
    number SERIAL NOT NULL,
    seller_id UUID NOT NULL
);
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_buyer_seller
    FOREIGN KEY (buyer_id, seller_id)
    REFERENCES buyers(id, seller_id);
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_currency 
    FOREIGN KEY (currency) REFERENCES currencies(currency);
CREATE UNIQUE INDEX idx_invoices_number_seller ON invoices (number, seller_id);
ALTER TABLE invoices
    ADD CONSTRAINT uq_invoices_id_seller UNIQUE (id, seller_id);

CREATE TABLE invoice_details (
    description VARCHAR(255) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    invoice_id UUID NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    seller_id UUID NOT NULL,
    tax_id UUID NOT NULL,
    unit_price DECIMAL(12,2) NOT NULL
);
ALTER TABLE invoice_details 
    ADD CONSTRAINT fk_invoice_details_invoice 
    FOREIGN KEY (invoice_id, seller_id) REFERENCES invoices(id, seller_id) ON DELETE CASCADE;
ALTER TABLE invoice_details 
    ADD CONSTRAINT fk_invoice_details_tax 
    FOREIGN KEY (tax_id) REFERENCES taxes(id);

CREATE TABLE expenses (
    category_id UUID NOT NULL,
    currency VARCHAR(3) NOT NULL,
    description VARCHAR(255) NOT NULL,
    file VARCHAR(500) NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    issue_date TIMESTAMP NOT NULL,
    number VARCHAR(50) NOT NULL,
    seller_id UUID NOT NULL,
    vendor_id UUID NOT NULL
);
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_vendor 
    FOREIGN KEY (vendor_id, seller_id) REFERENCES vendors(id, seller_id) ON DELETE CASCADE;
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_currency 
    FOREIGN KEY (currency) REFERENCES currencies(currency);
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_category 
    FOREIGN KEY (category_id, seller_id) REFERENCES expense_categories(id, seller_id);
ALTER TABLE expenses
    ADD CONSTRAINT uq_expenses_id_seller UNIQUE (id, seller_id);

CREATE TABLE expense_details (
    description VARCHAR(255) NOT NULL,
    expense_id UUID NOT NULL,
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    seller_id UUID NOT NULL,
    tax_id UUID NOT NULL,
    unit_price DECIMAL(12,2) NOT NULL
);
ALTER TABLE expense_details 
    ADD CONSTRAINT fk_expense_details_expense 
    FOREIGN KEY (expense_id, seller_id) REFERENCES expenses(id, seller_id) ON DELETE CASCADE;
ALTER TABLE expense_details 
    ADD CONSTRAINT fk_expense_details_tax 
    FOREIGN KEY (tax_id) REFERENCES taxes(id);
