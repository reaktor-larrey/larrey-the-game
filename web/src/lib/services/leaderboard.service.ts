export interface Score {
	playerId: string;
	score: number;
}

interface LeaderboardServiceInterface {
	getScores(): Promise<Score[]>;
	updateScore(playerId: string, newScore: number): Promise<void>;
}

export class LeaderboardService implements LeaderboardServiceInterface {
	private cache: Record<string, Score> = {};
	async getScores(): Promise<Score[]> {
		const res = await fetch('/.netlify/functions/list-scores');
		if (res.ok) {
			const json = (await res.json()) as { scores: Score[] };
			return json.scores;
		} else {
			throw Error('Failed to fetch scores');
		}
	}

	async updateScore(name: string, score: number): Promise<void> {
		console.log('Updating score in backend...');
		const res = await fetch('/.netlify/functions/update-score', {
			method: 'POST',
			body: JSON.stringify({ name, score: score })
		});
		console.log('... result:', res.status);
	}
}

export const leaderboardServiceInstance = new LeaderboardService();
