import { env } from '$env/dynamic/public';
import type { BuyerForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const buyersCreate = async (form: BuyerForm) =>
	await fetch(`${apiUrl}/buyers/create`, {
		body: JSON.stringify(form),
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		method: 'POST'
	});
