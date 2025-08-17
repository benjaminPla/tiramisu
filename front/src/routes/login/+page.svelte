<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { goto } from '$app/navigation';

	let form = {
		email: '',
		password: ''
	};
	let loading = false;

	async function login() {
		loading = true;
		try {
			const res = await fetch(`${env.PUBLIC_API_URL}/authentication/authenticate`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(form)
			});
			if (!res.ok) throw new Error('Update failed');
			const data = await res.json();
			localStorage.setItem('token', data.token);
			goto('/dashboard');
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	}
</script>

<h1>Login</h1>
<form on:submit|preventDefault={login}>
	<input bind:value={form.email} type="email" placeholder="Email" />
	<input bind:value={form.password} type="password" placeholder="Password" />
	<button type="submit">Login</button>
</form>
{#if loading}
	<p>Loading...</p>
{/if}
