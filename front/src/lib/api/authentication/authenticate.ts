import { env } from '$env/dynamic/public';
import { goto } from '$app/navigation';
import { loading, showNotification } from '$lib/stores';
import type { AuthenticateForm } from '$lib/types';

const apiUrl = env.PUBLIC_API_URL;

export const authenticate = async (form: AuthenticateForm) => {
	loading.set(true);
	try {
		const res = await fetch(`${apiUrl}/authentication/authenticate`, {
			body: JSON.stringify(form),
			credentials: 'include',
			headers: { 'Content-Type': 'application/json' },
			method: 'POST'
		});
		if (!res.ok) {
			const text = (await res.text()) || 'Something went wrong';
			showNotification(text, 'error');
			return;
		}
		goto('/dashboard');
	} catch (error) {
		showNotification('Internal server error', 'error');
	} finally {
		loading.set(false);
	}
};
