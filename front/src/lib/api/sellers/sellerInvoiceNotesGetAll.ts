import { env } from '$env/dynamic/public';

const apiUrl = env.PUBLIC_API_URL;

export const sellerInvoiceNotesGetAll = async () =>
	await fetch(`${apiUrl}/sellers/invoice_notes/get_all`, {
		credentials: 'include',
		method: 'GET'
	});
