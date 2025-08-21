<script lang="ts">
	import { countries } from '$lib/stores';
	import { loadSeller, seller, updateSeller } from '$lib/stores';
	import { onMount } from 'svelte';
	import type { Seller } from '$lib/types';

	let form: Seller = null;
	let loading = true;

	onMount(async () => {
		await loadSeller();
		$seller && (form = { ...$seller });
		loading = false;
	});
</script>

<details>
	<summary>Your Account</summary>
	{#if loading}
		<p>Loading...</p>
	{:else if form}
		<form on:submit|preventDefault={() => updateSeller(form)}>
			<input bind:value={form.address} placeholder="Address" />
			<input bind:value={form.bank_account} placeholder="Bank Account" />
			<input bind:value={form.city} placeholder="City" />
			<select bind:value={form.country}>
				<option value="" disabled>Select country</option>
				{#each $countries as country}
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
