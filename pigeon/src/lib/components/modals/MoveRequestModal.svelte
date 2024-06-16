<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { collections_store } from '$lib/CollectionStore';
	import type { CollectionMap, Collections, Request } from '$lib/Models';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	let collections: Collections;

	collections_store.subscribe((value) => {
		collections = value;
	});

	const formData = {
		collection_name: ''
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		let request: Request = $modalStore[0].meta.request;

		let original_collection_name = request.collection_name;
		invoke('delete_request', { request: request });

		request.collection_name = formData.collection_name;
		invoke('add_request', { request: request });

		collections_store.update((value) => {
			if (original_collection_name === 'orphan') {
				value.orphan_requests.delete(request.name);
			} else {
				let collection: CollectionMap | undefined = value.collections.get(original_collection_name);
				if (collection) {
					collection.requests.delete(request.name);
				}
			}
			if (request.collection_name === 'orphan') {
				value.orphan_requests.set(request.name, request);
			} else {
				let collection: CollectionMap | undefined = value.collections.get(request.collection_name);
				if (collection) {
					collection.requests.set(request.name, request);
				}
			}
			value = value;
			return value;
		});
		modalStore.close();
	}

	function map_collection_name(current_collection_name: string) {
		if (current_collection_name === 'orphan') {
			return 'No Collection';
		}
		return current_collection_name;
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Move Request</header>
		<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
			<p>Current Collection:  {map_collection_name($modalStore[0].meta.request.collection_name)}</p>
			<select bind:value={formData.collection_name} class="select mr-5 mt-5 ml-2 p-2 lg:inline-block" id="method">
				<option value="orphan">No collection</option>
				{#each Array.from(collections.collections) as [collection_name, _collection]}
					<option value={collection_name}>{collection_name}</option>
				{/each}
			</select>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Move</button>
		</footer>
	</div>
{/if}