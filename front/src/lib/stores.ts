import { apiFetch } from '$lib/api/apiFetch';
import { readable } from 'svelte/store';
import type { Country, Currency, Notification, NotificationType, Seller } from './types';
import { writable } from 'svelte/store';

// @ts-ignore
export const countries = readable<Country[]>([], async (set) => {
	const data = await apiFetch('/others/countries', {}, false);
	set(data);
});

// @ts-ignore
export const currencies = readable<Currency[]>([], async (set) => {
	const data = await apiFetch('/others/currencies', {}, false);
	set(data);
});

export const notification = writable<Notification | null>(null);
let notificationTimeout: number | null = null;
export const showNotification = (message: string | null, type: NotificationType) => {
	if (notificationTimeout) clearTimeout(notificationTimeout);
	notification.set({ message: message || 'Internal server error', type, isHided: false });
	notificationTimeout = setTimeout(() => notification.set(null), 3000);
};

const _seller = writable<Seller>();
export const seller = { subscribe: _seller.subscribe };
export async function loadSeller() {
	const data = await apiFetch('/authentication/me');
	_seller.set(data);
}
export async function updateSeller(form: Seller) {
	const data = await apiFetch('/sellers/update', {
		body: JSON.stringify(form),
		method: 'PUT'
	});
	_seller.set(data);
	showNotification('Update success', 'success');
}
