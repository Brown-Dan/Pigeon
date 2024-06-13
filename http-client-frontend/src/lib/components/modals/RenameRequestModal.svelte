<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { requests } from '$lib/RequestsStore';
	import type { Request } from '$lib/Models';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	const formData = {
		name: ''
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		let req: Request = $modalStore[0].meta.request;
		let original_name = req.name;
		invoke('delete_request', { request: req });
		req.name = formData.name;
		invoke('add_request', { request: req });
		requests.update((value) => {
			if (req.collection_name === 'orphan') {
				let idx = value.orphaned_requests.map(r => r.name).indexOf(original_name);
				value.orphaned_requests[idx].name = req.name
			} else {
				let col: Request[] = value.collections.filter(c => c.name === req.collection_name)[0].requests
				let idx = col.map(r => r.name).indexOf(original_name);
				col[idx].name = req.name
			}
			return value;
		});
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
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Create</button>
		</footer>
	</div>
{/if}