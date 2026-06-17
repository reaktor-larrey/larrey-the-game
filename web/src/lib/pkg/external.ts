import { goto } from '$app/navigation';
import { resolve } from '$app/paths';

export async function submit_score(score: number) {
	console.log('should submit score', score, typeof score);

	await goto(resolve('/enter/[score]', { score: score.toString() }));
}
