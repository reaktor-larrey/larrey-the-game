import { getStore } from '@netlify/blobs';
import { Context } from '@netlify/functions';
import { v4 as uuidv4 } from 'uuid';

export default async (_request: Request, _context: Context) => {
	const store = getStore('sessions');

	const sessionId = uuidv4();

	try {
		const result = await store.set(sessionId, Date.now().toString());
		return new Response(JSON.stringify({ sessionId, result }), {
			status: 201,
			headers: { 'Content-Type': 'application/json' }
		});
	} catch (e) {
		return new Response(e instanceof Error ? e.message : String(e), {
			status: 500,
			statusText: e instanceof Error ? e.message : String(e)
		});
	}
};
