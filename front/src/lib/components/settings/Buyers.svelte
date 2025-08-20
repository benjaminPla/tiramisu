<script lang="ts">
	import type { Buyer } from '$lib/types';
	import { env } from '$env/dynamic/public';
	import { onMount } from 'svelte';

	let countries: string[] = [];
	let form: Buyer = {
		address: '',
		city: '',
		country: '',
		email: '',
		name: '',
		postal_code: '',
		vat_number: ''
	};
	let loading = true;

	onMount(async () => {
		try {
			const countriesRes = await fetch(`${env.PUBLIC_API_URL}/others/countries`);
			if (!countriesRes.ok) throw new Error('Failed to load countries');
			const countriesData = await countriesRes.json();
			countries = countriesData;
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	});

	async function createBuyer() {
		try {
			const res = await fetch(`${env.PUBLIC_API_URL}/buyers/create`, {
				body: JSON.stringify(form),
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				method: 'POST'
			});
			if (!res.ok) throw new Error('Create failed');
			alert('Buyer created successfully!');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<details>
	<summary class="cursor-pointer font-semibold">Create buyer</summary>

	{#if loading}
		<p>Loading...</p>
	{:else}
		<form on:submit|preventDefault={createBuyer}>
			<input bind:value={form.address} placeholder="Address" />
			<input bind:value={form.city} placeholder="City" />
			<select bind:value={form.country}>
				<option value="" disabled>Select country</option>
				{#each countries as country}
					<option value={country}>{country}</option>
				{/each}
			</select>
			<input bind:value={form.email} type="email" placeholder="Email" />
			<input bind:value={form.name} placeholder="Name" />
			<input bind:value={form.postal_code} placeholder="Postal Code" />
			<input bind:value={form.vat_number} placeholder="VAT Number" />
			<button type="submit">Create</button>
		</form>
	{/if}
</details>
