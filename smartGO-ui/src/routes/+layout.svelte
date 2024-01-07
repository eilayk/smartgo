<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, initializeStores, getDrawerStore, Drawer } from '@skeletonlabs/skeleton';

	initializeStores();
	const drawerStore = getDrawerStore();
	const drawerOpen = () => {
		drawerStore.open();
	};
	const drawerClose = () => {
		drawerStore.close();
	};
</script>

<Drawer>
	<nav class="list-nav p-4">
		<ul>
			<li><a href="/" on:click={drawerClose}>Select stop</a></li>
			<li><a href="/route/" on:click={drawerClose}>Select route</a></li>
		</ul>
	</nav>
</Drawer>

<AppShell slotSidebarLeft="bg-surface-500/5 w-0 lg:w-64">
	<AppBar slotDefault="place-self-center">
		<svelte:fragment slot="lead">
			<div class="flex items-center">
				<button on:click={drawerOpen} class="lg:hidden btn btn-sm mr-4">
					<span>
						<svg viewBox="0 0 100 80" class="fill-token w-4 h-4">
							<rect width="100" height="20" />
							<rect y="30" width="100" height="20" />
							<rect y="60" width="100" height="20" />
						</svg>
					</span>
				</button>
				<strong class="text-xl">smartGO</strong>
			</div>
		</svelte:fragment>
		<div>
			<a class="hidden lg:inline hover:bg-surface-600 p-4 text-lg" href="/">Stops</a>
			<a class="hidden lg:inline hover:bg-surface-600 p-4 text-lg" href="/route/">Routes</a>
		</div>
	</AppBar>
	<div class="w-full h-full overflow-y-auto">
		<main class="container mx-auto my-3 max-w-screen-sm">
			<slot />
		</main>
	</div>
</AppShell>
