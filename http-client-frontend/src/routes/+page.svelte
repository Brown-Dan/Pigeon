<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { clipboard, CodeBlock, storeHighlightJs, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import hljs from 'highlight.js/lib/core';
	import json from 'highlight.js/lib/languages/json';

	hljs.registerLanguage('json', json);
	storeHighlightJs.set(hljs);

	let response: JSON;
	let headers: Map<any, any>;
	let tabSet: number = 0;
	const exampleData: string = 'https://httpbin.org/get';

	function send_request() {
		const url: string = (<HTMLInputElement>document.getElementById('url')).value;
		console.log(url);
		invoke('send_request', { url }).then(value => {
				if (typeof value === 'string') {
					response = JSON.parse(value);
				}
				headers = new Map(Object.entries(response.headers));
			}
		);
	}
</script>
<h1 class="h1 text-center m-5">Soxy</h1>
<div class="grid grid-cols-2 min-h-full">
	<div>
		<select name="request_method" id="request_method" class="text-black">
			<option value="POST">GET</option>
			<option value="POST">POST</option>
		</select>
		<label for="url">Enter url: </label>
		<input type="text" id="url" class="text-black">
		<button on:click={send_request}>Send Request</button>
	</div>
	<div class="m-5">
		<div>
			{#if response}
				<p>Response Status: {response.status}</p>
			{:else}
				<p>Response Status: </p>
			{/if}
		</div>
		<TabGroup>
			<Tab bind:group={tabSet} name="tab1" value={0}>
				<svelte:fragment slot="lead">Response Body</svelte:fragment>
			</Tab>
			<Tab bind:group={tabSet} name="tab2" value={1}>Headers</Tab>
			<Tab bind:group={tabSet} name="tab3" value={2}>Cookies</Tab>
			<!-- Tab Panels --->
			<svelte:fragment slot="panel">
				{#if tabSet === 0}
					<div class="border-white min-w-screen min-h-screen text-black rounded-2xl text-wrap p-5">
						{#if response}
							<CodeBlock lineNumbers=True language="json" code={response.body}></CodeBlock>
						{/if}
					</div>
				{:else if tabSet === 1}
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
				{:else if tabSet === 2}
					Content
				{/if}
			</svelte:fragment>
		</TabGroup>
	</div>
</div>



