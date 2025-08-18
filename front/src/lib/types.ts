export type Buyer = {
	address: string;
	city: string;
	country: string;
	email: string;
	name: string;
	postal_code: string;
	vat_number: string;
};

export type Currency = {
	currency: string;
	symbol: string;
};

export type InvoiceDetail = {
	description: string;
	quantity: number;
	tax_id: number;
	unit_price: number;
};

export type Invoice = {
	buyer_id: string;
	currency: string;
	details: InvoiceDetail[];
	issue_date: Date;
	due_date: Date;
};

export type Seller = {
	address: string;
	bank_account: string;
	city: string;
	country: string;
	email: string;
	name: string;
	postal_code: string;
	vat_number: string;
};

export type Tax = {
	id: number;
	rate: number;
	tax: string;
};
