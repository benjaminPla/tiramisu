<script lang="ts">
	import { handleResponse } from '$lib/api/handleResponse';
	import { notifications } from '$lib/stores';
	import { sellerInvoiceNotesCreate } from '$lib/api/sellers/sellerInvoiceNotesCreate';

	const EMPTY_FORM = { note: '' };
	let form = EMPTY_FORM;

	const handleSubmit = async () => {
		notifications.loading(true);
		try {
			const response = await sellerInvoiceNotesCreate(form);
			await handleResponse(response);
			form = EMPTY_FORM;
			notifications.add('Invoice note created', 'success');
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};
</script>

<details>
	<summary>Create invoice note</summary>
	<form on:submit|preventDefault={handleSubmit}>
		<label for="note_note">Note:</label>
		<input id="note_note" bind:value={form.note} placeholder="Note" required />
		<button type="submit">Create</button>
	</form>
</details>
