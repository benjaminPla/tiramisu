<script lang="ts">
	let expense = '';
	let message = '';

	const API_URL = import.meta.env.VITE_API_URL;

	async function submitCustomExpense(e: Event) {
		e.preventDefault();
		if (!customExpense.trim()) {
			message = 'Please enter an expense option.';
			return;
		}

		try {
			const res = await fetch(`${API_URL}/settings/expenses/post`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ expense: expense.trim() })
			});

			if (!res.ok) {
				message = 'Failed to add expense option.';
				return;
			}

			message = `Expense option "${expense.trim()}" added successfully!`;
			customExpense = '';
		} catch (err) {
			message = 'Error: ' + err;
		}
	}
</script>

<div>
	<p class="tooltip">You can customize your expenses by adding new expense options below.</p>
	<form on:submit={submitCustomExpense}>
		<label for="expense">Add an expense option:</label><br />
		<input id="expense" type="text" placeholder="E.g.: Childcare" bind:value={expense} required />
		<button type="submit">Submit</button>
	</form>

	{#if message}
		<p>{message}</p>
	{/if}
</div>
