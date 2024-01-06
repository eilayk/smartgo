import { getStops } from '$lib/api.js';
import type { Stop } from '$lib/models.js';
import { stopToAutocompleteOpton } from '$lib/utils.js';
import type { AutocompleteOption } from '@skeletonlabs/skeleton';
import type { PageLoad } from './$types.js';

export const load = (async ({ fetch }) => {
    let stops = await getStops(fetch);
    let autocompleteOptions: AutocompleteOption[] = stops.map((stop: Stop) => stopToAutocompleteOpton(stop));
    return {
        stops: autocompleteOptions,
    };
}) satisfies PageLoad;
