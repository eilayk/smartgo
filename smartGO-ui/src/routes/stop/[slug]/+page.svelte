<script lang="ts">
	import { goto } from '$app/navigation';
	import type { Route } from '$lib/models';
	import { onMount } from 'svelte';
    import type { PageData } from './$types';
    export let data: PageData;

    // onMount(() => {
    //     if (data.routes.length === 1) goto(`/route/${data.routes[0].routeId}/stop/${data.stopId}`, { replaceState: true })
    // })

    const onRouteClick = (route: Route) => {
        console.log(route);
        goto(`/route/${route.routeId}/stop/${data.stopId}`)
    }

</script>

<div class="card p-4">
    <ul class="list">
        {#each data.routes as route}
            <li class="p-2 m-1 hover:bg-surface-400">
                <button on:click={() => onRouteClick(route)}>
                    {route.routeName}
                </button>
            </li>
        {/each}
    </ul>
</div>