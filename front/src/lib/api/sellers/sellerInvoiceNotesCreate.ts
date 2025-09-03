import { env } from '$env/dynamic/public';
import type { SellerForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const sellerInvoiceNotesCreate = async (form: SellerForm) =>
	await fetch(`${apiUrl}/sellers/invoice_notes/create`, {
		body: JSON.stringify(form),
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		method: 'POST'
	});
