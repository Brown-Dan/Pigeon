<script lang="ts">
	import RequestResponse from '$lib/components/RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { get_scratchpad, type Request } from '$lib/Models';

	let tabSet: number = 0;
	let request_tabs: Request[] = [];

	function handleRequestBarClick(event: CustomEvent) {
		let idx: number = request_tabs.indexOf(event.detail, 0);
		if (idx !== -1) {
			tabSet = idx;
			return;
		}
		request_tabs.push(event.detail);
		request_tabs = request_tabs;
		idx = request_tabs.indexOf(event.detail, 0);
		tabSet = idx;
	}

	function close_tab(index: number) {
		request_tabs.splice(index, 1);
		request_tabs = request_tabs.filter(Boolean);
		tabSet -= 1;
	}

	function handle_deleted_collection(event: CustomEvent) {
		let idx: number = request_tabs.map(r => r.collection_name).indexOf(event.detail);
		close_tab(idx);
	}

	function handle_deleted_request(event: CustomEvent) {
		let idx: number = request_tabs.map(r => r.name).indexOf(event.detail);
		close_tab(idx);
	}

	function handle_rename(event: CustomEvent) {
		request_tabs = request_tabs;
	}

</script>
<svelte:window on:requestBarClick={handleRequestBarClick} on:deletedCollection={handle_deleted_collection} on:deletedRequest={handle_deleted_request} on:renameRequest={handle_rename}/>
<TabGroup class="mt-5"}>
	{#each request_tabs as request, i}
		<Tab bind:group={tabSet} name="tab{i}" value={i}>{request.name}
			<button on:click={() => close_tab(i)} type="button"
							class="bg-white rounded-md p-2 inline-flex items-center justify-center text-black hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
				<svg class="h-2 w-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor"
						 aria-hidden="true">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
				</svg>
			</button>
		</Tab>
	{/each}

	<svelte:fragment slot="panel">
		{#if request_tabs.at(tabSet) !== undefined}
			<RequestResponse request={request_tabs.at(tabSet)} />
			{:else}
			<RequestResponse request={get_scratchpad()} />
		{/if}
	</svelte:fragment>
</TabGroup>

