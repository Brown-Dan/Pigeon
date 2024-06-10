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
	import type { Request, Requests } from '$lib/Models';
	import AddCollectionModal from '$lib/components/modals/AddCollectionModal.svelte';
	import { requests } from '$lib/RequestsStore';
	import AddRequestModal from '$lib/components/modals/AddRequestModal.svelte';
	import { limit_request_chars, method_to_abb, method_to_colour } from '$lib/MethodUtils';


	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	initializeStores();

	export let requests_result: Requests;

	const modalStore = getModalStore();
	let selected_collection: String;


	const modalRegistry: Record<string, ModalComponent> = {
		addCollectionModal: { ref: AddCollectionModal },
		addRequestModal: { ref: AddRequestModal }
	};

	const collectionSettingsPopup: PopupSettings = {
		event: 'click',
		target: 'collectionSettingsPopup',
		placement: 'right',
		closeQuery: 'button'
	};

	const addCollectionModal: ModalSettings = {
		type: 'component',
		component: 'addCollectionModal'
	};

	async function add_request() {
		const addRequestModal: ModalSettings = {
			type: 'component',
			component: 'addRequestModal',
			meta: { name: selected_collection }
		};
		modalStore.trigger(addRequestModal);
	}

	const confirmDeleteModal: ModalSettings = {
		type: 'confirm',
		title: 'Please Confirm',
		body: 'Are you sure you wish to proceed with deleting collection?',
		response: (r: boolean) => {
			if (r === true) {
				invoke('delete_collection', { collectionName: selected_collection });
				requests.update((value) => {
					value.collections = value.collections.filter(collection => collection.name !== selected_collection);
					return value;
				});
				window.dispatchEvent(new CustomEvent('deletedCollection', { detail: selected_collection}))
			}
		}
	};

	function open_request_tab(request: Request) {
		window.dispatchEvent(new CustomEvent('requestBarClick', { detail: request }));
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

<div class="overflow-hidden">
	<select class="select mr-5 mt-5 ml-2 p-2 text-xs hidden lg:inline-block w-24" id="method">
		<option value="GET">LOCAL</option>
		<option value="PUT">PREPROD</option>
		<option value="PATCH">PROD</option>
	</select>

	<div class="ml-2 btn-group variant-filled mt-4 hidden lg:inline-block">
		<button on:click={() => modalStore.trigger(addCollectionModal)} class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round" d="M12 10.5v6m3-3H9m4.06-7.19-2.12-2.12a1.5 1.5 0 0 0-1.061-.44H4.5A2.25 2.25 0 0 0 2.25 6v12a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9a2.25 2.25 0 0 0-2.25-2.25h-5.379a1.5 1.5 0 0 1-1.06-.44Z" />
			</svg>
		</button>
		<button on:click={add_request} class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="#000000" class="w-5 h-5" viewBox="0 0 482.14 482.14" xml:space="preserve">
            <g>
                <path d="M302.599,0H108.966C80.66,0,57.652,23.025,57.652,51.315v379.509c0,28.289,23.008,51.315,51.314,51.315h264.205c28.275,0,51.316-23.026,51.316-51.315V121.449L302.599,0z M373.171,450.698H108.966c-10.969,0-19.89-8.905-19.89-19.874V51.315c0-10.953,8.921-19.858,19.89-19.858l181.875-0.189v67.218c0,19.653,15.949,35.603,35.588,35.603l65.877-0.189l0.725,296.925C393.03,441.793,384.142,450.698,373.171,450.698z" />
							<path d="M241.054,150.96c-49.756,0-90.102,40.347-90.102,90.109c0,49.764,40.346,90.11,90.102,90.11c49.771,0,90.117-40.347,90.117-90.11C331.171,191.307,290.825,150.96,241.054,150.96z M273.915,253.087h-20.838v20.835c0,6.636-5.373,12.017-12.023,12.017c-6.619,0-12.01-5.382-12.01-12.017v-20.835H208.21c-6.637,0-12.012-5.383-12.012-12.018c0-6.634,5.375-12.017,12.012-12.017h20.834v-20.835c0-6.636,5.391-12.018,12.01-12.018c6.65,0,12.023,5.382,12.023,12.018v20.835h20.838c6.635,0,12.008,5.383,12.008,12.017C285.923,247.704,280.55,253.087,273.915,253.087z" />
            </g>
        </svg>
		</button>
		<button class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418" />
			</svg>
		</button>
		<button class="p-1">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5">
				<path stroke-linecap="round" stroke-linejoin="round" d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z" />
			</svg>
		</button>
	</div>
</div>
<div class="overflow-y-auto overscroll-none mb-5 overflow-x-hidden">
	{#if requests_result}
		<TreeView class="hidden lg:block text-xs">
			{#each requests_result.collections as collection}
				<TreeViewItem class="my-0.5">
					<div class="flex items-center justify-between">
						<span class="flex items-center whitespace-nowrap overflow-hidden text-sm">{collection.name}</span>
						<button
							class="ml-1 p-1 flex items-center justify-center"
							on:click={(event) => {
                        event.stopPropagation();
                        selected_collection = collection.name;
                    }}
							use:popup={collectionSettingsPopup}
						>
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="w-5 h-5">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
							</svg>
						</button>
					</div>
					<svelte:fragment slot="children">
						{#each collection.requests as request}
							<TreeViewItem class="my-0.5" on:click={() => open_request_tab(request)}>
								<div class="flex items-center justify-between">
									<button
										class="p-1 flex items-center justify-center"
										on:click={(event) => {
                                    event.stopPropagation();
                                    selected_collection = request.collection_name;
                                }}
										use:popup={collectionSettingsPopup}
									>
										<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
												 stroke="currentColor" class="w-5 h-5">
											<path stroke-linecap="round" stroke-linejoin="round"
														d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
										</svg>
									</button>
									<div class="flex items-center">
										<span class="ml-1 badge variant-filled-success mr-2 text-xs">{request.method}</span>
										<span class="overflow-hidden whitespace-nowrap text-sm">{limit_request_chars(request.name)}</span>
									</div>
								</div>
							</TreeViewItem>
						{/each}
					</svelte:fragment>
				</TreeViewItem>
			{/each}
			{#each requests_result.orphaned_requests as request}
				<TreeViewItem class="my-0.5" on:click={() => open_request_tab(request)}>
					<div class="flex items-center justify-between">
						<button
							class="p-1 flex items-center justify-center"
							on:click={(event) => {
                        event.stopPropagation();
                        selected_collection = request.collection_name;
                    }}
							use:popup={collectionSettingsPopup}
						>
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="w-5 h-5">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
							</svg>
						</button>
						<div class="flex items-center">
							<span class="ml-1 badge {method_to_colour.get(request.method)} mr-2 text-xs">{method_to_abb.get(request.method)}</span>
							<span class="overflow-hidden whitespace-nowrap text-sm">{limit_request_chars(request.name)}</span>
						</div>
					</div>
				</TreeViewItem>
			{/each}
		</TreeView>
	{/if}
</div>
