import type { Notification } from './types';
import { writable, type Writable } from 'svelte/store';

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
