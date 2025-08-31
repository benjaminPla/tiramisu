<script lang="ts">
	import { buyersCreate } from '$lib/api/buyers/create';
	import { countries } from '$lib/stores';
	import { handleResponse } from '$lib/api/handleResponse';
	import { notifications } from '$lib/stores';
	import type { BuyerForm } from '$lib/types';

	const EMPTY_FORM: BuyerForm = {
		address: '',
		city: '',
		country: '',
		email: '',
		name: '',
		postal_code: '',
		vat_number: ''
	};
	let form = EMPTY_FORM;

	const handleSubmit = async () => {
		notifications.loading(true);
		try {
			const response = await buyersCreate(form);
			await handleResponse(response);
		} catch (error: unknown) {
			form = EMPTY_FORM;
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};
</script>

<details>
	<summary>Create buyer</summary>
	<form on:submit|preventDefault={handleSubmit}>
		<label for="address-buyer">Address:</label>
		<input id="address-buyer" bind:value={form.address} placeholder="Address" required />
		<label for="city-buyer">City:</label>
		<input id="city-buyer" bind:value={form.city} placeholder="City" required />
		<label for="country-buyer">Country:</label>
		<select id="country-buyer" bind:value={form.country} required>
			<option value="" disabled>Select country</option>
			{#each $countries ?? [] as country}
				<option value={country}>{country}</option>
			{/each}
		</select>
		<label for="email-buyer">Email:</label>
		<input id="email-buyer" bind:value={form.email} type="email" placeholder="Email" required />
		<label for="name-buyer">Name:</label>
		<input id="name-buyer" bind:value={form.name} placeholder="Name" required />
		<label for="postal_code-buyer">Postal Code:</label>
		<input
			id="postal_code-buyer"
			bind:value={form.postal_code}
			placeholder="Postal Code"
			required
		/>
		<label for="vat_number-buyer">VAT Number:</label>
		<input id="vat_number-buyer" bind:value={form.vat_number} placeholder="VAT Number" required />
		<button type="submit">Create</button>
	</form>
</details>
