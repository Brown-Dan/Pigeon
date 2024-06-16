<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { collections_store } from '$lib/CollectionStore';
	import { isOrphan, type Request } from '$lib/Models';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	const formData = {
		name: '',
		url: '',
		method: 'GET'
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		let request: Request = {
			name: formData.name,
			url: formData.url,
			method: formData.method,
			collection_name: $modalStore[0].meta.name === undefined ? 'orphan' : $modalStore[0].meta.name,
			headers: [],
			query_params: [],
			body: {
				content: '{}',
				enabled: false
			}
		};

		invoke('add_request', { request });
		collections_store.update((value) => {
			if (isOrphan(request)) {
				value.orphan_requests.set(request.name, request);
			} else {
				let collection = value.collections.get(request.name);
				if (collection) {
					collection.requests.set(request.name, request);
				}
			}
			return value;
		});
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Create Request</header>
		<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
			<label class="label">
				<span>Name</span>
				<input class="input" type="text" bind:value={formData.name} placeholder="Enter request name..." />
			</label>
			<label class="label">
				<span>Url</span>
				<input class="input" type="text" bind:value={formData.url} placeholder="Enter request url..." />
			</label>
			<label class="label">
				<span>Method</span>
				<select class="select" bind:value={formData.method} id="method">
					<option value="GET">GET</option>
					<option value="PUT">PUT</option>
					<option value="PATCH">PATCH</option>
					<option value="DELETE">DELETE</option>
					<option value="POST">POST</option>
				</select>
			</label>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Create</button>
		</footer>
	</div>
{/if}