import { env } from '$env/dynamic/public';
import type { Buyer } from '../../types';

const apiUrl = env.PUBLIC_API_URL;

export const createBuyer = async (form: Buyer) => {
	try {
		const res = await fetch(`${apiUrl}/buyers/create`, {
			body: JSON.stringify(form),
			credentials: 'include',
			headers: { 'Content-Type': 'application/json' },
			method: 'POST'
		});
		if (!res.ok) throw new Error('Create failed');
	} catch (error) {
		console.error(error);
	}
};
