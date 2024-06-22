<script lang="ts">
	import RequestResponse from '$lib/components/RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { get_scratchpad } from '$lib/Models';
	import { current_tab_index, decrement, open_tabs } from '$lib/TabStore';

	function close_tab(tab_index: number) {
		open_tabs.update((tabs) => {
			tabs.splice(tab_index, 1);
			decrement();
			return tabs;
		});
	}
</script>

<TabGroup>
	{#each $open_tabs as request, i}
		<Tab bind:group={$current_tab_index} name="tab{i}" value={i}>
			<div class="flex items-center">
				{request.name}
				<button
					on:click={() => close_tab(i)}
					type="button"
					class="ml-1 inline-flex items-center justify-center rounded-md bg-white p-2 text-black hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
				>
					<svg
						class="h-2 w-2"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
		</Tab>
	{/each}
	<svelte:fragment slot="panel">
		<RequestResponse request={$open_tabs.at($current_tab_index) ?? get_scratchpad()} />
	</svelte:fragment>
</TabGroup>
