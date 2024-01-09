<script lang="ts">
    import { apiTimeToLocalTime, dateToApiDate, nullToEmptyString, stopTimeHasActualArrivalTime, timeToApiTime } from '$lib/utils';
	import { onMount } from 'svelte';
	import type { StopTimeResponse } from '$lib/models';
	import { getStopTimes } from '$lib/api';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';

    let stopTimesResponse: StopTimeResponse;

    onMount(async () => {
        let stopId = $page.url.searchParams.get('stopId');
        let routeId = $page.url.searchParams.get('routeId');
        if (!stopId || !routeId) {
            goto('/');
            return;
        }
        let date = new Date();
        let currentTime = date.toLocaleTimeString("en-US", { timeZone: "America/Toronto"});
        let currentDate = date.toLocaleDateString();
        stopTimesResponse = await getStopTimes(fetch, stopId, routeId, dateToApiDate(currentDate), timeToApiTime(currentTime));
    });
</script>

{#if stopTimesResponse}
    <div>
        <h2 class="h2">{stopTimesResponse.stopName} - {stopTimesResponse.routeName} Line</h2>
    </div>
    {#if stopTimesResponse.stopTimes.length === 0}
    <div class="card px-2 py-5 my-2">
        <h5 class="h5 font-bold text-center">No trains arriving today</h5>
    </div>
    {/if}

    {#each stopTimesResponse.stopTimes as stopTime}
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
{/if}
