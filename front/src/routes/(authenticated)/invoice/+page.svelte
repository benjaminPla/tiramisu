<script lang="ts">
	import { env } from '$env/dynamic/public';
	import { Buyer, Invoice, Seller } from '$lib/types';
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
	let invoice: Invoice = {
		buyer_id: null,
		currency: null,
		details: [],
		issue_date: null,
		due_date: null
	};
	let seller: Seller = null;

	onMount(async () => {
		const token = localStorage.getItem('token');

		const sellerRes = await fetch(`${env.PUBLIC_API_URL}/sellers/me`, {
			headers: { Authorization: token }
		});
		const sellerData = await sellerRes.json();
		seller = sellerData;

		const buyersRes = await fetch(`${env.PUBLIC_API_URL}/buyers/get`, {
			headers: { Authorization: token }
		});
		const buyersData = await buyersRes.json();
		buyers = buyersData;
	});

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

	function downloadPdf() {
		if (!seller || !buyer) return; // show message

		const doc = new jsPDF();
		currentY = MARGIN_Y;

		// Title
		doc.setFontSize(FONT_SIZE_HEADER);
		addLine(doc, 'INVOICE', true);
		currentY += JUMP_LINE;

		doc.setFontSize(FONT_SIZE_NORMAL);
		addLine(doc, `Invoice No: TEMP-001`);
		addLine(doc, `Issue Date: ${invoice.issue_date}`);
		addLine(doc, `Due Date: ${invoice.due_date}`);
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
		if (seller.bank_account) addLine(doc, `IBAN: ${seller.bank_account}`);
		currentY += JUMP_LINE * 2;

		// Table header
		const colDescX = MARGIN_X;
		const colUnitPriceX = colDescX + 90;
		const colQtyX = colUnitPriceX + 40;
		const colTotalX = PAGE_WIDTH - MARGIN_X;

		doc.setFillColor(200, 200, 200);
		doc.rect(MARGIN_X, currentY - JUMP_LINE + 2, TABLE_WIDTH, 8, 'F');
		doc.setFont(FONT_FAMILY, 'bold');
		doc.text('DESCRIPTION', colDescX, currentY);
		doc.text('UNIT PRICE', colUnitPriceX, currentY);
		doc.text('QTY', colQtyX, currentY);
		doc.text('TOTAL', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE * 2;

		// Details rows
		let subtotal = 0;
		doc.setFont(FONT_FAMILY, 'normal');
		for (const item of invoice.details) {
			const total = item.unit_price * item.quantity;
			subtotal += total;
			doc.text(item.description, colDescX, currentY);
			doc.text(`${item.unit_price} ${invoice.currency}`, colUnitPriceX, currentY);
			doc.text(String(item.quantity), colQtyX, currentY);
			doc.text(`${total} ${invoice.currency}`, colTotalX, currentY, { align: 'right' });
			currentY += JUMP_LINE * 2;
		}

		// Totals
		doc.text('SUBTOTAL', colQtyX, currentY);
		doc.text(`${subtotal} ${invoice.currency}`, colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.text('Tax', colQtyX, currentY);
		doc.text('0', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.setFont(FONT_FAMILY, 'bold');
		doc.text('TOTAL', colQtyX, currentY);
		doc.text(`${subtotal} ${invoice.currency}`, colTotalX, currentY, { align: 'right' });
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
		bind:value={invoice.buyer_id}
		on:change={() => (buyer = buyers.find((b) => b.id === invoice.buyer_id) ?? null)}
	>
		<option value="" disabled selected>Select buyer</option>
		{#each buyers as buyer}
			<option value={buyer.id}>{buyer.name} ({buyer.email})</option>
		{/each}
	</select>

	<label>Issue Date</label>
	<input type="date" bind:value={invoice.issue_date} />

	<label>Due Date</label>
	<input type="date" bind:value={invoice.due_date} />

	<label>Currency</label>
	<input type="text" bind:value={invoice.currency} placeholder="e.g. EUR" />

	<h2>Invoice Details</h2>
	{#each invoice.details as item, i}
		<div class="detail-row">
			<input placeholder="Description" bind:value={item.description} />
			<input type="number" placeholder="Unit Price" bind:value={item.unit_price} step="0.01" />
			<input type="number" placeholder="Quantity" bind:value={item.quantity} />
		</div>
	{/each}
	<button type="button" on:click={addDetail}>+ Add Item</button>

	<button type="submit">Generate Invoice PDF</button>
</form>
