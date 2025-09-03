<script lang="ts">
	import { buyersGetAll } from '$lib/api/buyers/getAll';
	import { handleResponse } from '$lib/api/handleResponse';
	import { invoicesCreate } from '$lib/api/invoices/create';
	import { jsPDF } from 'jspdf';
	import { me } from '$lib/api/authentication/me';
	import { notifications } from '$lib/stores';
	import { onMount } from 'svelte';
	import { sellerInvoiceNotesGetAll } from '$lib/api/sellers/sellerInvoiceNotesGetAll';
	import type {
		Buyer,
		Currency,
		InvoiceCreateResponse,
		InvoiceDetail,
		InvoiceForm,
		Seller,
		SellerInvoiceNote,
		Tax
	} from '$lib/types';

	export let data: {
		currencies: Currency[];
		taxes: Tax[];
	};

	const { currencies, taxes } = data;

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

	const EMPTY_INVOICE_DETAIL: InvoiceDetail = {
		description: '',
		unit_price: 0,
		quantity: 0,
		tax_id: 0
	};
	const EMPTY_FORM: InvoiceForm = {
		buyer_id: '',
		currency: '',
		details: [{ ...EMPTY_INVOICE_DETAIL }],
		due_date: '',
		issue_date: '',
		notes: []
	};

	let buyers: Buyer[] = [];
	let form = EMPTY_FORM;
	let selectedSellerInvoiceNotes: string[] = [];
	let sellerInvoiceNotes: SellerInvoiceNote[] = [];

	$: form.notes = selectedSellerInvoiceNotes;

	const addInvoiceDetail = () => {
		form.details = [...form.details, { ...EMPTY_INVOICE_DETAIL }];
	};

	const removeInvoiceDetails = (i: number) => {
		form.details = form.details.filter((_, index) => index !== i);
	};

	const addLine = (doc: jsPDF, text: string, bold = false, align: 'left' | 'right' = 'left') => {
		if (bold) {
			doc.setFont(FONT_FAMILY, 'bold');
		} else {
			doc.setFont(FONT_FAMILY, 'normal');
		}
		const x = align === 'left' ? MARGIN_X : PAGE_WIDTH - MARGIN_X;
		doc.text(text, x, currentY, { align });
		currentY += JUMP_LINE;
	};

	const handleSubmit = async () => {
		notifications.loading(true);
		try {
			const response = await invoicesCreate(form);
			const invoiceData = await handleResponse<InvoiceCreateResponse>(response);

			const sellerResponse = await me();
			const seller = await handleResponse<Seller>(sellerResponse);

			if (!seller || !invoiceData || !currencies.length || !taxes.length) {
				notifications.error('Error creating invoice');
				return;
			}

			const buyer = buyers.find((buyer) => buyer.id === form.buyer_id);
			const currency = currencies.find((currency: Currency) => currency.currency === form.currency);
			if (!buyer || !currency) return;
			const currencySymbol = currency.symbol;

			const doc = new jsPDF();
			currentY = MARGIN_Y;
			doc.setFontSize(FONT_SIZE_HEADER);
			addLine(doc, 'INVOICE', true);
			currentY += JUMP_LINE;
			doc.setFontSize(FONT_SIZE_NORMAL);
			addLine(doc, `Invoice No: TRM-${invoiceData.invoice.number}`);
			addLine(doc, `Issue Date: ${form.issue_date}`);
			addLine(doc, `Due Date: ${form.due_date}`);
			currentY += JUMP_LINE;
			addLine(doc, 'ISSUED TO:', true);
			addLine(doc, buyer.name);
			addLine(doc, buyer.address);
			addLine(doc, `${buyer.city}, ${buyer.country} (${buyer.postal_code})`);
			addLine(doc, buyer.email);
			currentY += JUMP_LINE;
			addLine(doc, 'PAY TO:', true);
			addLine(doc, seller.name);
			addLine(doc, seller.address);
			addLine(doc, `${seller.city}, ${seller.country} (${seller.postal_code})`);
			addLine(doc, seller.email);
			addLine(doc, `IBAN: ${seller.bank_account}`);
			currentY += JUMP_LINE * 2;
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
			let subtotal = 0;
			let totalTax = 0;
			doc.setFont(FONT_FAMILY, 'normal');
			for (const invoiceDetail of invoiceData.invoice_details) {
				const total = invoiceDetail.unit_price * invoiceDetail.quantity;
				subtotal += total;
				const tax = taxes.find((tax: Tax) => tax.id === invoiceDetail.tax_id);
				if (!tax) {
					notifications.error('Error selecting taxes');
					return;
				}
				const rate = tax.rate;
				const taxLabel = tax.tax;
				const lineTax = (total * rate) / 100;
				totalTax += lineTax;
				doc.text(invoiceDetail.description, colDescX, currentY);
				doc.text(`${currencySymbol} ${invoiceDetail.unit_price}`, colUnitPriceX, currentY);
				doc.text(String(invoiceDetail.quantity), colQtyX, currentY);
				doc.text(taxLabel, colTaxX, currentY);
				doc.text(`${currencySymbol} ${total.toFixed(2)}`, colTotalX, currentY, { align: 'right' });
				currentY += JUMP_LINE * 2;
			}
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
			doc.setFontSize(FONT_SIZE_SMALL);
			doc.setFont(FONT_FAMILY, 'normal');
			for (const invoiceNote of invoiceData.invoice_notes) {
				addLine(doc, invoiceNote.note);
			}
			doc.save('invoice.pdf');
			notifications.add('Invoice created', 'success');
			selectedSellerInvoiceNotes = [];
			form = { ...EMPTY_FORM, details: [{ ...EMPTY_INVOICE_DETAIL }] };
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	};

	onMount(async () => {
		notifications.loading(true);
		try {
			const buyersResponse = await buyersGetAll();
			buyers = await handleResponse<Buyer[]>(buyersResponse);
			const sellerInvoiceNotesResponse = await sellerInvoiceNotesGetAll();
			sellerInvoiceNotes = await handleResponse<SellerInvoiceNote[]>(sellerInvoiceNotesResponse);
		} catch (error: unknown) {
			notifications.error(error);
		} finally {
			notifications.loading(false);
		}
	});
</script>

<svelte:head>
	<link rel="stylesheet" href="/styles/invoice.css" />
</svelte:head>
<h1>Create Invoice</h1>
<form on:submit|preventDefault={handleSubmit}>
	<label for="buyer">Buyer:</label>
	<select bind:value={form.buyer_id} id="buyer">
		<option value="" disabled>Select buyer</option>
		{#each buyers as buyer}
			<option value={buyer.id}>{buyer.name} ({buyer.email})</option>
		{/each}
	</select>
	<label for="issue_date">Issue date:</label>
	<input bind:value={form.issue_date} id="issue_date" type="date" />
	<label for="due_date">Due date:</label>
	<input bind:value={form.due_date} id="due_date" type="date" />
	<label for="currency">Currency:</label>
	<select bind:value={form.currency} id="currency">
		<option value="" disabled>Select tax</option>
		{#each currencies.sort( (a: Currency, b: Currency) => a.currency.localeCompare(b.currency) ) as currency}
			<option value={currency.currency}>{currency.currency} {currency.symbol}</option>
		{/each}
	</select>
	<div class="invoice-details">
		<p>Description:</p>
		<p>Unit price:</p>
		<p>Tax:</p>
		<p>Quantity:</p>
		<p>Action:</p>
		{#each form.details as item, i}
			<input bind:value={item.description} id="description" placeholder="Description" />
			<input
				bind:value={item.unit_price}
				id="unit_price"
				min="0.01"
				placeholder="Unit Price"
				step="0.01"
				type="number"
			/>
			<select bind:value={item.tax_id} id="tax_id">
				<option value="" disabled>Select tax</option>
				{#each taxes as tax}
					<option value={tax.id}>{tax.tax}</option>
				{/each}
			</select>
			<input
				bind:value={item.quantity}
				id="quantity"
				min="0.01"
				placeholder="Quantity"
				step="0.01"
				type="number"
			/>
			<button class="button-danger" type="button" on:click={() => removeInvoiceDetails(i)}>-</button
			>
		{/each}
	</div>
	<button type="button" on:click={addInvoiceDetail}>+ Add Detail</button>
	<div class="invoice-notes">
		{#each sellerInvoiceNotes as sellerInvoiceNote}
			<label>
				<input
					type="checkbox"
					bind:group={selectedSellerInvoiceNotes}
					value={sellerInvoiceNote.id}
				/>
				{sellerInvoiceNote.note}
			</label>
		{/each}
	</div>
	<button type="submit">Generate Invoice PDF</button>
</form>
