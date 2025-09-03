import { env } from '$env/dynamic/public';

const apiUrl = env.PUBLIC_API_URL;

export const vendorsGetAll = async () =>
	await fetch(`${apiUrl}/vendors/get_all`, {
		credentials: 'include',
		method: 'GET'
	});

