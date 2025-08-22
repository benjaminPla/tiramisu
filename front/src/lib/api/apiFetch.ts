import { env } from '$env/dynamic/public';
import { goto } from '$app/navigation';
import { showNotification } from '$lib/stores';

const apiUrl = env.PUBLIC_API_URL;

export const apiFetch = async (url: string, options: RequestInit = {}, withAuth = true) => {
	try {
		const res = await fetch(`${apiUrl}${url}`, {
			credentials: withAuth ? 'include' : 'omit',
			headers: { 'Content-Type': 'application/json', ...(options.headers || {}) },
			...options
		});

		if (!res.ok) {
			const text = (await res.text()) || 'Something went wrong';
			switch (res.status) {
				case 401:
					showNotification(text, 'error');
					goto('/login');
					return null;
				default:
					showNotification(text, 'error');
					return null;
			}
		}

		return res.json();
	} catch (error) {
		showNotification('Internal server error', 'error');
		return null;
	}
};
