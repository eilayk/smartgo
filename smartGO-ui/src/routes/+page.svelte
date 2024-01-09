<script lang="ts">
	import { Autocomplete, type AutocompleteOption } from "@skeletonlabs/skeleton";
	import type { Stop } from "$lib/models";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import { getStops } from "$lib/api";
	import { stopToAutocompleteOpton } from "$lib/utils";
	let searchStopsInput = "";

	let autoCompleteStop: AutocompleteOption[] = [];
	onMount(async () => {
		autoCompleteStop = (await getStops(fetch)).map(stop => stopToAutocompleteOpton(stop));
	});

	const onStopSelection = (event:  CustomEvent<AutocompleteOption<Stop>>) => {
		// console.log("selected", event.detail.value)
		goto(`/stop?stopId=${event.detail.value.stopId}`);
	}
</script>

<input class="input p-4" type="search" name="demo" bind:value={searchStopsInput} placeholder="Search stop..." />
<div class="card w-full overflow-y-auto p-4 my-2" tabindex="-1">
	<Autocomplete bind:input={searchStopsInput} options={autoCompleteStop} on:selection={onStopSelection} />
</div>