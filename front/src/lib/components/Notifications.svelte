<script lang="ts">
	import { notifications } from '$lib/stores';

	const handleClick = (id: number) => {
		notifications.remove(id);
	};
</script>

<div class="notifications">
	{#each $notifications as notification}
		<div class={`notification notification-${notification.type}`}>
			<p>{notification.message}</p>
			<button on:click={() => handleClick(notification.id)}>Close</button>
		</div>
	{/each}
</div>
