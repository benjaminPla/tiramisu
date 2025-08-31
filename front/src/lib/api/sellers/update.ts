import { env } from '$env/dynamic/public';
import type { SellerForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const sellerUpdate = async (form: SellerForm) =>
	await fetch(`${apiUrl}/sellers/update`, {
		body: JSON.stringify(form),
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		method: 'PUT'
	});
