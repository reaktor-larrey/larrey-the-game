<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { leaderboardServiceInstance } from '$lib/services/leaderboard.service';
	import type { PageProps } from './$types';

	const { params }: PageProps = $props();
	const score = $derived(parseInt(params.score));

	const leaderboardService = leaderboardServiceInstance;

	let userName = $state('player1');

	async function onSubmit(e: Event) {
		e.preventDefault();
		await leaderboardService.updateScore(userName, score);
		await goto(resolve('/'));
	}
</script>

<h1>Submit your high score!</h1>

<form onsubmit={onSubmit}>
	<div>
		<input type="text" name="name" bind:value={userName} />
		{score}
	</div>
	<input type="submit" value="Submit" />
</form>
