<script lang="ts">
	import { createBuyer } from '$lib/api/buyers/create';
	import { env } from '$env/dynamic/public';
	import { onMount } from 'svelte';
	import type { Buyer } from '$lib/types';

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

	const handleSubmit = async () => {
		await createBuyer(form);
		form = {
			address: '',
			city: '',
			country: '',
			email: '',
			name: '',
			postal_code: '',
			vat_number: ''
		};
	};

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
</script>

<details>
	<summary>Create buyer</summary>
	{#if loading}
		<p>Loading...</p>
	{:else}
		<form on:submit|preventDefault={handleSubmit}>
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
