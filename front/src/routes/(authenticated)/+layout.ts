import { env } from '$env/dynamic/public';
import { error } from '@sveltejs/kit';
import type { LayoutLoad } from './$types';

const apiUrl = env.PUBLIC_API_URL;

export const load: LayoutLoad = async ({ fetch }) => {
	try {
		const [countriesRes, currenciesRes, taxesRes] = await Promise.all([
			fetch(`${apiUrl}/others/countries`),
			fetch(`${apiUrl}/others/currencies`),
			fetch(`${apiUrl}/others/taxes`)
		]);

		if (!countriesRes.ok) throw error(countriesRes.status, 'Failed to fetch countries');
		if (!currenciesRes.ok) throw error(currenciesRes.status, 'Failed to fetch currencies');
		if (!taxesRes.ok) throw error(taxesRes.status, 'Failed to fetch taxes');

		const [countries, currencies, taxes] = await Promise.all([
			countriesRes.json(),
			currenciesRes.json(),
			taxesRes.json()
		]);

		return { countries, currencies, taxes };
	} catch (err) {
		throw error(500, 'Failed to load layout data');
	}
};
