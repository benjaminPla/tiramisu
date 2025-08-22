import { apiFetch } from '$lib/api/apiFetch';

export const me = async () => await apiFetch('/authentication/me');
