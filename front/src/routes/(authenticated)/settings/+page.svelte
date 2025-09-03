<script lang="ts">
	import BuyersCreate from '$lib/components/settings/BuyersCreate.svelte';
	import InvoiceNotes from '$lib/components/settings/InvoiceNotes.svelte';
	import type { Country } from '$lib/types';
	import VendorsCreate from '$lib/components/settings/VendorsCreate.svelte';
	import YourAccount from '$lib/components/settings/YourAccount.svelte';

	export let data: { countries: Country[] };

	const { countries } = data;
</script>

<h1>Settings</h1>
<BuyersCreate {countries} />
<InvoiceNotes />
<VendorsCreate />
<YourAccount {countries} />
