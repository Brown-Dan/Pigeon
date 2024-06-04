<script lang="ts">
	import RequestResponse from './RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';

	let tabSet: number = 0;

	let requests: string[] = ['Get Customer', 'Find Customer', 'Fetch Customer'];

	function handleNewTabEvent(event: Event) {
		requests.push(event.detail)
		requests = requests
	}

	function close_tab(index: number) {
		requests.splice(index, 1);
		requests = requests.filter(Boolean);
		console.log(requests);
	}

</script>
<svelte:window on:requestBarClick={(e) => handleNewTabEvent(e)} />
<TabGroup on:message={(event) => handleNewTabEvent(event)}>
	{#each requests as request, i}
		<Tab bind:group={tabSet} name="tab1" value={i}>{request}
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
		<RequestResponse />
	</svelte:fragment>
</TabGroup>

