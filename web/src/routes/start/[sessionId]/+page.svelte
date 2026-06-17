<script lang="ts">
	import init from '$lib/pkg/larrey';
	import { onMount } from 'svelte';
	import type { PageProps } from './$types';
	import { start_game_with_session } from '$lib/pkg/larrey';

	const { params }: PageProps = $props();

	const { sessionId } = $derived(params);

	onMount(async () => {
		await init();
		start_game_with_session(sessionId);
	});
</script>

<div class="spaced game">
	<canvas id="game-canvas"></canvas>
	<div class="instructions">⬅️ and ➡️ arrows to move</div>
</div>

<style>
	.instructions {
		margin: 32px 0;
		color: white;
	}
	.game {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
