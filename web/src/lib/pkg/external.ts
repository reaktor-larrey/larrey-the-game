import { leaderboardServiceInstance } from '$lib/services/leaderboard.service';

export async function submit_score(score: number) {
	console.log('should submit score', score, typeof score);

	await leaderboardServiceInstance.updateScore('testonly', score);
}
