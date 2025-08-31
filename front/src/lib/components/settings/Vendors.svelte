<script lang="ts">
	import { createVendor } from '$lib/api/vendors/create';
	import type { VendorForm } from '$lib/types';

	const EMPTY_FORM: VendorForm = { address: '', name: '', seller_id: '', vat_number: '' };
	let form: VendorForm = EMPTY_FORM;

	const handleSubmit = async () => {
		await createVendor(form);
		form = EMPTY_FORM;
	};
</script>

<details>
	<summary>Create vendors</summary>
	<form on:submit|preventDefault={handleSubmit}>
		<label for="address_vendor">Address:</label>
		<input id="address_vendor" bind:value={form.address} placeholder="Address" />
		<label for="name_vendor">Name:</label>
		<input id="name_vendor" bind:value={form.name} placeholder="Name" />
		<label for="seller_id__vendor">Seller:</label>
		<input id="seller_id_vendor" bind:value={form.seller_id} placeholder="Seller" />
		<label for="vat_number_vendor">VAT Number:</label>
		<input id="vat_number_vendor" bind:value={form.vat_number} placeholder="VAT Number" />
		<button type="submit">Create</button>
	</form>
</details>
