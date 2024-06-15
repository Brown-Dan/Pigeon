<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { collections_store } from '$lib/CollectionStore';
	import { isOrphan, type Request } from '$lib/Models';
	import { open_tabs_store } from '$lib/OpenTabStore';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	const formData = {
		name: $modalStore[0].meta.request.name
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);

		let updated_request: Request = $modalStore[0].meta.request;
		let original_name = updated_request.name;

		invoke('delete_request', { request: updated_request });
		updated_request.name = formData.name;
		invoke('add_request', { request: updated_request });

		collections_store.update((value) => {
			if (isOrphan(updated_request)) {
				value.orphan_requests.get(original_name)!.name = updated_request.name;
				console.log(value.orphan_requests.get(original_name))
			} else {
				value.collections.get(updated_request.collection_name)!.requests.get(original_name)!.name =
					updated_request.name;
			}
			return value;
		});
		open_tabs_store.update((value) => {
			let original_request = value.filter(request => request.name === original_name).at(0);
			if (original_request) {
				original_request.name = updated_request.name;
			}
			return value;
		})
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Rename Request</header>
		<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
			<label class="label">
				<span>Name</span>
				<input class="input" type="text" bind:value={formData.name} placeholder="Enter new request name..." />
			</label>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Rename</button>
		</footer>
	</div>
{/if}