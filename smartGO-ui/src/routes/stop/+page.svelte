<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Route } from '$lib/models';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { getRoutesPerStop } from '$lib/api';

    let stopId: string;
    let routes: Route[] = [];

    onMount(async () => {
        const stopIdOrNull = $page.url.searchParams.get('stopId');
        if (!stopIdOrNull) {
            goto('/');
        } else {
            stopId = stopIdOrNull;
        }
        let routesData = await getRoutesPerStop(fetch, stopId);
        if (routesData.length == 1) {
            goto(`stopTime?stopId=${stopId}&routeId=${routesData[0].routeId}`, { replaceState: true })
            return;
        }
        routes = routesData;
    });

    const onRouteClick = (route: Route) => {
        console.log(route);
        goto(`stopTime?stopId=${stopId}&routeId=${route.routeId}`)
    }

</script>

{#if routes.length > 0}
    <div class="card p-4">
        <ul class="list">
            {#each routes as route}
                <li class="p-2 m-1 hover:bg-surface-400">
                    <button on:click={() => onRouteClick(route)}>
                        {route.routeName}
                    </button>
                </li>
            {/each}
        </ul>
    </div>
{/if}
