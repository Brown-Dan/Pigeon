<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { requests } from '$lib/RequestsStore';
	import type { Request, Requests } from '$lib/Models';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	let request_results: Requests;

	requests.subscribe((value) => {
		request_results = value;
	});

	const formData = {
		collection_name: ''
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		let req: Request = $modalStore[0].meta.request;
		let original_collection = req.collection_name;
		invoke('delete_request', { request: req });
		req.collection_name = formData.collection_name;
		invoke('add_request', { request: req });
		requests.update((value) => {
			if (original_collection === 'orphan') {
				let idx = value.orphaned_requests.map(r => r.name).indexOf(req.name);
				value.orphaned_requests.splice(idx, 1);
				value.orphaned_requests = value.orphaned_requests.filter(Boolean);
			} else {
				let col: Request[] = value.collections.filter(c => c.name === original_collection)[0].requests
				let idx = col.map(r => r.name).indexOf(req.name)
				col.splice(idx, 1)
				col = col;
			}

			if (req.collection_name === 'orphan') {
				value.orphaned_requests.push(req);
				value.orphaned_requests = value.orphaned_requests;
			} else {
				console.log(value.collections);
				console.log(req.collection_name);
				value.collections.filter(c => c.name === req.collection_name)[0].requests.push(req);
				value.collections = value.collections;
			}
			return value;
		});
		window.dispatchEvent(new CustomEvent('renameRequest', { detail: (req.name, original_collection) }));
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
				{#each request_results.collections as collection}
					<option value={collection.name}>{collection.name}</option>
				{/each}
			</select>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Move</button>
		</footer>
	</div>
{/if}