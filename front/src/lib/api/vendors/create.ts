import { apiFetch } from '$lib/api/apiFetch';
import type { Vendor } from '../../types';

export const createVendor = async (form: Vendor) =>
	await apiFetch('/vendors/create', {
		body: JSON.stringify(form),
		method: 'POST'
	});
