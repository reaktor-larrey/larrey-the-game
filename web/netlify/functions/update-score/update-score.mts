import type { Config } from '@netlify/functions';
import { getStore } from '@netlify/blobs';

type PlayerResult = {
	name: string;
	score: number;
};

export default async (req: Request) => {
	const store = getStore('scores');

	const { name, score } = (await req.json()) as PlayerResult;

	console.log({ store });

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
		return new Response(e instanceof Error ? e.message : String(e), {
			status: 500,
			statusText: e instanceof Error ? e.message : String(e)
		});
	}
};

export const config: Config = {
	method: 'POST'
};
