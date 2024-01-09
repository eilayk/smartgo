<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { Stop } from '$lib/models';
	import { getStopsPerRoute } from '$lib/api';
	import { page } from '$app/stores';
    
    let stops: Stop[] = [];
    let routeId: string;

    onMount(async () => {
        let routeIdOrNull = $page.url.searchParams.get('routeId');
        if (!routeIdOrNull) {
            goto('/routes');
        } else {
            routeId = routeIdOrNull;
        }
        stops = await getStopsPerRoute(fetch, routeId);
    });

    const onStopClick = (routeId: string, stopId: string) => {
        goto(`stopTime?stopId=${stopId}&routeId=${routeId}`)
    }
</script>

{#if stops.length > 0} 
    <div class="card p-4">
        <ul class="list">
            {#each stops as stop}
                <li class="p-2 m-1 hover:bg-surface-400">
                    <button on:click={() => onStopClick(routeId, stop.stopId)}>
                        {stop.stopName}
                    </button>
                </li>
            {/each}
        </ul>
    </div>
{/if}
