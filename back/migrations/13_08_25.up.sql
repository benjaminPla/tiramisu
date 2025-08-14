CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE sellers (
    address VARCHAR(255) NOT NULL,
    bank_account VARCHAR(50) NOT NULL,
    city VARCHAR(60) NOT NULL,
    country VARCHAR(60) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(128) NOT NULL,
    id SERIAL PRIMARY KEY,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
    name VARCHAR(255) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    public_id UUID NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    vat_number VARCHAR(20) NOT NULL,
);
ALTER TABLE sellers 
    ADD CONSTRAINT fk_sellers_country 
    FOREIGN KEY (country) REFERENCES countries(country);

CREATE TABLE buyers (
    address VARCHAR(255) NOT NULL,
    city VARCHAR(60) NOT NULL,
    country VARCHAR(60) NOT NULL,
    email VARCHAR(255) NOT NULL,
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    postal_code VARCHAR(20) NOT NULL,
    public_id UUID NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    seller_id INTEGER NOT NULL
    vat_number VARCHAR(20) NOT NULL,
);
ALTER TABLE buyers 
    ADD CONSTRAINT fk_buyers_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE buyers 
    ADD CONSTRAINT fk_buyers_country 
    FOREIGN KEY (country) REFERENCES countries(country);

CREATE TABLE vendors (
    address VARCHAR(255) NOT NULL,
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    seller_id INTEGER NOT NULL
    vat_number VARCHAR(20) NOT NULL,
);
ALTER TABLE vendors 
    ADD CONSTRAINT fk_vendors_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;

CREATE TABLE invoices (
    buyer_id INTEGER NOT NULL
    currency VARCHAR(3) NOT NULL,
    due_date TIMESTAMP NOT NULL,
    id SERIAL PRIMARY KEY,
    issue_date TIMESTAMP NOT NULL,
    number INTEGER NOT NULL,
    public_id UUID NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
    seller_id INTEGER NOT NULL,
);
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_buyer 
    FOREIGN KEY (buyer_id) REFERENCES buyers(id) ON DELETE CASCADE;
ALTER TABLE invoices 
    ADD CONSTRAINT fk_invoices_currency 
    FOREIGN KEY (currency) REFERENCES currencies(currency);
CREATE UNIQUE INDEX idx_invoices_number_seller ON invoices (number, seller_id);

CREATE TABLE invoice_details (
    description VARCHAR(255) NOT NULL,
    id SERIAL PRIMARY KEY,
    invoice_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    tax_id INTEGER NOT NULL
    unit_price DECIMAL(12,2) NOT NULL,
);
ALTER TABLE invoice_details 
    ADD CONSTRAINT fk_invoice_details_invoice 
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE;
ALTER TABLE invoice_details 
    ADD CONSTRAINT fk_invoice_details_tax 
    FOREIGN KEY (tax_id) REFERENCES taxes(id);

CREATE TABLE expenses (
    category_id INTEGER NOT NULL
    currency VARCHAR(3) NOT NULL,
    description VARCHAR(255) NOT NULL,
    file VARCHAR(500) NOT NULL,
    id SERIAL PRIMARY KEY,
    issue_date TIMESTAMP NOT NULL,
    number VARCHAR(50) NOT NULL,
    seller_id INTEGER NOT NULL,
    vendor_id INTEGER NOT NULL,
);
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_vendor 
    FOREIGN KEY (vendor_id) REFERENCES vendors(id) ON DELETE CASCADE;
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_currency 
    FOREIGN KEY (currency) REFERENCES currencies(currency);
ALTER TABLE expenses 
    ADD CONSTRAINT fk_expenses_category 
    FOREIGN KEY (category_id) REFERENCES expense_categories(id);

CREATE TABLE expense_details (
    description VARCHAR(255) NOT NULL,
    expense_id INTEGER NOT NULL,
    id SERIAL PRIMARY KEY,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    tax_id INTEGER NOT NULL
    unit_price DECIMAL(12,2) NOT NULL,
);
ALTER TABLE expense_details 
    ADD CONSTRAINT fk_expense_details_expense 
    FOREIGN KEY (expense_id) REFERENCES expenses(id) ON DELETE CASCADE;
ALTER TABLE expense_details 
    ADD CONSTRAINT fk_expense_details_tax 
    FOREIGN KEY (tax_id) REFERENCES taxes(id);

