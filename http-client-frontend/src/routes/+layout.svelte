<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import type { PopupSettings } from '@skeletonlabs/skeleton';
	import {
		AppBar,
		getModalStore,
		initializeStores,
		Modal,
		type ModalComponent,
		type ModalSettings,
		popup,
		storePopup,
		Toast,
		TreeView,
		TreeViewItem
	} from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Request, Requests } from '$lib/Models';
	import HistoryModal from '$lib/components/modals/HistoryModal.svelte';
	import AddCollectionModal from '$lib/components/modals/AddCollectionModal.svelte';
	import { requests } from '$lib/RequestsStore';
	import { onMount } from 'svelte';
	import AddRequestModal from '$lib/components/modals/AddRequestModal.svelte';
	import Header from '$lib/components/Header.svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	initializeStores();

	let requests_result: Requests;

	onMount(async () => {
		try {
			requests_result = await invoke('get_collections', {}).then((value) => <Requests>value);
			requests.set(requests_result);
		} catch (error) {
			console.error('Error fetching history:', error);
		}
	});
	requests.subscribe((value) => {
		requests_result = value;
	});

</script>

<Toast />
<Header />
<div class="grid grid-cols-10 gap-2 h-screen overflow-y-auto">
	<Sidebar {requests_result}/>
	<div class="col-span-8 min-h-max overflow-y-auto">
		<slot />
	</div>
</div>