<script lang="ts">
	import init from '$lib/pkg/larrey';
</script>

<div>
	<button
		onclick={() => {
			init();
		}}>Start game</button
	>
</div>

<div>
	<canvas id="game-canvas"></canvas>
</div>
