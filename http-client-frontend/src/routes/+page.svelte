<script lang="ts">
	import RequestResponse from './RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import type { Request, Requests } from '$lib/Request';
	import { invoke } from '@tauri-apps/api/tauri';

	let tabSet: number = 0;
	let request_tabs: Request[] = [];

	function handleNewTabEvent(event: Event) {
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

	let requests: Promise<Requests> = invoke('get_collections', {}).then((value) => <Requests>value);


</script>
<svelte:window on:requestBarClick={(e) => handleNewTabEvent(e)} />
<TabGroup class="mt-5" on:message={(event) => handleNewTabEvent(event)}>
	{#each request_tabs as request, i}
		<Tab bind:group={tabSet} name="tab{i}" value={i}>{request.name}
			<button on:click={() => close_tab(i)} type="button"
							class="bg-white rounded-md p-2 inline-flex items-center justify-center text-black hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
				<svg class="h-2 w-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor"
						 aria-hidden="true">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
				</svg>
			</button>
		</Tab>
	{/each}

	<svelte:fragment slot="panel">
			<RequestResponse request={request_tabs.at(tabSet)} />
	</svelte:fragment>
</TabGroup>

