<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { requests } from '$lib/RequestsStore';
	import type { Collection, Request } from '$lib/Request';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	const formData = {
		name: '',
		url: '',
		method: ''
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		let new_request: Request = {
			name: formData.name,
			url: formData.url,
			method: formData.method,
			collection_name: 'orphan'
		};

		invoke('add_request', { request: new_request });
		requests.update((value) => {
			value.orphaned_requests.push(new_request);
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
				<input class="input" type="tel" bind:value={formData.method} placeholder="Select Request Method" />
			</label>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Create</button>
		</footer>
	</div>
{/if}