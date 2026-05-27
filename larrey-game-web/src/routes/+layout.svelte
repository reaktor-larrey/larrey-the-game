<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Larrey the Game</title>
</svelte:head>

{@render children()}

<style>
	:global(body) {
		font-family: monospace;
		background-color: #233234;
	}
</style>
