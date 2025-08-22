<script lang="ts">
	import { notification } from '$lib/stores';
</script>

{#if $notification}
	<div class={`notification notification-${$notification.type}`}>
		<p>{$notification.message}</p>
	</div>
{/if}
