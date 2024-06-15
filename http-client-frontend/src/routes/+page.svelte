<script lang="ts">
	import RequestResponse from '$lib/components/RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { get_scratchpad, type Request } from '$lib/Models';
	import { open_tabs_store } from '$lib/OpenTabStore';

	let tab_number: number = 0;
	let open_tabs: Request[];

	open_tabs_store.subscribe(value => open_tabs = value);

	function close_tab(index: number) {
		open_tabs_store.update((value) => {
			value.splice(index, 1);
			tab_number -= 1;
			return value;
		})
	}
</script>
<TabGroup }>
	{#each open_tabs as request, i}
		<Tab bind:group={tab_number} name="tab{i}" value={i}>{request.name}
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
		{#if open_tabs.at(tab_number) !== undefined}
			<RequestResponse request={open_tabs.at(tab_number)} />
		{:else}
			<RequestResponse request={get_scratchpad()} />
		{/if}
	</svelte:fragment>
</TabGroup>

