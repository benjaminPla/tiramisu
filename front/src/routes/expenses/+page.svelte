<script lang="ts">
	const options = [
		{ id: 1, label: 'Childcare' },
		{ id: 2, label: 'Electricity' },
		{ id: 3 label: 'English Classes' },
		{ id: 4, label: 'Internet' },
		{ id: 5, label: 'Mobile Phone' },
		{ id: 6, label: 'Private Health Insurance' },
		{ id: 7, label: 'Tech Purchases' }
	];

	// Array of forms: each form has id and selected expense key
	let forms: { id: string; expenseKey: string }[] = [];

	// Track the current select value
	let currentSelection = '';

	function addForm() {
		if (currentSelection) {
			forms = [...forms, { id: crypto.randomUUID(), expenseKey: currentSelection }];
			currentSelection = '';
		}
	}

	function removeForm(id: string) {
		forms = forms.filter((f) => f.id !== id);
	}
</script>

<label for="expenseType">Select expense to upload:</label>
<select id="expenseType" bind:value={currentSelection}>
	<option value="" disabled selected>Select expense</option>
	{#each options as option}
		<option value={option.id}>{option.label}</option>
	{/each}
</select>

<button on:click={addForm} disabled={!currentSelection}>Add</button>

{#each forms as form (form.id)}
	<div style="border:1px solid #ccc; padding: 10px; margin: 10px 0;">
		<p><strong>{options.find((o) => o.id === form.expenseKey)?.label}</strong></p>
		<input type="file" required />
		<input type="date" required />
		<input type="date" required />
		<button on:click={() => removeForm(form.id)}>Delete</button>
	</div>
{/each}
