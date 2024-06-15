<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import { initializeStores, storePopup, Toast } from '@skeletonlabs/skeleton';
	import type { Requests } from '$lib/Models';
	import { requests } from '$lib/RequestsStore';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();

	let requests_result: Requests;

	requests.subscribe((value) => {
		requests_result = value;
	});
</script>

<Toast />
<Header />
<div class="grid grid-cols-10 gap-2 h-screen overflow-hidden">
	<div class="col-span-2 overflow-y-auto h-screen overscroll-none p-2">
		<Sidebar {requests_result} />
	</div>
	<div class="col-span-8 overflow-y-auto h-screen overscroll-none">
		<slot />
	</div>
</div>
