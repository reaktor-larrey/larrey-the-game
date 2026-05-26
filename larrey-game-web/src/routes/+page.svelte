<script lang="ts">
	import init from '$lib/pkg/larrey';

	let started = $state(false);
</script>

{#if !started}
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
{/if}

<div class="spaced">
	<canvas id="game-canvas"></canvas>
</div>

<style>
	.big-button {
		font-size: 18px;
		padding: 16px;
		display: flex;
		border-radius: 8px;
		gap: 8px;
		align-items: center;
		text-transform: uppercase;
	}

	.spaced {
		margin: 16px 0;
	}
</style>
