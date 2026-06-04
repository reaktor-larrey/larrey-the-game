import type { Config } from '@netlify/functions';
import { getStore } from '@netlify/blobs';

export default async (req: Request) => {
	try {
		const store = getStore('scores');

		const scores = await store.get('scores');

		return new Response(JSON.stringify(scores), { status: 200 });
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
