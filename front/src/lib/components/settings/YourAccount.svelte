<script lang="ts">
	import { countries } from '$lib/stores';
	import { handleResponse } from '$lib/api/handleResponse';
	import { me } from '$lib/api/authentication/me';
	import { notifications } from '$lib/stores';
	import { onMount } from 'svelte';
	import { sellerUpdate } from '$lib/api/sellers/update';
	import type { SellerForm, Seller } from '$lib/types';

	const EMPTY_FORM: SellerForm = {
		address: '',
		bank_account: '',
		city: '',
		country: '',
		email: '',
		name: '',
		postal_code: '',
		vat_number: ''
	};
	let form = EMPTY_FORM;

	const handleSubmit = async (form: SellerForm) => {
		notifications.loading(true);
		try {
			const response = await sellerUpdate(form);
			await handleResponse(response);
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};

	onMount(async () => {
		notifications.loading(true);
		try {
			const response = await me();
			form = await handleResponse<Seller>(response);
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	});
</script>

<details>
	<summary>Your Account</summary>
	<form on:submit|preventDefault={() => handleSubmit(form)}>
		<label for="address-seller">Address:</label>
		<input id="address-seller" bind:value={form.address} placeholder="Address" required />
		<label for="bank_account-seller">Bank Account:</label>
		<input
			id="bank_account-seller"
			bind:value={form.bank_account}
			placeholder="Bank Account"
			required
		/>
		<label for="city-seller">City:</label>
		<input id="city-seller" bind:value={form.city} placeholder="City" required />
		<label for="country-seller">Country:</label>
		<select id="country-seller" bind:value={form.country} required>
			<option value="" disabled>Select country</option>
			{#each $countries as country}
				<option value={country}>{country}</option>
			{/each}
		</select>
		<label for="email-seller">Email:</label>
		<input id="email-seller" bind:value={form.email} type="email" placeholder="Email" required />
		<label for="name-seller">Name:</label>
		<input id="name-seller" bind:value={form.name} placeholder="Name" required />
		<label for="postal_code-seller">Postal Code:</label>
		<input
			id="postal_code-seller"
			bind:value={form.postal_code}
			placeholder="Postal Code"
			required
		/>
		<label for="vat_number-seller">VAT Number:</label>
		<input id="vat_number-seller" bind:value={form.vat_number} placeholder="VAT Number" required />
		<button type="submit">Update</button>
	</form>
</details>
