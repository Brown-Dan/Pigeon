<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getToastStore, Tab, TabGroup, type ToastSettings } from '@skeletonlabs/skeleton';
	import ResponseView from './ResponseView.svelte';
	import { requests } from '$lib/RequestsStore';
	import type { Header, Request, Response } from '$lib/Models';

	export let request: Request;

	const toastStore = getToastStore();
	const request_success: ToastSettings = {
		message: '📤 Sent request',
		timeout: 3000,
		background: 'variant-filled-success'
	};
	const request_failure: ToastSettings = {
		message: '😭 Failed to send request',
		timeout: 3000,
		background: 'variant-filled-error'
	};

	let response: Response;

	let requestTabSet: number = 0;
	let numOfParams: number = 1;
	let numOfHeaders: number = 1;

	function update_request() {
		requests.subscribe(value => {
			value.orphaned_requests.forEach(r => invoke('add_request', { request: r }));
		});
	}

	function send_request() {
		const button_spinner = document.getElementById('button_spinner');
		button_spinner.removeAttribute('hidden');
		const button_content = document.getElementById('button_content');
		button_content?.setAttribute('hidden', 'hidden');

		update_request();
		invoke('send_request', { request: request })
			.then(value => {
				if (typeof value === 'string') {
					let json: any = JSON.parse(value);
					response = {
						status: json.status,
						size: json.size,
						body: json.body,
						headers: json.headers,
						elapsed: json.elapsed
					};
				}
				button_content?.removeAttribute('hidden');
				button_spinner?.setAttribute('hidden', 'hidden');
				toastStore.trigger(request_success);
			});
	}

	function add_query_param() {
		request.query_params.push({name: "", value: "", enabled: true})
		request.query_params = request.query_params
	}

	function delete_query_params() {
		request.query_params = []
		request.query_params = request.query_params
	}

	function add_header() {
		request.headers.push({name: "", value: "", enabled: true})
		request.headers = request.headers
	}

	function delete_headers() {
		request.headers = []
		request.headers = request.headers
	}
</script>
<div class="grid grid-cols-2 min-h-full m-5 overflow-auto">
	<div class="mt-16">
		<div class="input-group input-group-divider grid-cols-[1fr_auto] mb-5">
			<input bind:value={request.url} type="text" placeholder="https://example.com/" id="url" />
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
				Body
			</Tab>
			<Tab bind:group={requestTabSet} name="tab2" value={1}>Parameters</Tab>
			<Tab bind:group={requestTabSet} name="tab3" value={2}>Headers</Tab>
			<svelte:fragment slot="panel">
				<div hidden={requestTabSet !== 0} id="body">
					<label class="label">
						<textarea class="textarea" rows="4"
											placeholder="Lorem ipsum dolor sit amet consectetur adipisicing elit." />
					</label>
				</div>
				<div hidden={requestTabSet !== 1} id="queryParams">
					<div class="btn-group variant-filled mb-5">
						<button type="button" class=" btn-sm" on:click={add_query_param}>Add</button>
						<button type="button" class=" btn-sm" on:click={delete_query_params}>Delete All</button>
					</div>
					{#each request.query_params as query_param}
						<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
							<input bind:value={query_param.name} type="text" placeholder="name" disabled={!query_param.enabled} />
							<input bind:value={query_param.value} type="text" placeholder="value" disabled={!query_param.enabled} />
							<div class="input-group-shim"><input bind:checked={query_param.enabled} class="checkbox" type="checkbox"/></div>
						</div>
					{/each}
				</div>
				<div hidden={requestTabSet !== 2} id="headers">
					<div class="btn-group variant-filled mb-5">
						<button type="button" class=" btn-sm" on:click={add_header}>Add</button>
						<button type="button" class=" btn-sm" on:click={delete_headers}>Delete All</button>
					</div>
					<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
						<input type="text" placeholder="name" disabled value="Accept" />
						<input type="text" placeholder="value" id="accept" value="*/*" />
					</div>
					<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
						<input type="text" placeholder="name" disabled value="Host" />
						<input type="text" placeholder="value" id="host" disabled value="<calculated at runtime>" />
					</div>
					{#each request.headers.filter(h => h.name !== "Accept" && h.name !== "Host") as header}
						<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
							<input bind:value={header.name} type="text" placeholder="name" disabled={!header.enabled} />
							<input bind:value={header.value} type="text" placeholder="value" disabled={!header.enabled} />
							<div class="input-group-shim"><input bind:checked={header.enabled} class="checkbox" type="checkbox"/></div>
						</div>
					{/each}
				</div>
			</svelte:fragment>
		</TabGroup>
		<button id="send_request_btn" on:click={send_request} type="button" class="btn btn-xl variant-filled mt-5 text">
			<svg id="button_spinner" hidden aria-hidden="true"
					 class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none"
					 xmlns="http://www.w3.org/2000/svg">
				<path
					d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
					fill="currentColor" />
				<path
					d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
					fill="currentFill" />
			</svg>
			<b id="button_content" hidden="">Send</b>
		</button>
	</div>
	{#if response !== undefined}
		<ResponseView {response} />
	{:else}
		<div class="card m-5 p-4 text-white text-xl text-center max-h-40">
			<section class="p-4">
				<kbd class="kbd">⌘ + Enter</kbd> to send a request.
				<br>
				<kbd class="kbd">⌘ + E</kbd> to edit environment.
			</section>
		</div>
	{/if}
</div>