CREATE TABLE expense_categories (
    category VARCHAR(100) NOT NULL,
    id SERIAL PRIMARY KEY,
    seller_id INTEGER NOT NULL
);
ALTER TABLE expense_categories 
    ADD CONSTRAINT fk_expense_categories_seller 
    FOREIGN KEY (seller_id) REFERENCES sellers(id) ON DELETE CASCADE;

CREATE TABLE countries (
    country VARCHAR(60) PRIMARY KEY
);

CREATE TABLE currencies (
    currency VARCHAR(3) PRIMARY KEY,
    symbol VARCHAR(5) NOT NULL
);

CREATE TABLE taxes (
    id SERIAL PRIMARY KEY,
    rate DECIMAL(4,2) NOT NULL
    tax VARCHAR(20) NOT NULL,
);
CREATE UNIQUE INDEX idx_taxes_tax_rate ON taxes (tax, rate);

--  https://dbdiagram.io/d
--  Table sellers {
  --  address varchar(255) [not null]
  --  bank_account varchar(50) [not null]
  --  city varchar(60) [not null]
  --  country varchar(60) [not null, ref: > countries.country]
  --  email varchar(255) [not null, unique]
  --  hashed_password varchar(128) [not null]
  --  id integer [primary key]
  --  is_admin boolean [not null]
  --  name varchar(255) [not null]
  --  postal_code varchar(20) [not null]
  --  public_id uuid [not null, unique]
  --  vat_number varchar(20) [not null]
--  }

--  Table buyers {
  --  address varchar(255) [not null]
  --  city varchar(60) [not null]
  --  country varchar(60) [not null, ref: > countries.country]
  --  email varchar(255) [not null]
  --  id integer [primary key]
  --  name varchar(255) [not null]
  --  postal_code varchar(20) [not null]
  --  public_id uuid [not null, unique]
  --  seller_id integer [not null, ref: > sellers.id]
  --  vat_number varchar(20) [not null]
--  }

--  Table vendors {
  --  address varchar(255) [not null]
  --  id integer [primary key]
  --  name varchar(255) [not null]
  --  seller_id integer [not null, ref: > sellers.id]
  --  vat_number varchar(20) [not null]
--  }

--  Table invoices {
  --  buyer_id integer [not null, ref: > buyers.id]
  --  currency varchar(3) [not null, ref: > currencies.currency]
  --  due_date timestamp [not null]
  --  id integer [primary key]
  --  issue_date timestamp [not null]
  --  number integer [not null]
  --  public_id uuid [not null, unique]
  --  seller_id integer [not null, ref: > sellers.id]
  --  indexes {
    --  (number, seller_id) [unique]
  --  }
--  }

--  Table invoice_details {
  --  description varchar(255) [not null]
  --  id integer [primary key]
  --  invoice_id integer [not null, ref: > invoices.id]
  --  quantity integer [not null, default: 1]
  --  tax_id integer [not null, ref: > taxes.id]
  --  unit_price decimal(12,2) [not null]
--  }

--  Table expenses {
  --  category_id integer [not null, ref: > expense_categories.id]
  --  currency varchar(3) [not null, ref: > currencies.currency]
  --  description varchar(255) [not null]
  --  file varchar(500) [not null]
  --  id integer [primary key]
  --  issue_date timestamp [not null]
  --  number varchar(50) [not null]
  --  seller_id integer [not null, ref: > sellers.id]
  --  vendor_id integer [not null, ref: > vendors.id]
--  }

--  Table expense_details {
  --  description varchar(255) [not null]
  --  expense_id integer [not null, ref: > expenses.id]
  --  id integer [primary key]
  --  quantity integer [not null, default: 1]
  --  tax_id integer [not null, ref: > taxes.id]
  --  unit_price decimal(12,2) [not null]
--  }

--  Table expense_categories {
  --  category varchar [not null]
  --  id integer [primary key]
  --  seller_id integer [not null, ref: > sellers.id]
--  }

--  Table countries {
  --  country varchar(60) [primary key]
--  }

--  Table currencies {
  --  currency varchar(3) [primary key]
  --  symbol varchar(5) [not null]
--  }

--  Table taxes {
  --  id integer [primary key]
  --  tax varchar(20) [not null]
  --  rate decimal(4,2) [not null]
  --  indexes {
    --  (tax, rate) [unique]
  --  }
--  }
