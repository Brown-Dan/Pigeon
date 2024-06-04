<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getToastStore, storeHighlightJs, Tab, TabGroup, type ToastSettings } from '@skeletonlabs/skeleton';
	import ResponseView from './ResponseView.svelte';
	import type { Request } from '$lib/Request';

	export let request: Request | undefined;

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

	let response: JSON;
	let headers: Map<any, any>;
	let time: number;
	let requestTabSet: number = 0;
	let numOfParams: number = 1;
	let numOfHeaders: number = 1;

	function send_request() {
		const btn_spinner = document.getElementById('btn_spinner');
		btn_spinner.removeAttribute('hidden');
		const btn_content = document.getElementById('btn_content');
		btn_content?.setAttribute('hidden', 'hidden');
		const url: string = (<HTMLInputElement>document.getElementById('url')).value;
		let start_time = window.performance.now();
		invoke('send_request', { url: url + gatherParams(), headers: gatherHeaders() }).then(value => {
				try {
					let end_time = window.performance.now();
					time = end_time - start_time;
					btn_content?.removeAttribute('hidden');
					btn_spinner?.setAttribute('hidden', 'hidden');
					if (typeof value === 'string') {
						response = JSON.parse(value);
					}
					headers = new Map(Object.entries(response.headers));
					toastStore.trigger(request_success);
				} catch (e) {
					toastStore.trigger(request_failure);
				}
			}
		);
	}

	function gatherHeaders(): Map<String, String> {
		let headers: Map<String, String> = new Map<String, String>();
		// let host_value = <HTMLInputElement>document.getElementById('host').value;
		let accept_value: string = (<HTMLInputElement>document.getElementById('accept')).value;
		headers.set('Accept', accept_value);

		if (numOfHeaders >= 1) {
			for (let i = 0; i < numOfHeaders; i++) {
				let header_name: string = (<HTMLInputElement>document.getElementById('header_name_' + i)).value;
				if (header_name.length === 0) {
					continue;
				}
				let header_value: string = (<HTMLInputElement>document.getElementById('header_value_' + i)).value;
				let header_checkbox = <HTMLInputElement>document.getElementById('header_checkbox_' + i);
				if (header_checkbox.checked) {
					headers.set(header_name, header_value);
				}
			}
		}
		return headers;
	}

	function gatherParams(): string {
		let queryParams = '';
		if (numOfParams >= 1) {
			queryParams = queryParams + '?'
			for (let i = 0; i < numOfParams; i++) {
				let param_name: string = (<HTMLInputElement>document.getElementById('param_name_' + i)).value;
				if (param_name.length === 0) {
					continue;
				}
				let param_value: string = (<HTMLInputElement>document.getElementById('param_value_' + i)).value;
				let param_checkbox = <HTMLInputElement>document.getElementById('param_checkbox_' + i);
				if (param_checkbox.checked) {
					queryParams = queryParams + param_name + '=' + param_value + '&';
				}
			}
			queryParams = queryParams.substring(0, queryParams.length - 1);
		}
		return queryParams;
	}

	function increaseParamCount() {
		numOfParams += 1;
	}

	function deleteParams() {
		numOfParams = 0;
	}

	function increaseHeaderCount() {
		numOfHeaders += 1;
	}

	function deleteHeader() {
		numOfHeaders = 0;
	}

	function disable(num: number, prefix: string) {
		let param_name = <HTMLInputElement>document.getElementById(prefix + '_name_' + num);
		let param_value = <HTMLInputElement>document.getElementById(prefix + '_value_' + num);
		if (param_name.getAttribute('disabled') === 'disabled') {
			param_name.removeAttribute('disabled');
		} else {
			param_name.setAttribute('disabled', 'disabled');
		}
		if (param_value.getAttribute('disabled') === 'disabled') {
			param_value.removeAttribute('disabled');
		} else {
			param_value.setAttribute('disabled', 'disabled');
		}
	}

</script>
<div class="grid grid-cols-2 min-h-full m-5 overflow-auto">
	<div class="mt-16">
		<div class="input-group input-group-divider grid-cols-[1fr_auto] mb-5">
			<input type="text" placeholder="https://example.com/" id="url" value={request !== undefined ? request.url : ""} />
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
						<button type="button" class=" btn-sm" on:click={increaseParamCount}>Add</button>
						<button type="button" class=" btn-sm" on:click={deleteParams}>Delete All</button>
					</div>
					{#each { length: numOfParams } as _, i}
						<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
							<input type="text" placeholder="name" id={"param_name_" + i} disabled="" />
							<input type="text" placeholder="value" id={"param_value_" + i} disabled="" />
							<div class="input-group-shim"><input on:change={() => disable(i, 'param')} id="{'param_checkbox_' + i}"
																									 class="checkbox" type="checkbox"
																									 checked /></div>
						</div>
					{/each}
				</div>
				<div hidden={requestTabSet !== 2} id="headers">
					<div class="btn-group variant-filled mb-5">
						<button type="button" class=" btn-sm" on:click={increaseHeaderCount}>Add</button>
						<button type="button" class=" btn-sm" on:click={deleteHeader}>Delete All</button>
					</div>
					<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
						<input type="text" placeholder="name" disabled value="Accept" />
						<input type="text" placeholder="value" id="accept" value="*/*" />
					</div>
					<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
						<input type="text" placeholder="name" disabled value="Host" />
						<input type="text" placeholder="value" id="host" disabled value="<calculated at runtime>" />
					</div>
					{#each { length: numOfHeaders } as _, i}
						<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] m-3">
							<input type="text" placeholder="name" id={"header_name_" + i} disabled="" />
							<input type="text" placeholder="value" id={"header_value_" + i} disabled="" />
							<div class="input-group-shim"><input on:change={() => disable(i, 'header')} id="{'header_checkbox_' + i}"
																									 class="checkbox" type="checkbox"
																									 checked /></div>
						</div>
					{/each}
				</div>
			</svelte:fragment>
		</TabGroup>
		<button id="send_request_btn" on:click={send_request} type="button" class="btn btn-xl variant-filled mt-5 text">
			<svg id="btn_spinner" hidden aria-hidden="true"
					 class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none"
					 xmlns="http://www.w3.org/2000/svg">
				<path
					d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
					fill="currentColor" />
				<path
					d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
					fill="currentFill" />
			</svg>
			<b id="btn_content" hidden="">Send</b>
		</button>
	</div>
	<ResponseView {response} {time} {headers} />
</div>



