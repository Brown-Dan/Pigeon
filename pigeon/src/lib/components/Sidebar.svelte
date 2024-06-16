<script lang="ts">
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import {
		getModalStore,
		initializeStores,
		Modal,
		type ModalComponent,
		type ModalSettings,
		popup,
		type PopupSettings,
		storePopup,
		TreeView,
		TreeViewItem
	} from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { type CollectionMap, deep_copy, isOrphan, type Request } from '$lib/Models';
	import AddCollectionModal from '$lib/components/modals/AddCollectionModal.svelte';
	import AddRequestModal from '$lib/components/modals/AddRequestModal.svelte';
	import { limit_chars, method_to_abb, method_to_colour } from '$lib/MethodUtils';
	import RenameRequestModal from '$lib/components/modals/RenameRequestModal.svelte';
	import MoveRequestModal from '$lib/components/modals/MoveRequestModal.svelte';
	import { collections_store } from '$lib/CollectionStore';
	import { increment, open_tabs } from '$lib/TabStore';

	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	initializeStores();

	const modalStore = getModalStore();
	let selected_collection: String;
	let selected_request: Request;

	const modalRegistry: Record<string, ModalComponent> = {
		addCollectionModal: { ref: AddCollectionModal },
		addRequestModal: { ref: AddRequestModal },
		renameRequestModal: { ref: RenameRequestModal },
		moveRequestModal: { ref: MoveRequestModal }
	};

	const collectionSettingsPopup: PopupSettings = {
		event: 'click',
		target: 'collectionSettingsPopup',
		placement: 'right',
		closeQuery: 'button'
	};

	const requestSettingsPopup: PopupSettings = {
		event: 'click',
		target: 'requestSettingsPopup',
		placement: 'right',
		closeQuery: 'button'
	};

	const addCollectionModal: ModalSettings = {
		type: 'component',
		component: 'addCollectionModal'
	};

	async function move_request() {
		const moveRequestModal: ModalSettings = {
			type: 'component',
			component: 'moveRequestModal',
			meta: { request: selected_request }
		};
		modalStore.trigger(moveRequestModal);
	}

	async function add_request() {
		const addRequestModal: ModalSettings = {
			type: 'component',
			component: 'addRequestModal',
			meta: { name: selected_collection }
		};
		modalStore.trigger(addRequestModal);
	}

	async function rename_request() {
		const renameRequestModal: ModalSettings = {
			type: 'component',
			component: 'renameRequestModal',
			meta: { request: selected_request }
		};
		modalStore.trigger(renameRequestModal);
	}

	const confirmDeleteModal: ModalSettings = {
		type: 'confirm',
		title: 'Please Confirm',
		body: 'Are you sure you wish to proceed with deleting collection?',
		response: (response: boolean) => {
			if (response) {
				invoke('delete_collection', { collectionName: selected_collection });
				collections_store.update((value) => {
					value.collections.delete(selected_collection);
					return value;
				});
				open_tabs.update((value) => {
					let index = value.map(req => req.collection_name).indexOf(selected_collection.toString());
					if (index !== -1) {
						value.splice(index, 1);
					}
					return value;
				});
			}
		}
	};

	const confirmDeleteRequestModal: ModalSettings = {
		type: 'confirm',
		title: 'Please Confirm',
		body: 'Are you sure you wish to proceed with deleting request?',
		response: (r: boolean) => {
			if (r === true) {
				invoke('delete_request', { request: selected_request });
				collections_store.update((value) => {
					if (isOrphan(selected_request)) {
						value.orphan_requests.delete(selected_request.name);
					} else {
						let collection: CollectionMap | undefined = value.collections.get(selected_request.collection_name);
						if (collection) {
							collection.requests.delete(selected_request.name);
						}
					}
					return value;
				});
				open_tabs.update((value) => {
					let index = value.map(req => req.name).indexOf(selected_request.name);
					if (index !== -1) {
						value.splice(index, 1);
					}
					return value;
				});
			}
		}
	};

	function clone_request() {
		const request_copy =  deep_copy(selected_request);
		request_copy.name = request_copy.name + ' clone';
		invoke('add_request', { request: request_copy });
		collections_store.update((value) => {
			if (isOrphan(request_copy)) {
				value.orphan_requests.set(request_copy.name, request_copy);
			} else {
				let collection = value.collections.get(request_copy.collection_name);
				if (collection) {
					collection.requests.set(request_copy.name, request_copy);
				}
			}
			return value;
		});
	}

	function open_request_tab(request: Request) {
		open_tabs.update((value) => {
			if (value.filter(req => req === request).length === 0) {
				value.push(request);
				increment();
			}
			return value;
		});
	}

	async function delete_request() {
		collectionSettingsPopup.closeQuery;
		modalStore.trigger(confirmDeleteRequestModal);
	}

	async function delete_collection() {
		collectionSettingsPopup.closeQuery;
		modalStore.trigger(confirmDeleteModal);
	}
