CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--  https://dbdiagram.io/d
--  Table sellers {
  --  address varchar [not null]
  --  bank_account varchar [not null]
  --  city varchar [not null]
  --  country varchar [not null, ref: > countries.country]
  --  email varchar [not null, unique]
  --  hashed_password varchar [not null]
  --  id integer [primary key]
  --  is_admin boolean [not null]
  --  name varchar [not null]
  --  postal_code varchar [not null]
  --  public_id uuid [not null, unique]
  --  vat_number varchar(20) [not null]
--  }

--  Table countries {
  --  country varchar [primary key]
--  }

--  Table currencies {
  --  currency varchar(3) [primary key]
  --  symbol varchar(1) [not null]
--  }

--  Table taxes {
  --  id integer [primary key]
  --  tax varchar(10) [not null]
  --  rate decimal [not null]
  --  indexes {
    --  (tax, rate) [unique]
  --  }
--  }

--  Table expense_categories {
  --  category varchar [not null]
  --  id integer [primary key]
  --  seller_id integer [not null, ref: > sellers.id]
--  }

--  Table buyers {
  --  address varchar [not null]
  --  city varchar [not null]
  --  country varchar [not null, ref: > countries.country]
  --  email varchar [not null]
  --  id integer [primary key]
  --  name varchar [not null]
  --  postal_code varchar [not null]
  --  public_id uuid [not null, unique]
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
  --  description varchar [not null]
  --  id integer [primary key]
  --  invoice_id integer [not null, ref: > invoices.id]
  --  quantity integer [not null, default: 1]
  --  tax_id integer [not null, ref: > taxes.id]
  --  unit_price decimal [not null]
--  }

--  Table expenses {
  --  category_id integer [not null, ref: > expense_categories.id]
  --  currency varchar(3) [not null, ref: > currencies.currency]
  --  description varchar [not null]
  --  id integer [primary key]
  --  issue_date timestamp [not null]
  --  number varchar [not null]
  --  file varchar [not null]
  --  seller_id integer [not null, ref: > sellers.id]
  --  vendor_id integer [not null, ref: > vendors.id]
--  }

--  Table expense_details {
  --  description varchar [not null]
  --  expense_id integer [not null, ref: > expenses.id]
  --  id integer [primary key]
  --  quantity integer [not null, default: 1]
  --  tax_id integer [not null, ref: > taxes.id]
  --  unit_price decimal [not null]
--  }

--  Table vendors {
  --  address varchar [not null]
  --  id integer [primary key]
  --  name varchar [not null]
  --  seller_id integer [not null, ref: > sellers.id]
  --  vat_number varchar(20) [not null]
--  }
