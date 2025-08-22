import { apiFetch } from '$lib/api/apiFetch';
import type { Buyer } from '../../types';

export const createBuyer = async (form: Buyer) => {
	await apiFetch('/buyers/create', {
		body: JSON.stringify(form),
		method: 'POST'
	});
};
