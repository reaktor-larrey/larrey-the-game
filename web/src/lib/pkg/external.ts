import { goto } from '$app/navigation';
import { resolve } from '$app/paths';
import { leaderboardServiceInstance } from '$lib/services/leaderboard.service';

export async function submit_score(score: number) {
	console.log('should submit score', score, typeof score);

	await goto(resolve('/enter/[score]', { score: score.toString() }));
	// await leaderboardServiceInstance.updateScore('testonly', score);
}
