<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { onMount } from 'svelte';
	import type { Seller } from '$lib/types';

	let countries: string[] = [];
	let form: Seller = {
		address: '',
		bank_account: '',
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
			const sellerRes = await fetch(`${env.PUBLIC_API_URL}/authentication/me`, {
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' }
			});
			if (!sellerRes.ok) throw new Error('Failed to load seller info');
			const sellerData = await sellerRes.json();
			form = sellerData;

			const countriesRes = await fetch(`${env.PUBLIC_API_URL}/others/countries`, {
				headers: { 'Content-Type': 'application/json' }
			});
			if (!countriesRes.ok) throw new Error('Failed to load countries');
			const countriesData = await countriesRes.json();
			countries = countriesData;
		} catch (e) {
			console.error(e);
		} finally {
			loading = false;
		}
	});

	async function updateSeller() {
		try {
			const res = await fetch(`${env.PUBLIC_API_URL}/sellers/update`, {
				body: JSON.stringify(form),
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				method: 'PUT'
			});
			if (!res.ok) throw new Error('Update failed');
			alert('Seller updated successfully!');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<details>
	<summary class="cursor-pointer font-semibold">Your Account</summary>

	{#if loading}
		<p>Loading...</p>
	{:else}
		<form on:submit|preventDefault={updateSeller}>
			<input bind:value={form.address} placeholder="Address" />
			<input bind:value={form.bank_account} placeholder="Bank Account" />
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
			<button type="submit">Update</button>
		</form>
	{/if}
</details>
