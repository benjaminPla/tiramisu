import { env } from '$env/dynamic/public';
import { goto } from '$app/navigation';
import { showNotification } from '$lib/stores';

const apiUrl = env.PUBLIC_API_URL;

export const apiFetch = async (url: string, options: RequestInit = {}, withAuth = true) => {
	const res = await fetch(`${apiUrl}${url}`, {
		credentials: withAuth ? 'include' : 'omit',
		headers: { 'Content-Type': 'application/json', ...(options.headers || {}) },
		...options
	});

	if (!res.ok) {
		switch (res.status) {
			case 401:
				showNotification('Session expired, please log in', 'error');
				goto('/login');
				throw new Error('Unauthorized');
			default:
				const text = await res.text();
				throw new Error(text || 'API request failed');
		}
	}

	return res.json();
};
