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
	import { change_tab_index, current_tab_index, increment, open_tabs } from '$lib/TabStore';
	import { FileCog, FilePlus, FolderClosed, FolderPlus, Globe, Info, Menu } from 'lucide-svelte';
	import hotkeys from 'hotkeys-js';

	hotkeys("cmd+o", toggle_sidebar)

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
					let index = value
						.map((req) => req.collection_name)
						.indexOf(selected_collection.toString());
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
						let collection: CollectionMap | undefined = value.collections.get(
							selected_request.collection_name
						);
						if (collection) {
							collection.requests.delete(selected_request.name);
						}
					}
					return value;
				});
				open_tabs.update((value) => {
					let index = value.map((req) => req.name).indexOf(selected_request.name);
					if (index !== -1) {
						value.splice(index, 1);
					}
					return value;
				});
			}
		}
	};

	function clone_request() {
		const request_copy = deep_copy(selected_request);
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
			if (value.filter((req) => req === request).length === 0) {
				value.push(request);
				change_tab_index($open_tabs.length - 1);
			} else {
				current_tab_index.set($open_tabs.indexOf(request));
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

	function toggle_sidebar() {
		let collections = document.getElementById("collections");
		if (collections) {
			collections.hidden = !collections.hidden;
		}
	}
</script>

<Modal components={modalRegistry} />

<div class="card z-50 w-48 py-2 text-center shadow-xl" data-popup="collectionSettingsPopup">
	<div class="btn-group-vertical min-w-full">
		<button on:click={add_request}>Add Request</button>
		<button>Environments</button>
		<button on:click={delete_collection}>Delete</button>
	</div>
	<div class="bg-surface-100-800-token arrow" />
</div>

<div class="card z-50 w-48 py-2 text-center shadow-xl" data-popup="requestSettingsPopup">
	<div class="btn-group-vertical min-w-full">
		<button on:click={rename_request}>Rename</button>
		<button on:click={delete_request}>Delete</button>
		<button on:click={clone_request}>Clone</button>
		<button on:click={move_request}>Move</button>
	</div>
	<div class="bg-surface-100-800-token arrow" />
</div>

<div class="flex items-center overflow-hidden mb-2">
	<select class="select ml-2 mr-5 mt-5 hidden w-24 p-2 text-xs md:flex" id="method">
		<option value="GET">LOCAL</option>
		<option value="PUT">PREPROD</option>
		<option value="PATCH">PROD</option>
	</select>

	<div class="variant-filled btn-group mt-4 hidden px-2 md:flex md:flex-row">
		<button on:click={() => modalStore.trigger(addCollectionModal)}>
			<FolderPlus />
		</button>
		<button on:click={add_request}>
			<FilePlus />
		</button>
		<button>
			<Globe />
		</button>
		<button>
			<Info />
		</button>
		<button on:click={() => toggle_sidebar()}>
			<Menu />
		</button>
	</div>
</div>
<div id="collections" hidden="" class="mb-2 overflow-y-auto overflow-x-hidden overscroll-none max-h-80">
	{#if $collections_store}
		<TreeView class="hidden text-xs lg:block">
			{#each Array.from($collections_store.collections) as [collection_name, collection]}
				<TreeViewItem class="my-0.5">
					<svelte:fragment slot="lead">
						<button
							on:click={() => (selected_collection = collection_name)}
							use:popup={collectionSettingsPopup}
						>
							<FolderClosed />
						</button>
					</svelte:fragment>
					<div class="flex items-center justify-between">
						<span class="flex items-center overflow-hidden whitespace-nowrap text-sm"
							>{collection_name}</span
						>
					</div>
					<svelte:fragment slot="children">
						{#each Array.from(collection.requests) as [request_name, request]}
							<TreeViewItem class="my-0.5" on:click={() => open_request_tab(request)}>
								<svelte:fragment slot="lead">
									<button
										on:click={(event) => {
											event.stopPropagation();
											selected_request = request;
										}}
										use:popup={requestSettingsPopup}
									>
										<FileCog />
									</button>
								</svelte:fragment>
								<div class="flex items-center justify-between">
									<div class="flex items-center">
										<span class="badge {method_to_colour.get(request.method)} mr-2 text-xs"
											>{method_to_abb.get(request.method)}</span
										>
										<span class="overflow-hidden whitespace-nowrap text-sm"
											>{request_name}</span
										>
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
						<button
							on:click={(event) => {
								event.stopPropagation();
								selected_request = request;
							}}
							use:popup={requestSettingsPopup}
						>
							<FileCog />
						</button>
					</svelte:fragment>
					<span class="ml-0 badge {method_to_colour.get(request.method)} mr-2 text-xs"
						>{method_to_abb.get(request.method)}</span
					>
					<span class="overflow-hidden whitespace-nowrap text-sm"
						>{request_name}</span
					>
				</TreeViewItem>
			{/each}
		</TreeView>
	{/if}
</div>
