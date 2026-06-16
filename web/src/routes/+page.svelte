<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Leaderboard from '$lib/components/Leaderboard.svelte';
</script>

<main>
	<Leaderboard />

	<div class="spaced">
		<button
			class="big-button"
			onclick={async () => {
				const res = await fetch('/.netlify/functions/start-game', { method: 'POST' });
				if (res.ok) {
					const json = (await res.json()) as { sessionId: string };
					goto(resolve('/start/[sessionId]', { sessionId: json.sessionId }));
				}
			}}
		>
			<div>Start game</div>
			<div>▶️</div>
		</button>
	</div>
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
</style>
