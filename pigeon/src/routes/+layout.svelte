<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import { initializeStores, storePopup, Toast } from '@skeletonlabs/skeleton';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { onMount } from 'svelte';
	import { autoModeWatcher } from '@skeletonlabs/skeleton';

	onMount(() => {
		autoModeWatcher();
	});

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
	initializeStores();
</script>

<Toast />
<Header />
<div class="flex h-screen flex-row gap-2 overflow-hidden">
	<div class="flex h-screen w-full flex-col overflow-y-auto overscroll-none p-2">
		<Sidebar />
		<div>
			<slot />
		</div>
	</div>
</div>
