<script lang="ts">
	import { onMount } from 'svelte';
	import { leaderboardServiceInstance } from '../services/leaderboard.service';
	import type { Score } from '$lib/types';

	const leaderboardService = leaderboardServiceInstance;

	let scores: Score[] = $state([]);

	onMount(async () => {
		scores = await leaderboardService.getScores();
		console.log('got scores!', scores);
	});
</script>

<h1>Leaderboard</h1>
<ol>
	{#each scores as { playerId, score } (playerId)}
		<li>{playerId.toUpperCase()}: {score}</li>
	{/each}
</ol>
