import { apiFetch } from '$lib/api/apiFetch';
import { readable } from 'svelte/store';
import type { Country, Currency, Notification, NotificationType, Seller } from './types';
import { writable } from 'svelte/store';

// @ts-ignore
export const countries = readable<Country[] | null>([], async (set) => {
	const data = await apiFetch<Country[]>('/others/countries', {}, false);
	set(data);
});

// @ts-ignore
export const currencies = readable<Currency[] | null>([], async (set) => {
	const data = await apiFetch<Currency[]>('/others/currencies', {}, false);
	set(data);
});

export const notification = writable<Notification | null>(null);
let notificationTimeout: number | null = null;
export const showNotification = (message: string | null, type: NotificationType) => {
	if (notificationTimeout) clearTimeout(notificationTimeout);
	notification.set({ message: message || 'Internal server error', type, isHided: false });
	if (type !== 'loading') notificationTimeout = setTimeout(() => notification.set(null), 3000);
};

export const loading = writable<boolean>(false);
loading.subscribe((isLoading) => {
	isLoading
		? showNotification('Loading...', 'loading')
		: notification.update((n) => (n?.type === 'loading' ? null : n));
});

const _seller = writable<Seller | null>();
export const seller = { subscribe: _seller.subscribe };
export async function loadSeller() {
	const data = await apiFetch<Seller>('/authentication/me');
	_seller.set(data);
}
export async function updateSeller(form: Seller) {
	const data = await apiFetch<Seller>(
		'/sellers/update',
		{
			body: JSON.stringify(form),
			method: 'PUT'
		},
		true,
		true
	);
	_seller.set(data);
}