</script>
<Modal components={modalRegistry} />

<div class="z-50 card w-48 shadow-xl py-2 text-center" data-popup="collectionSettingsPopup">
	<div class="btn-group-vertical min-w-full">
		<button on:click={add_request}>Add Request</button>
		<button>Environments</button>
		<button on:click={delete_collection}>Delete</button>
	</div>
	<div class="arrow bg-surface-100-800-token" />
</div>

<div class="z-50 card w-48 shadow-xl py-2 text-center" data-popup="requestSettingsPopup">
	<div class="btn-group-vertical min-w-full">
		<button on:click={rename_request}>Rename</button>
		<button on:click={delete_request}>Delete</button>
		<button on:click={clone_request}>Clone</button>
		<button on:click={move_request}>Move</button>
	</div>
	<div class="arrow bg-surface-100-800-token" />
</div>

<div class="overflow-hidden">
	<select class="select mr-5 mt-5 ml-2 p-2 text-xs hidden lg:inline-block w-24" id="method">
		<option value="GET">LOCAL</option>
		<option value="PUT">PREPROD</option>
		<option value="PATCH">PROD</option>
	</select>

	<div class="ml-2 btn-group variant-filled mt-4 hidden lg:inline-block">
		<button on:click={() => modalStore.trigger(addCollectionModal)} class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
					 class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round"
							d="M12 10.5v6m3-3H9m4.06-7.19-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
			</svg>
		</button>
		<button on:click={add_request} class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
					 class="size-6">
				<path stroke-linecap="round" stroke-linejoin="round"
							d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m3.75 9v6m3-3H9m1.5-12H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
			</svg>
		</button>
		<button class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
					 class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round"
							d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418" />
			</svg>
		</button>
		<button class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
					 class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round"
							d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z" />
			</svg>
		</button>
	</div>
</div>
<div class="overflow-y-auto overscroll-none mb-5 overflow-x-hidden">
	{#if $collections_store}
		<TreeView class="hidden lg:block text-xs">
			{#each Array.from($collections_store.collections) as [collection_name, collection]}
				<TreeViewItem class="my-0.5">
					<svelte:fragment slot="lead">
						<button on:click={() => selected_collection = collection_name} use:popup={collectionSettingsPopup}>
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M2.25 12.75V12A2.25 2.25 0 0 1 4.5 9.75h15A2.25 2.25 0 0 1 21.75 12v.75m-8.69-6.44-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
							</svg>
						</button>
					</svelte:fragment>
					<div class="flex items-center justify-between">
						<span class="flex items-center whitespace-nowrap overflow-hidden text-sm">{collection_name}</span>
					</div>
					<svelte:fragment slot="children">
						{#each Array.from(collection.requests) as [request_name, request]}
							<TreeViewItem class="my-0.5" on:click={() => open_request_tab(request)}>
								<svelte:fragment slot="lead">
									<button on:click={(event) => {event.stopPropagation(); selected_request = request;}}
													use:popup={requestSettingsPopup}>
										<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
												 stroke="currentColor" class="size-6">
											<path stroke-linecap="round" stroke-linejoin="round"
														d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
										</svg>
									</button>
								</svelte:fragment>
								<div class="flex items-center justify-between">
									<div class="flex items-center">
										<span
											class="badge {method_to_colour.get(request.method)} mr-2 text-xs">{method_to_abb.get(request.method)}</span>
										<span class="overflow-hidden whitespace-nowrap text-sm">{limit_chars(request_name, 16)}</span>
									</div>
								</div>
							</TreeViewItem>
						{/each}
					</svelte:fragment>
				</TreeViewItem>
			{/each}
			{#each Array.from($collections_store.orphan_requests) as [request_name, request]}
				<TreeViewItem class="my-0.5" on:click={() => open_request_tab(request)}>
					<svelte:fragment slot="lead">
						<button on:click={(event) => {event.stopPropagation();selected_request = request;}}
										use:popup={requestSettingsPopup}>
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
							</svg>
						</button>
					</svelte:fragment>
					<span
						class="ml-0 badge { method_to_colour.get(request.method)} mr-2 text-xs">{method_to_abb.get(request.method)}</span>
					<span class="overflow-hidden whitespace-nowrap text-sm">{limit_chars(request_name, 16)}</span>
				</TreeViewItem>
			{/each}
		</TreeView>
	{/if}
</div>
