<script lang="ts">
	import Leaderboard from '$lib/components/Leaderboard.svelte';
	import init from '$lib/pkg/larrey';

	let started = $state(false);
</script>

<main>
	{#if !started}
		<Leaderboard />

		<div class="spaced">
			<button
				class="big-button"
				onclick={() => {
					init();
					started = true;
				}}
			>
				<div>Start game</div>
				<div>▶️</div>
			</button>
		</div>
	{:else}
		<div class="spaced game">
			<canvas id="game-canvas"></canvas>
			<div class="instructions">⬅️ and ➡️ arrows to move</div>
		</div>
	{/if}
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.big-button {
		font-size: 18px;
		padding: 16px;
		display: flex;
		border-radius: 8px;
		gap: 8px;
		align-items: center;
		text-transform: uppercase;
		cursor: pointer;
		&:hover {
			background-color: burlywood;
			transform: scale(1.05);
		}
	}

	.spaced {
		margin: 16px 0;
	}

	.game {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.instructions {
		margin: 32px 0;
		color: white;
	}
</style>
