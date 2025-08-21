<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { goto } from '$app/navigation';
	import Notification from '$lib/components/Notification.svelte';
	import { showNotification } from '$lib/stores';

	const apiUrl = env.PUBLIC_API_URL;
	let form = { email: '', password: '' };
	let loading = false;

	async function handleSubmit() {
		loading = true;
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
				throw new Error(text);
			}
			goto('/dashboard');
		} catch (error) {
			showNotification(error.message, 'error');
		} finally {
			loading = false;
		}
	}
</script>

<Notification />
<h1>Login</h1>
<form on:submit|preventDefault={handleSubmit}>
	<label for="email">Email:</label>
	<input bind:value={form.email} id="email" type="email" placeholder="Email" />
	<label for="password">Password:</label>
	<input bind:value={form.password} id="password" type="password" placeholder="Password" />
	<button type="submit">Login</button>
</form>
{#if loading}
	<p>Loading...</p>
{/if}
