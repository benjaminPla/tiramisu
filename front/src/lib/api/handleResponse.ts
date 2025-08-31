import { goto } from '$app/navigation';

export async function handleResponse<T>(response: Response): Promise<T> {
	if (response.ok) return response.json() as Promise<T>;

	let message: string | undefined;
	try {
		message = await response.text();
	} catch {
		message = 'Unknown error';
	}

	switch (response.status) {
		case 401:
			goto('/login');
			throw new Error('Unauthorized');
		default:
			throw new Error(message || `HTTP ${response.status}`);
	}
}
