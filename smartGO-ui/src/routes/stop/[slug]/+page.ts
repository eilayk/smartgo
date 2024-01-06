import { getRoutesPerStop } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (async ({ fetch, params }) => {
    let routes = await getRoutesPerStop(fetch, params.slug);
    if (routes.length === 1) {
        throw redirect(302, `/route/${routes[0].routeId}/stop/${params.slug}`);
    }

    return {
        routes: routes,
        stopId: params.slug
    };
}) satisfies PageLoad;