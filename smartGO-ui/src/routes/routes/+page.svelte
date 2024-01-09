<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { getAllRoutes } from '$lib/api';
	import type { Route } from '$lib/models';
    
    let routes: Route[] = [];
    onMount(async () => {
        routes = await getAllRoutes(fetch);
    });

    const onRouteClick = (routeId: string) => {
        goto(`/route?routeId=${routeId}`);
    }
</script>

<div class="card p-4">
    <ul class="list">
        {#each routes as route}
            <li class="p-2 m-1 hover:bg-surface-400">
                <button on:click={() => onRouteClick(route.routeId)}>
                    {route.routeName}
                </button>
            </li>
        {/each}
    </ul>
</div>