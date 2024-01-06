<script lang="ts">
	import { goto } from '$app/navigation';
    import type { PageData } from './$types';
    
    export let data: PageData;

    const onRouteClick = (routeId: string) => {
        goto(`/route/${routeId}`);
    }
</script>

<div class="card p-4">
    <ul class="list">
        {#each data.routes as route}
            <li class="p-2 m-1 hover:bg-surface-400">
                <button on:click={() => onRouteClick(route.routeId)}>
                    {route.routeName}
                </button>
            </li>
        {/each}
    </ul>
</div>