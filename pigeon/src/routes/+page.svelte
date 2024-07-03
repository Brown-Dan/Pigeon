<script lang="ts">
	import RequestResponse from '$lib/components/RequestResponse.svelte';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import { get_scratchpad } from '$lib/Models';
	import { current_tab_index, decrement, increment, open_tabs } from '$lib/TabStore';
	import { X } from 'lucide-svelte';
	import hotkeys from 'hotkeys-js';

	hotkeys('cmd+left', left_tab);
	hotkeys('cmd+right', right_tab);

	function left_tab() {
		if ($current_tab_index > 0) {
			decrement();
		}
	}

	function right_tab() {
		if ($current_tab_index < $open_tabs.length - 1) {
			increment();
		}
	}

	function close_tab(tab_index: number) {
		open_tabs.update((tabs) => {
			tabs.splice(tab_index, 1);
			if ($current_tab_index > 0 || $open_tabs.length === 0) {
				decrement();
			}
			return tabs;
		});
	}
</script>

<div class="h-full overflow-y-auto">
	<TabGroup>
		{#each $open_tabs as request, i}
			<Tab bind:group={$current_tab_index} name="tab{i}" value={i}>
				<div class="flex items-center">
					{request.name}
					<button
						on:click={() => close_tab(i)}
						type="button"
						class="text-xs ml-1 inline-flex items-center justify-center rounded-md bg-white p-2 text-black hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
					>
						<X class="w-2 h-2" />
					</button>
				</div>
			</Tab>
		{/each}
		<svelte:fragment slot="panel">
			<RequestResponse request={$open_tabs.at($current_tab_index) ?? get_scratchpad()} />
		</svelte:fragment>
	</TabGroup>
</div>