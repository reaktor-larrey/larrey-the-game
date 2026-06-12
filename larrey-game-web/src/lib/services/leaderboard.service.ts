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

	async updateScore(_playerId: string, _newScore: number): Promise<void> {
		//
	}
}

export const leaderboardServiceInstance = new LeaderboardService();
