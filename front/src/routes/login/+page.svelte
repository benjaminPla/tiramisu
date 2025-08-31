<script lang="ts">
	import { authenticate } from '$lib/api/authentication/authenticate';
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/stores';
	import type { AuthenticateForm } from '$lib/types';
	import Notifications from '$lib/components/Notifications.svelte';

	let form: AuthenticateForm = { email: '', password: '' };

	const handleSubmit = async () => {
		notifications.loading(true);
		try {
			const response = await authenticate(form);
			if (!response.ok) {
				const text = await response.text();
				throw new Error(text || 'Unauthorized');
			}
			goto('/dashboard');
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};
</script>

<Notifications />
<h1>Login</h1>
<form on:submit|preventDefault={handleSubmit}>
	<label for="email_authenticate">Email:</label>
	<input bind:value={form.email} id="email_authenticate" placeholder="Email" type="email" />
	<label for="password_autenthicate">Password:</label>
	<input
		bind:value={form.password}
		id="password_autenthicate"
		placeholder="Password"
		type="password"
	/>
	<button type="submit">Login</button>
</form>
