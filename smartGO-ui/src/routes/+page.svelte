<script lang="ts">
	import { Autocomplete, type AutocompleteOption } from "@skeletonlabs/skeleton";
	import type { PageData } from "./$types";
	import type { Stop } from "$lib/models";
	import { goto } from "$app/navigation";
	export let data: PageData;
	let searchStopsInput = "";

	const onStopSelection = (event:  CustomEvent<AutocompleteOption<Stop>>) => {
		// console.log("selected", event.detail.value)
		goto(`/stop/${event.detail.value.stopId}`);
	}
</script>

<input class="input p-4" type="search" name="demo" bind:value={searchStopsInput} placeholder="Search stop..." />
<div class="card w-full overflow-y-auto p-4 my-2" tabindex="-1">
	<Autocomplete bind:input={searchStopsInput} options={data.stops} on:selection={onStopSelection} />
</div>