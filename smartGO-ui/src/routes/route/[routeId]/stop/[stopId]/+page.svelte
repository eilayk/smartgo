<script lang="ts">
    import { apiTimeToLocalTime, nullToEmptyString, stopTimeHasActualArrivalTime } from '$lib/utils';
import type { PageData } from './$types';
    export let data: PageData;
</script>
<!-- 
<div class="container h-full mx-auto flex justify-center items-center flex-col">
    {#each data.stopTimes as stopTime}
        <div class="card">
            <h3>{stopTime.headsign}</h3>
            <p>{stopTime.arrivalTime}</p>
        </div>
    {/each}
</div> -->

<div>
    <h2 class="h2">{data.stopName} - {data.routeName} Line</h2>
</div>

{#if data.stopTimes.length === 0}
<div class="card px-2 py-5 my-2">
    <h5 class="h5 font-bold text-center">No trains arriving today</h5>
</div>
{/if}

{#each data.stopTimes as stopTime}
<div class="card p-2 my-2">
    <h5 class="h5 font-bold">{stopTime.headsign}</h5>
    <p>
        Arriving at:
        {#if !stopTime.actualArrivalTime} 
            <span class="text-success-500 font-bold">{apiTimeToLocalTime(stopTime.arrivalTime)}</span>
        {:else}
            <s >
                <span class="text-error-500 font-bold">{apiTimeToLocalTime(stopTime.arrivalTime)} </span>
            </s>
            <span class="text-warning-500 font-bold">{apiTimeToLocalTime(stopTime.actualArrivalTime)}</span>
        {/if}
    </p>
    <p>
        platform: {stopTime.actualPlatform ? nullToEmptyString(stopTime.actualPlatform) : nullToEmptyString(stopTime.scheduledPlatform)}
    </p>
    <p>train length: {nullToEmptyString(stopTime.trainLength)}</p>
</div>
{/each}