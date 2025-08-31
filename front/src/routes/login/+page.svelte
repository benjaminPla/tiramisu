<script lang="ts">
	import { authenticate } from '$lib/api/authentication/authenticate';
	import Notification from '$lib/components/Notification.svelte';
	import type { AuthenticateForm } from '$lib/types';

	let form: AuthenticateForm = { email: '', password: '' };

	const handleSubmit = async () => {
		await authenticate(form);
	};
</script>

<Notification />
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
