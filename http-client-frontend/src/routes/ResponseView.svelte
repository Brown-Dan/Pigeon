<script lang="ts">
	import { friendlyHttpStatus } from '$lib/StatusUtils.js';
	import { clipboard, CodeBlock, storeHighlightJs, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import hljs from 'highlight.js/lib/core';
	import json from 'highlight.js/lib/languages/json';
	import 'highlight.js/styles/srcery.css';

	hljs.registerLanguage('json', json);
	storeHighlightJs.set(hljs);

	let responseTabSet: number = 0;
	const exampleData: string = 'https://httpbin.org/get';

	export let response: JSON;
	export let time: number;
	export let headers: Map<String, String>;

</script>

<div class="m-5">
	<div>
		{#if response}
			<span class="badge variant-filled">{response.status} {friendlyHttpStatus[response.status]}</span>
				<span class="badge variant-filled">{time.toFixed()}ms</span>
				<span class="badge variant-filled">{(new TextEncoder().encode(response.body)).length} B</span>
			{:else}
				<span class="badge variant-filled invisible">Badge</span>
			{/if}
			</div>
			<TabGroup>
				<Tab bind:group={responseTabSet} name="tab1" value={0}>
					<svelte:fragment slot="lead">Response Body</svelte:fragment>
				</Tab>
				<Tab bind:group={responseTabSet} name="tab2" value={1}>Headers</Tab>
				<Tab bind:group={responseTabSet} name="tab3" value={2}>Cookies</Tab>
				<svelte:fragment slot="panel">
					{#if responseTabSet === 0}
						<div class="border-white min-w-screen min-h-screen text-black rounded-2xl text-wrap p-5 overflow-scroll">
							{#if response}
								<CodeBlock lineNumbers language="json" code={response.body}></CodeBlock>
							{:else}
								<div class="card p-4 text-white text-xl">
									<kbd class="kbd">⌘ + Enter</kbd> to send a request.
									<br>
									<kbd class="kbd">⌘ + E</kbd> to edit environment.
								</div>
							{/if}
						</div>
					{:else if responseTabSet === 1}
						<div class="table-container">
							<table class="table table-hover col-span-2">
								<thead>
								<tr>
									<th>Name</th>
									<th>Value</th>
								</tr>
								</thead>
								<tbody>
								{#if headers}
									{#each [...headers] as [key, value]}
										<tr>
											<td>{key}</td>
											<td>{value}</td>
										</tr>
									{/each}
								{/if}
								</tbody>
								<tfoot>
								<tr class="col-span-2">
									<td>
										<button use:clipboard={exampleData}>Copy</button>
									</td>
								</tr>
								</tfoot>
							</table>
						</div>
					{:else if responseTabSet === 2}
						Empty :(
					{/if}
				</svelte:fragment>
			</TabGroup>
			</div>