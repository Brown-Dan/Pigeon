<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import { initializeStores, storePopup, Toast } from '@skeletonlabs/skeleton';
	import type { Collections } from '$lib/Models';
	import { collections_store } from '$lib/CollectionStore';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();

	let collections: Collections;

	collections_store.subscribe((value) => {
		collections = value;
	});
</script>

<Toast />
<Header />
<div class="grid grid-cols-10 gap-2 h-screen overflow-hidden">
	<div class="col-span-2 overflow-y-auto h-screen overscroll-none p-2">
		<Sidebar />
	</div>
	<div class="col-span-8 overflow-y-auto h-screen overscroll-none">
		<slot />
	</div>
</div>
