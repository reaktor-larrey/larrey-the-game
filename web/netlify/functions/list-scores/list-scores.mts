import type { Config } from '@netlify/functions';
import { getStore } from '@netlify/blobs';
import { type Score } from '../../../src/lib/services/leaderboard.service';

export default async (_req: Request) => {
	try {
		const store = getStore('scores');

		const contents = await store.list();
		const scores: Score[] = await Promise.all(
			contents.blobs.map(async (b) => {
				const entry = await store.get(b.key);
				console.log({ entry });
				return {
					playerId: b.key,
					score: JSON.parse(entry.toString())
				};
			})
		);

		console.log({ store, scores });

		return new Response(JSON.stringify({ scores }), { status: 200 });
	} catch (e) {
		return new Response(e instanceof Error ? e.message : String(e), {
			status: 500,
			statusText: e instanceof Error ? e.message : String(e)
		});
	}
};

export const config: Config = {
	method: 'GET'
};
