<script lang="ts">
	import { buyersCreate } from '$lib/api/buyers/create';
	import { handleResponse } from '$lib/api/handleResponse';
	import { notifications } from '$lib/stores';
	import type { BuyerForm } from '$lib/types';

	export let countries;

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
			form = EMPTY_FORM;
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};
</script>

<details>
	<summary>Create buyer</summary>
	<form on:submit|preventDefault={handleSubmit}>
		<label for="address_buyer">Address:</label>
		<input id="address_buyer" bind:value={form.address} placeholder="Address" required />
		<label for="city_buyer">City:</label>
		<input id="city_buyer" bind:value={form.city} placeholder="City" required />
		<label for="country_buyer">Country:</label>
		<select id="country_buyer" bind:value={form.country} required>
			<option value="" disabled>Select country</option>
			{#each countries ?? [] as country}
				<option value={country}>{country}</option>
			{/each}
		</select>
		<label for="email_buyer">Email:</label>
		<input id="email_buyer" bind:value={form.email} type="email" placeholder="Email" required />
		<label for="name_buyer">Name:</label>
		<input id="name_buyer" bind:value={form.name} placeholder="Name" required />
		<label for="postal_code_buyer">Postal Code:</label>
		<input
			id="postal_code_buyer"
			bind:value={form.postal_code}
			placeholder="Postal Code"
			required
		/>
		<label for="vat_number_buyer">VAT Number:</label>
		<input id="vat_number_buyer" bind:value={form.vat_number} placeholder="VAT Number" required />
		<button type="submit">Create</button>
	</form>
</details>
