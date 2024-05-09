import type { LayoutServerLoad } from './$types';

export const load = (async ({ params }) => {
	return {
		gameId: params.gameId
	};
}) satisfies LayoutServerLoad;
