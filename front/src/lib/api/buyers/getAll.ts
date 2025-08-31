import { env } from '$env/dynamic/public';

const apiUrl = env.PUBLIC_API_URL;

export const buyersGetAll = async () =>
	await fetch(`${apiUrl}/buyers/get_all`, {
		credentials: 'include',
		method: 'GET'
	});
