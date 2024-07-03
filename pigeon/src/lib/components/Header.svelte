<script lang="ts">
	import {
		AppBar,
		getModalStore,
		initializeStores,
		LightSwitch,
		Modal,
		type ModalComponent,
		type ModalSettings,
		storePopup
	} from '@skeletonlabs/skeleton';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import HistoryModal from '$lib/components/modals/HistoryModal.svelte';
	import hotkeys from 'hotkeys-js';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	initializeStores();

	const modalStore = getModalStore();

	const modalRegistry: Record<string, ModalComponent> = {
		historyModal: { ref: HistoryModal }
	};

	const historyModal: ModalSettings = {
		type: 'component',
		component: 'historyModal'
	};

	import { CircleHelp, Clock, CircleUser, Settings } from 'lucide-svelte';
</script>

<Modal components={modalRegistry} />

<AppBar>
	<svelte:fragment slot="lead">
		<img
			alt="Pigeon Logo"
			class="ml-3 max-h-12 max-w-28"
			src="http://www.i2clipart.com/cliparts/f/5/c/a/clipart-pigeon-64x64-f5ca.png"
		/>
	</svelte:fragment>
	Pigeon
	<svelte:fragment slot="trail">
		<LightSwitch />
		<div class="variant-filled btn-group">
			<button type="button" class="variant-filled btn-sm">
				<CircleHelp />
			</button>
			<button
				on:click={() => modalStore.trigger(historyModal)}
				type="button"
				class="variant-filled btn-sm"
			>
				<Clock />
			</button>
			<button type="button" class="variant-filled btn-sm">
				<CircleUser />
			</button>
			<button type="button" class="variant-filled btn-sm">
				<Settings />
			</button>
		</div>
	</svelte:fragment>
</AppBar>
