<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import { autoModeWatcher, initializeStores, storePopup, Toast } from '@skeletonlabs/skeleton';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		autoModeWatcher();
	});

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();
</script>

<Toast />
<div class="flex h-screen flex-col overflow-hidden">
	<div class="flex-none h-min mb-1 overflow-y-auto">
		<Header />
	</div>
	<div class="flex-none min-h-min max-h-4/20 overflow-y-hidden">
		<Sidebar />
	</div>
	<div class="flex-grow overflow-y-hidden">
		<slot />
	</div>
</div>