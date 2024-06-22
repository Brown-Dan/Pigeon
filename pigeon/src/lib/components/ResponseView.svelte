<script lang="ts">
	import { get_status_colour, getMessageForStatus } from '$lib/StatusUtils';
	import { clipboard, CodeBlock, storeHighlightJs, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import hljs from 'highlight.js/lib/core';
	import json from 'highlight.js/lib/languages/json';
	import 'highlight.js/styles/srcery.css';
	import { duration_to_string, type Response } from '$lib/Models';

	hljs.registerLanguage('json', json);
	storeHighlightJs.set(hljs);

	export let response: Response;

	let response_format: string = "json";
	let current_tab_index: number = 0;
</script>
<div>
	<div>
		<select bind:value={response_format} class="select mr-1 ml-2 p-2 text-xs hidden lg:inline-block w-24" id="format">
			<option value="json">JSON</option>
			<option value="html">HTML</option>
			<option value="text">TEXT</option>
		</select>
		<span
			class="badge {get_status_colour(parseInt(response.status))}">{response.status} {getMessageForStatus.get(response.status.toString())}</span>
		<span class="badge variant-filled">{duration_to_string(response.elapsed)}</span>
		<span class="badge variant-filled">{(new TextEncoder().encode(JSON.parse(JSON.stringify(response.body)))).length}
			B</span>
	</div>
	<TabGroup>
		<Tab bind:group={current_tab_index} name="tab1" value={0}>
			<svelte:fragment slot="lead">Response Body</svelte:fragment>
		</Tab>
		<Tab bind:group={current_tab_index} name="tab2" value={1}>Headers</Tab>
		<Tab bind:group={current_tab_index} name="tab3" value={2}>Cookies</Tab>
		<svelte:fragment slot="panel">
			{#if current_tab_index === 0}
				<div class="border-white min-w-screen min-h-screen text-black rounded-2xl text-wrap p-5 overflow-scroll">
					<CodeBlock lineNumbers language={response_format} code={response.body}></CodeBlock>
				</div>
			{:else if current_tab_index === 1}
				<div class="table-container">
					<table class="table table-hover col-span-2">
						<thead>
						<tr>
							<th>Name</th>
							<th>Value</th>
						</tr>
						</thead>
						<tbody>
						{#each response.headers as header}
							<tr>
								<td>{header.name}</td>
								<td>{header.value}</td>
							</tr>
						{/each}
						</tbody>
						<tfoot>
						<tr class="col-span-2">
							<td>
								<button use:clipboard={'https://httpbin.org/get'}>Copy</button>
							</td>
						</tr>
						</tfoot>
					</table>
				</div>
			{:else if current_tab_index === 2}
				Empty :(
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
