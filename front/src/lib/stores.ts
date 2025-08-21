import { env } from '$env/dynamic/public';
import { writable } from 'svelte/store';
import { readable } from 'svelte/store';
import type { Country, Currency, Seller } from './types';

const apiUrl = env.PUBLIC_API_URL;

// @ts-ignore
export const countries = readable<Country[]>([], async (set) => {
	try {
		const res = await fetch(`${apiUrl}/others/countries`, {
			headers: { 'Content-Type': 'application/json' }
		});
		if (!res.ok) throw new Error('Failed to load countries');
		const data: Country[] = await res.json();
		set(data);
	} catch (error) {
		console.error(error);
	}
});

// @ts-ignore
export const currencies = readable<Currency[]>([], async (set) => {
	try {
		const res = await fetch(`${apiUrl}/others/currencies`, {
			headers: { 'Content-Type': 'application/json' }
		});
		if (!res.ok) throw new Error('Failed to load currencies');
		const data: Currency[] = await res.json();
		set(data);
	} catch (error) {
		console.error(error);
	}
});

const _seller = writable<Seller | null>(null);
export const seller = { subscribe: _seller.subscribe };
export async function loadSeller() {
	try {
		const res = await fetch(`${env.PUBLIC_API_URL}/authentication/me`, {
			credentials: 'include',
			headers: { 'Content-Type': 'application/json' }
		});
		if (!res.ok) throw new Error('Failed to load seller');
		const data: Seller = await res.json();
		_seller.set(data);
	} catch (error) {
		console.error(error);
	}
}
export async function updateSeller(form: Seller) {
	try {
		const res = await fetch(`${env.PUBLIC_API_URL}/sellers/update`, {
			body: JSON.stringify(form),
			credentials: 'include',
			headers: { 'Content-Type': 'application/json' },
			method: 'PUT'
		});
		if (!res.ok) throw new Error('Update failed');
		_seller.set(form);
	} catch (error) {
		console.error(error);
	}
}
