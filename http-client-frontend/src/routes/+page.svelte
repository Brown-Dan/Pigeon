<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { clipboard, CodeBlock, storeHighlightJs, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import hljs from 'highlight.js/lib/core';
	import json from 'highlight.js/lib/languages/json';
	import { friendlyHttpStatus } from '$lib/StatusUtils';


	hljs.registerLanguage('json', json);
	storeHighlightJs.set(hljs);

	let response: JSON;
	let headers: Map<any, any>;
	let time: number;
	let responseTabSet: number = 0;
	let requestTabSet: number = 0;
	let numOfParams: number = 1;
	const exampleData: string = 'https://httpbin.org/get';

	function send_request() {
		const url: string = (<HTMLInputElement>document.getElementById('url')).value;
		let start_time = window.performance.now();
		invoke('send_request', { url }).then(value => {
				let end_time = window.performance.now();
				time = end_time - start_time;
				if (typeof value === 'string') {
					response = JSON.parse(value);
				}
				headers = new Map(Object.entries(response.headers));
			}
		);
	}

	function increaseParamCount() {
		numOfParams += 1;
	}

	function deleteParams() {
		numOfParams = 0;
	}

	function shouldDisableParamInput(num: number): boolean {
		let element = <HTMLInputElement>document.getElementById('param_checkbox' + num);
		console.log(num)
		if (element) {
			return !element.checked;
		}
		return false;
	}

</script>
<div class="grid grid-cols-2 min-h-full m-5">
	<div class="mt-16">
		<div class="input-group input-group-divider grid-cols-[1fr_auto] mb-5">
			<input type="text" placeholder="www.example.com" id="url" />
			<select id="method">
				<option value="1">GET</option>
				<option value="2">PUT</option>
				<option value="3">PATCH</option>
				<option value="4">DELETE</option>
				<option value="5">POST</option>
			</select>
		</div>
		<TabGroup>
			<Tab bind:group={requestTabSet} name="tab1" value={0}>
				<svelte:fragment slot="lead">Body</svelte:fragment>
			</Tab>
			<Tab bind:group={requestTabSet} name="tab2" value={1}>Parameters</Tab>
			<Tab bind:group={requestTabSet} name="tab3" value={2}>Headers</Tab>
			<!-- Tab Panels --->
			<svelte:fragment slot="panel">
				{#if requestTabSet === 0}
					<label class="label">
						<textarea class="textarea" rows="4"
											placeholder="Lorem ipsum dolor sit amet consectetur adipisicing elit." />
					</label>
				{:else if requestTabSet === 1}
					<div class="btn-group variant-filled mb-5">
						<button type="button" class=" btn-sm" on:click={increaseParamCount}>Add</button>
						<button type="button" class=" btn-sm" on:click={deleteParams}>Delete All</button>
					</div>
					{#each { length: numOfParams } as _, i}
						<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
							<input type="text" placeholder="key" disabled={shouldDisableParamInput(i)} />
							<input type="text" placeholder="value" disabled={shouldDisableParamInput(i)} />
							<div id="{'param_checkbox' + i}" class="input-group-shim"><input class="checkbox" type="checkbox"
																																							 checked /></div>
						</div>
					{/each}
				{:else if requestTabSet === 2}
					(tab panel 3 contents)
				{/if}
			</svelte:fragment>
		</TabGroup>
		<button on:click={send_request} type="button" class="btn btn-xl variant-filled mt-5 text">Send</button>
	</div>
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
							<CodeBlock lineNumbers=True language="json" code={response.body}></CodeBlock>
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
</div>



