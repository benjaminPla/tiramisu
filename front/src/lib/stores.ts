import { env } from '$env/dynamic/public';
import { readable } from 'svelte/store';
import type { Country, Currency, Notification, Tax } from './types';
import { writable, type Writable } from 'svelte/store';

const apiUrl = env.PUBLIC_API_URL;

// @ts-ignore
export const countries = readable<Country[]>([], async (set) => {
	try {
		const res = await fetch(`${apiUrl}/others/countries`);
		if (!res.ok) throw new Error('Error getting countries');
		const data = await res.json();
		set(data);
	} catch (error: unknown) {
		notifications.error(error);
	}
});

// @ts-ignore
export const currencies = readable<Currency[]>([], async (set) => {
	try {
		const res = await fetch(`${apiUrl}/others/currencies`);
		if (!res.ok) throw new Error('Error getting currencies');
		const data = await res.json();
		set(data);
	} catch (error: unknown) {
		notifications.error(error);
	}
});

export class NotificationStore {
	private store: Writable<Notification[]>;
	private counter = 0;
	private loadingId: number | null = null;
	subscribe: Writable<Notification[]>['subscribe'];
	constructor() {
		this.store = writable<Notification[]>([]);
		this.subscribe = this.store.subscribe;
	}
	add(message: string, type: Notification['type']) {
		const id = ++this.counter;
		const notification: Notification = { id, message, type };
		this.store.update((all) => [...all, notification]);
		if (type !== 'loading') {
			setTimeout(() => this.remove(id), 3000);
		}
		return id;
	}
	error(e: unknown) {
		let message = 'Internal server error';
		if (typeof e === 'string') {
			message = e;
		} else if (e && typeof e === 'object' && 'message' in e) {
			message = (e as { message?: string }).message ?? message;
		}
		this.add(message, 'error');
	}
	loading(isLoading: boolean) {
		if (isLoading) {
			if (this.loadingId === null) {
				this.loadingId = this.add('Loading...', 'loading');
			}
		} else {
			if (this.loadingId !== null) {
				this.remove(this.loadingId);
				this.loadingId = null;
			}
		}
	}
	remove(id: number) {
		this.store.update((all) => all.filter((n) => n.id !== id));
		if (this.loadingId === id) {
			this.loadingId = null;
		}
	}
}
export const notifications = new NotificationStore();

// @ts-ignore
export const taxes = readable<Tax[]>([], async (set) => {
	try {
		const res = await fetch(`${apiUrl}/others/taxes`);
		if (!res.ok) throw new Error('Error getting taxes');
		const data = await res.json();
		set(data);
	} catch (error: unknown) {
		notifications.error(error);
	}
});
