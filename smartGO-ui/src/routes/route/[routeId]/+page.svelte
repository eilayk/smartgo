<script lang="ts">
	import { goto } from '$app/navigation';
    import type { PageData } from './$types';
    
    export let data: PageData;

    const onStopClick = (routeId: string, stopId: string) => {
        goto(`/route/${routeId}/stop/${stopId}`);
    }
</script>

<div class="card p-4">
    <ul class="list">
        {#each data.stops as stop}
            <li class="p-2 m-1 hover:bg-surface-400">
                <button on:click={() => onStopClick(data.routeId, stop.stopId)}>
                    {stop.stopName}
                </button>
            </li>
        {/each}
    </ul>
</div>