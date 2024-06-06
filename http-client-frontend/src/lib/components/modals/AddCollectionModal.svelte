<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { getModalStore } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { requests } from '$lib/RequestsStore';
	import type { Collection } from '$lib/Models';

	export let parent: SvelteComponent;
	const modalStore = getModalStore();

	const formData = {
		name: '',
		description: ''
	};

	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(formData);
		invoke("add_collection", {config: {name: formData.name, description: formData.description} })
		let collection: Collection = {
			name: formData.name,
			description: formData.description,
			requests: []
		}
		requests.update((value) => {
			value.collections.push(collection)
			return value;
		})
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="modal-example-form card p-4 w-modal shadow-xl space-y-4">
		<header class="text-2xl font-bold">Create Collection</header>
		<form class="modal-form border border-surface-500 p-4 space-y-4 rounded-container-token">
			<label class="label">
				<span>Name</span>
				<input class="input" type="text" bind:value={formData.name} placeholder="Enter collection name..." />
			</label>
			<label class="label">
				<span>Description</span>
				<input class="input" type="tel" bind:value={formData.description} placeholder="Enter collection description..." />
			</label>
		</form>
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Create</button>
		</footer>
	</div>
{/if}