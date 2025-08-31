import { env } from '$env/dynamic/public';
import type { InvoiceForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const invoicesCreate = async (form: InvoiceForm) =>
	await fetch(`${apiUrl}/invoices/create`, {
		body: JSON.stringify(form),
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		method: 'POST'
	});
