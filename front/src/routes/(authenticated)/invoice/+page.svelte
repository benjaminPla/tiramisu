<script lang="ts">
	import { env } from '$env/dynamic/public';
	import type { Buyer, Currency, Invoice, Seller, Tax } from '$lib/types';
	import { jsPDF } from 'jspdf';
	import { onMount } from 'svelte';

	const FONT_FAMILY = 'courier';
	const FONT_SIZE_HEADER = 20;
	const FONT_SIZE_NORMAL = 10;
	const FONT_SIZE_SMALL = 8;
	const JUMP_LINE = 5;
	const MARGIN_X = 20;
	const MARGIN_Y = 20;
	const PAGE_WIDTH = 210;
	const TABLE_WIDTH = PAGE_WIDTH - MARGIN_X * 2;
	let currentY = MARGIN_Y;

	let buyer: Buyer = null;
	let buyers: Buyer[] = [];
	let currencies: Currency[] = [];
	let form: Invoice = {
		buyer_id: '',
		currency: '',
		details: [{ description: '', unit_price: 0, quantity: 1, tax_id: '' }],
		issue_date: '',
		due_date: ''
	};
	let seller: Seller = null;
	let taxes: Tax[] = [];

	onMount(async () => {
		const token = localStorage.getItem('token');

		const buyersRes = await fetch(`${env.PUBLIC_API_URL}/buyers/get_all`, {
			headers: { Authorization: token }
		});
		const buyersData = await buyersRes.json();
		buyers = buyersData;

		const currenciesRes = await fetch(`${env.PUBLIC_API_URL}/others/currencies`);
		const currenciesData = await currenciesRes.json();
		currencies = currenciesData;

		const sellerRes = await fetch(`${env.PUBLIC_API_URL}/authentication/me`, {
			headers: { Authorization: token }
		});
		const sellerData = await sellerRes.json();
		seller = sellerData;

		const taxesRes = await fetch(`${env.PUBLIC_API_URL}/others/taxes`);
		const taxesData = await taxesRes.json();
		taxes = taxesData;
	});

	function addDetail() {
		form.details = [
			...form.details,
			{
				description: '',
				unit_price: 0,
				quantity: 1,
				tax_id: ''
			}
		];
	}

	function addLine(doc: jsPDF, text: string, bold = false, align: 'left' | 'right' = 'left') {
		if (bold) {
			doc.setFont(FONT_FAMILY, 'bold');
		} else {
			doc.setFont(FONT_FAMILY, 'normal');
		}
		const x = align === 'left' ? MARGIN_X : PAGE_WIDTH - MARGIN_X;
		doc.text(text, x, currentY, { align });
		currentY += JUMP_LINE;
	}

	async function downloadPdf() {
		const invoiceRes = await fetch(`${env.PUBLIC_API_URL}/invoices/create`, {
			method: 'POST',
			headers: {
				Authorization: localStorage.getItem('token'),
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(form)
		});
		if (!invoiceRes.ok) throw new Error('Create failed');
		const invoice = await invoiceRes.json();

		if (!seller || !buyer) return;

		const currencySymbol = currencies.find(
			(currency) => currency.currency === form.currency
		).symbol;

		const doc = new jsPDF();
		currentY = MARGIN_Y;

		// Title
		doc.setFontSize(FONT_SIZE_HEADER);
		addLine(doc, 'INVOICE', true);
		currentY += JUMP_LINE;

		doc.setFontSize(FONT_SIZE_NORMAL);
		addLine(doc, `Invoice No: TRM-${invoice.invoice.number}`);
		addLine(doc, `Issue Date: ${form.issue_date}`);
		addLine(doc, `Due Date: ${form.due_date}`);
		currentY += JUMP_LINE;

		// Issued To (buyer)
		addLine(doc, 'ISSUED TO:', true);
		addLine(doc, buyer.name);
		addLine(doc, buyer.address);
		addLine(doc, `${buyer.city}, ${buyer.country} (${buyer.postal_code})`);
		addLine(doc, buyer.email);
		currentY += JUMP_LINE;

		// Pay To (seller)
		addLine(doc, 'PAY TO:', true);
		addLine(doc, seller.name);
		addLine(doc, seller.address);
		addLine(doc, `${seller.city}, ${seller.country} (${seller.postal_code})`);
		addLine(doc, seller.email);
		addLine(doc, `IBAN: ${seller.bank_account}`);
		currentY += JUMP_LINE * 2;

		// Table header
		const colDescX = MARGIN_X;
		const colUnitPriceX = colDescX + 60;
		const colQtyX = colUnitPriceX + 30;
		const colTaxX = colQtyX + 20;
		const colTotalX = PAGE_WIDTH - MARGIN_X;

		doc.setFillColor(200, 200, 200);
		doc.rect(MARGIN_X, currentY - JUMP_LINE + 2, TABLE_WIDTH, 8, 'F');
		doc.setFont(FONT_FAMILY, 'bold');
		doc.text('DESCRIPTION', colDescX, currentY);
		doc.text('UNIT PRICE', colUnitPriceX, currentY);
		doc.text('QTY', colQtyX, currentY);
		doc.text('TAX', colTaxX, currentY);
		doc.text('TOTAL', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE * 2;

		// Details rows
		let subtotal = 0;
		let totalTax = 0;
		doc.setFont(FONT_FAMILY, 'normal');
		for (const item of invoice.invoice_details) {
			const total = item.unit_price * item.quantity;
			subtotal += total;

			// find the tax object
			const taxObj = taxes.find((t) => t.id === item.tax_id);
			const rate = taxObj ? taxObj.rate : 0;
			const taxLabel = taxObj ? taxObj.tax : '';
			const lineTax = (total * rate) / 100;
			totalTax += lineTax;

			doc.text(item.description, colDescX, currentY);
			doc.text(`${currencySymbol} ${item.unit_price}`, colUnitPriceX, currentY);
			doc.text(String(item.quantity), colQtyX, currentY);
			doc.text(taxLabel, colTaxX, currentY);
			doc.text(`${currencySymbol} ${total.toFixed(2)}`, colTotalX, currentY, { align: 'right' });
			currentY += JUMP_LINE * 2;
		}

		// Totals
		doc.text('SUBTOTAL', colQtyX, currentY);
		doc.text(`${currencySymbol} ${subtotal.toFixed(2)}`, colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.text('TAX', colQtyX, currentY);
		doc.text(`${currencySymbol} ${totalTax.toFixed(2)}`, colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.setFont(FONT_FAMILY, 'bold');
		doc.text('TOTAL', colQtyX, currentY);
		doc.text(`${currencySymbol} ${(subtotal + totalTax).toFixed(2)}`, colTotalX, currentY, {
			align: 'right'
		});
		currentY += JUMP_LINE * 2;

		// Footer
		doc.setFontSize(FONT_SIZE_SMALL);
		doc.setFont(FONT_FAMILY, 'normal');
		doc.text(
			'Invoice exempt from VAT under Directive 2006/112/EC and Article 25 of Law 37/1992',
			MARGIN_X,
			currentY
		);

		doc.save('invoice.pdf');
	}
</script>

<h1>Create Invoice</h1>

<form on:submit|preventDefault={downloadPdf}>
	<label>Buyer</label>
	<select
		bind:value={form.buyer_id}
		on:change={(e) => {
			const id = e.target.value;
			buyer = buyers.find((b) => b.id === id) || null;
		}}
	>
		<option value="" disabled>Select buyer</option>
		{#each buyers as buyer}
			<option value={buyer.id}>{buyer.name} ({buyer.email})</option>
		{/each}
	</select>

	<label>Issue Date</label>
	<input type="date" bind:value={form.issue_date} />

	<label>Due Date</label>
	<input type="date" bind:value={form.due_date} />

	<label>Currency</label>
	<select bind:value={form.currency}>
		<option value="" disabled>Select tax</option>
		{#each currencies as currency}
			<option value={currency.currency}>{currency.symbol} {currency.currency}</option>
		{/each}
	</select>

	<h2>Invoice Details</h2>
	{#each form.details as item, i}
		<div class="detail-row">
			<input placeholder="Description" bind:value={item.description} />
			<input
				type="number"
				placeholder="Unit Price"
				bind:value={item.unit_price}
				min="0.01"
				step="0.01"
			/>
			<select bind:value={item.tax_id}>
				<option value="" disabled>Select tax</option>
				{#each taxes as tax}
					<option value={tax.id}>{tax.tax}</option>
				{/each}
			</select>
			<input type="number" placeholder="Quantity" bind:value={item.quantity} min="1" step="1" />
		</div>
	{/each}
	<button type="button" on:click={addDetail}>+ Add Item</button>

	<button type="submit">Generate Invoice PDF</button>
</form>
