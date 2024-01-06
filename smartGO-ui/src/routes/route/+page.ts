import { getAllRoutes } from '$lib/api';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    let routes = await getAllRoutes(fetch);
    return  { routes }
}) satisfies PageLoad;