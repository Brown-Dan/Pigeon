<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getToastStore, SlideToggle, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import ResponseView from './ResponseView.svelte';
	import { collections_store } from '$lib/CollectionStore';
	import type { Header, Request, Response } from '$lib/Models';
	import HeadersForm from '$lib/components/HeadersForm.svelte';
	import QueryParamsForm from '$lib/components/QueryParamsForm.svelte';
	import 'highlight.js/styles/srcery.css';
	import UrlMethodInput from '$lib/components/UrlMethodInput.svelte';
	import {
		get_failure_formatting_json_notification,
		get_failure_to_send_request_notification,
		get_request_sent_notification
	} from '$lib/ToastService';
	import { EditorView } from 'codemirror';
	import { onMount } from 'svelte';
	import { getCodeMirror } from '$lib/RequestBodyCodeMirror';
	import { current_tab_index, open_tabs } from '$lib/TabStore';
	import { response } from '$lib/ResponseStore';

	export let request: Request;

	function updateRequest(updatedRequest: Request) {
			open_tabs.update(value => {
				value[$current_tab_index] = updatedRequest;
				return value;
			});
	}

	const toastStore = getToastStore();

	let editor: EditorView;
	onMount(() => {
		editor = getCodeMirror(request);
	});

	let current_tab: number = 0;
	let pending_request = false;

	function update_request() {
		request.body.content = editor.state.doc.toString();
		collections_store.subscribe(value => {
			value.orphan_requests.forEach(request => invoke('add_request', { request }));
			value.collections.forEach(collection => collection.requests.forEach(request => invoke('add_request', { request })));
		});
	}

	function send_request() {
		pending_request = true;
		update_request();
		if (request.body.enabled) {
			if (!request.headers.map(h => h.name).includes('content-type', 0)) {
				let content_type_json: Header = {
					name: 'content-type',
					value: 'application/json',
					enabled: true
				};
				request.headers.push(content_type_json);
			}
		}
		invoke('send_request', { request: request })
			.then(value => {
				if (typeof value === 'string') {
					if (value.includes('error sending request for url') || value.includes('Error sending Request')) {
						toastStore.trigger(get_failure_to_send_request_notification(value));
						pending_request = false;
					} else {
						let json: any = JSON.parse(value);
						let new_response: Response = {
							status: json.status,
							size: json.size,
							body: json.content_type.includes('application/json') ? JSON.stringify(JSON.parse(json.body), null, 2) : json.body,
							headers: json.headers,
							elapsed: json.elapsed,
							content_type: json.content_type
						};
						response.update(value => value.set(request.name, new_response))
						toastStore.trigger(get_request_sent_notification());
						pending_request = false;
					}
				}
			});
	}

	function format_body() {
		try {
			const transaction = editor.state.update({
				changes: {
					from: 0,
					to: editor.state.doc.length,
					insert: JSON.stringify(JSON.parse(editor.state.doc.toString()), null, 2)
				}
			});
			editor.dispatch(transaction);
			update_request();
		} catch (e) {
			console.log(e);
			toastStore.trigger(get_failure_formatting_json_notification());
		}
	}

	$: {
		if (editor) {
			const transaction = editor.state.update({
				changes: { from: 0, to: editor.state.doc.length, insert: request.body.content }
			});
			editor.dispatch(transaction);
		}
	}
</script>
<div class="grid grid-cols-10 min-h-max m-5">
	<div class="mt-16 col-span-4">
		<UrlMethodInput bind:request on:update={e => updateRequest(e.detail)} />
		<TabGroup>
			<Tab bind:group={current_tab} name="tab1" value={0}>Body</Tab>
			<Tab bind:group={current_tab} name="tab2" value={1}>Parameters</Tab>
			<Tab bind:group={current_tab} name="tab3" value={2}>Headers</Tab>
			<Tab bind:group={current_tab} name="tab4" value={3}>Scripts</Tab>
			<svelte:fragment slot="panel">
				<div hidden={current_tab !== 0} class="mt-2">
					<SlideToggle name="slider-label" bind:checked={request.body.enabled}>Include Body</SlideToggle>
					<div id="body" class="{request.body.enabled ? '' : 'hidden'}">
						<button on:click={format_body} type="button"
										class="btn text variant-filled px-4 py-2 bg-blue-500 text-white rounded mb-2">
							Format JSON
						</button>
					</div>
				</div>
				<div hidden={current_tab !== 1}>
					<QueryParamsForm {request} />
				</div>
				<div hidden={current_tab !== 2}>
					<HeadersForm {request} />
				</div>
				<div hidden={current_tab !== 4}>
					Scripts
				</div>
			</svelte:fragment>
		</TabGroup>
		<button on:click={send_request} type="button" class="btn variant-filled mt-5 text">
			<svg
				class="{pending_request === false ? 'hidden' : ''}  w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600"
				viewBox="0 0 100 101" fill="none"
				xmlns="http://www.w3.org/2000/svg">
				<path
					d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
					fill="currentColor" />
				<path
					d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
					fill="currentFill" />
			</svg>
			<b hidden={pending_request}>Send</b>
		</button>
	</div>
	<div class="col-span-6">
		{#if $response.get(request.name) !== undefined}
			<ResponseView response={$response.get(request.name)} />
		{:else}
			<div class="card m-5 p-4 text-white text-xl text-center">
				<section class="p-4">
					<kbd class="kbd">⌘ + Enter</kbd> to send a request.
					<br>
					<kbd class="kbd">⌘ + E</kbd> to edit environment.
				</section>
			</div>
		{/if}
	</div>
</div>



