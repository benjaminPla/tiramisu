<script lang="ts">
	import { countries } from '$lib/stores';
	import { loadSeller, seller, updateSeller } from '$lib/stores';
	import { onMount } from 'svelte';
	import type { Seller } from '$lib/types';

	let form: Seller = null;
	let loading = true;

	onMount(async () => {
		await loadSeller().finally((loading = false));
		$seller && (form = { ...$seller });
	});
</script>

<details>
	<summary>Your Account</summary>
	{#if loading}
		<p>Loading...</p>
	{:else if form}
		<form on:submit|preventDefault={() => updateSeller(form)}>
			<label for="address-seller">Address:</label>
			<input id="address-seller" bind:value={form.address} placeholder="Address" />
			<label for="bank_account-seller">Bank Account:</label>
			<input id="bank_account-seller" bind:value={form.bank_account} placeholder="Bank Account" />
			<label for="city-seller">City:</label>
			<input id="city-seller" bind:value={form.city} placeholder="City" />
			<label for="country-seller">Country:</label>
			<select id="country-seller" bind:value={form.country}>
				<option value="" disabled>Select country</option>
				{#each $countries as country}
					<option value={country}>{country}</option>
				{/each}
			</select>
			<label for="email-seller">Email:</label>
			<input id="email-seller" bind:value={form.email} type="email" placeholder="Email" />
			<label for="name-seller">Name:</label>
			<input id="name-seller" bind:value={form.name} placeholder="Name" />
			<label for="postal_code-seller">Postal Code:</label>
			<input id="postal_code-seller" bind:value={form.postal_code} placeholder="Postal Code" />
			<label for="vat_number-seller">VAT Number:</label>
			<input id="vat_number-seller" bind:value={form.vat_number} placeholder="VAT Number" />
			<button type="submit">Update</button>
		</form>
	{/if}
</details>
