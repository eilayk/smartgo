import { getStopsPerRoute } from '$lib/api';
import type { PageLoad } from './$types';

export const load = (async ({ fetch, params }) => {
    let stops = await getStopsPerRoute(fetch, params.routeId);
    return {stops, routeId: params.routeId};
}) satisfies PageLoad;