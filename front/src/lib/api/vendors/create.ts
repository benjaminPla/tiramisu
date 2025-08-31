import { env } from '$env/dynamic/public';
import type { VendorForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const vendorsCreate = async (form: VendorForm) =>
	await fetch(`${apiUrl}/vendors/create`, {
		body: JSON.stringify(form),
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		method: 'POST'
	});
