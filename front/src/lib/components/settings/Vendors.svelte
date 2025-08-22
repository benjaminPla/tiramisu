<script lang="ts">
	import { createVendor } from '$lib/api/vendors/create';
	import { onMount } from 'svelte';
	import type { Vendor } from '$lib/types';

	const emptyForm = { address: '', name: '', postal_code: '', vat_number: '' };
	let form: Vendor = emptyForm;
	let loading = false;

	const handleSubmit = async () => {
		loading = true;
		await createVendor(form).finally((loading = false));
		form = emptyForm;
	};
</script>

<details>
	<summary>Create vendors</summary>
	{#if loading}
		<p>Loading...</p>
	{:else}
		<form on:submit|preventDefault={handleSubmit}>
			<label for="address-vendor">Address:</label>
			<input id="address-vendor" bind:value={form.address} placeholder="Address" />
			<label for="name-vendor">Name:</label>
			<input id="name-vendor" bind:value={form.name} placeholder="Name" />
			<label for="postal_code-vendor">Postal Code:</label>
			<input id="postal_code-vendor" bind:value={form.postal_code} placeholder="Postal Code" />
			<label for="vat_number-vendor">VAT Number:</label>
			<input id="vat_number-vendor" bind:value={form.vat_number} placeholder="VAT Number" />
			<button type="submit">Create</button>
		</form>
	{/if}
</details>
