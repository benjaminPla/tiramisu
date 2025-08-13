<script lang="ts">
	import { jsPDF } from 'jspdf';

	const JUMP_LINE = 5;
	const FONT_FAMILY = 'courier';
	const FONT_SIZE_HEADER = 20;
	const FONT_SIZE_NORMAL = 10;
	const FONT_SIZE_SMALL = 8;
	const MARGIN_X = 20;
	const MARGIN_Y = 20;
	const PAGE_WIDTH = 210; // A4 in mm
	const TABLE_WIDTH = PAGE_WIDTH - MARGIN_X * 2;

	let currentY = MARGIN_Y;

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
		const doc = new jsPDF();

		// Title
		doc.setFontSize(FONT_SIZE_HEADER);
		addLine(doc, 'INVOICE', true);
		currentY += JUMP_LINE;

		doc.setFontSize(FONT_SIZE_NORMAL);
		addLine(doc, `Invoice No: 07/25`);
		addLine(doc, `Date: 01.07.2025`);
		addLine(doc, `Due Date: 31.08.2025`);
		addLine(doc, `Service Period: 01.07.2025 - 31.07.2025`);
		currentY += JUMP_LINE; // space

		// Issued To
		addLine(doc, 'ISSUED TO:', true);
		addLine(doc, 'Point65 Sweden AB');
		addLine(doc, 'Shelundsvägen 26');
		addLine(doc, 'Eslöv, Sweden (172 73)');
		addLine(doc, 'ap@point65.com');
		currentY += JUMP_LINE;

		// Pay To
		addLine(doc, 'PAY TO:', true);
		addLine(doc, 'Benjamin Pia');
		addLine(doc, 'Calle Colina Blanca 8');
		addLine(doc, 'Málaga, Spain (29640)');
		addLine(doc, 'benjaminpia.dev@gmail.com');
		addLine(doc, 'IBAN: ES79 0081 0596 6200 0282 3694');
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

		// Table row
		doc.setFont(FONT_FAMILY, 'normal');
		doc.text('Full Stack Developer Services', colDescX, currentY);
		doc.text('€4417', colUnitPriceX, currentY);
		doc.text('1', colQtyX, currentY);
		doc.text('€4417', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE * 2;

		// Totals
		doc.text('SUBTOTAL', colQtyX, currentY);
		doc.text('€4417', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.text('Tax', colQtyX, currentY);
		doc.text('€0', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE;

		doc.setFont(FONT_FAMILY, 'bold');
		doc.text('TOTAL', colQtyX, currentY);
		doc.text('€4417', colTotalX, currentY, { align: 'right' });
		currentY += JUMP_LINE * 2;

		// Footer note
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

<h1>Taxes</h1>
<button on:click={downloadPdf}> Download Invoice </button>
