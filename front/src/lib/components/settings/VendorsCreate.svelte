<script lang="ts">
	import { handleResponse } from '$lib/api/handleResponse';
	import { notifications } from '$lib/stores';
	import type { VendorForm } from '$lib/types';
	import { vendorsCreate } from '$lib/api/vendors/create';

	const EMPTY_FORM: VendorForm = { address: '', name: '', vat_number: '' };
	let form: VendorForm = EMPTY_FORM;

	const handleSubmit = async () => {
		notifications.loading(true);
		try {
			const response = await vendorsCreate(form);
			await handleResponse(response, false);
			form = EMPTY_FORM;
			notifications.add('Vendor created', 'success');
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};
</script>

<details>
	<summary>Create vendors</summary>
	<form on:submit|preventDefault={handleSubmit}>
		<label for="address_vendor">Address:</label>
		<input id="address_vendor" bind:value={form.address} placeholder="Address" required />
		<label for="name_vendor">Name:</label>
		<input id="name_vendor" bind:value={form.name} placeholder="Name" required />
		<label for="vat_number_vendor">VAT Number:</label>
		<input id="vat_number_vendor" bind:value={form.vat_number} placeholder="VAT Number" required />
		<button type="submit">Create</button>
	</form>
</details>
