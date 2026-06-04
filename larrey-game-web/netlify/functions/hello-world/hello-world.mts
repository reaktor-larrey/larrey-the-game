import type { Context } from '@netlify/functions';
import { getStore } from '@netlify/blobs';

type PlayerResult = {
	name: string;
	score: number;
};

export default async (req: Request, context: Context) => {
	const store = getStore('scores');

	const { name, score } = (await req.json()) as PlayerResult;

	try {
		const result = await store.set(name, score.toString());

		return new Response(
			JSON.stringify({
				message: `Congratulations, "${name}" for entering the score ${score} into the leaderboard`,
				result
			}),

			{ status: 201 }
		);
	} catch (e) {
		return new Response(
			JSON.stringify({
				message: 'Failed to enter score into leaderboard',
				error: e instanceof Error ? e.message : String(e)
			}),
			{ status: 500 }
		);
	}
};
