export type AuthenticateForm = {
	email: string;
	password: string;
};

export type Buyer = BuyerForm & {
	id: string;
};

export type BuyerForm = {
	address: string;
	city: string;
	country: string;
	email: string;
	name: string;
	postal_code: string;
	vat_number: string;
};

export type Country = string;

export type Currency = {
	currency: string;
	symbol: string;
};

export type InvoiceCreateResponse = InvoiceDetail & {
	invoice_details: InvoiceDetail[];
	invoice: { id: string; number: number; seller_id: string };
	invoice_notes: SellerInvoiceNote[];
};

export type InvoiceDetail = {
	description: string;
	quantity: number;
	tax_id: number;
	unit_price: number;
};

export type InvoiceForm = {
	buyer_id: string;
	currency: string;
	details: InvoiceDetail[];
	due_date: string;
	issue_date: string;
    notes: string[]
};

export type Notification = {
	id: number;
	message: string;
	type: NotificationType;
};

export type NotificationType = 'error' | 'info' | 'loading' | 'success';

export type Seller = SellerForm & {
	id: string;
};

export type SellerForm = {
	address: string;
	bank_account: string;
	city: string;
	country: string;
	email: string;
	name: string;
	postal_code: string;
	vat_number: string;
};

export type SellerInvoiceNote = {
	id: string;
	note: string;
	seller_id: string;
};

export type Tax = {
	id: number;
	rate: number;
	tax: string;
};

export type VendorForm = {
	address: string;
	name: string;
	seller_id: string;
	vat_number: string;
};
