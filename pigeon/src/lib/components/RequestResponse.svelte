<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { getToastStore, SlideToggle, Tab, TabGroup } from '@skeletonlabs/skeleton';
	import ResponseView from './ResponseView.svelte';
	import { collections_store } from '$lib/CollectionStore';
	import type { Header, QueryParam, Request, Response } from '$lib/Models';
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
	import hotkeys from 'hotkeys-js';
	import { Send } from 'lucide-svelte';

	hotkeys('cmd+enter', send_request);
	hotkeys('cmd+l', format_body);
	hotkeys('cmd+b', toggle_body);

	export let request: Request;

	function updateRequest(updatedRequest: Request) {
		open_tabs.update((value) => {
			value[$current_tab_index] = updatedRequest;
			return value;
		});
	}

	const toastStore = getToastStore();

	// TODO: Review and update default values as necessary
	// Initialize with default headers, can be empty or pre-filled
	const defaultHeaders: Header[] = [];

	// Default response structure, update types and values as needed
	const defaultResponse: Response = {
		status: '404', // Default status code
		size: '0', // Default response size
		body: '', // Default response body
		headers: defaultHeaders, // Default headers
		elapsed: { secs: 0, nanos: 0 }, // Default elapsed time
		content_type: '' // Default content type
	};

	let editor: EditorView;
	onMount(() => {
		editor = getCodeMirror(request);
	});

	let current_tab: number = 0;
	let pending_request = false;

	function update_request() {
		request.body.content = editor.state.doc.toString();
		collections_store.subscribe((value) => {
			value.orphan_requests.forEach((request) => invoke('add_request', { request }));
			value.collections.forEach((collection) =>
				collection.requests.forEach((request) => invoke('add_request', { request }))
			);
		});
	}

	function send_request() {
		pending_request = true;
		update_request();
		if (request.body.enabled) {
			if (!request.headers.map((h) => h.name).includes('content-type', 0)) {
				let content_type_json: Header = {
					name: 'content-type',
					value: 'application/json',
					enabled: true
				};
				request.headers.push(content_type_json);
			}
		}
		invoke('send_request', { request: request }).then((value) => {
			if (typeof value === 'string') {
				if (
					value.includes('error sending request for url') ||
					value.includes('Error sending Request')
				) {
					toastStore.trigger(get_failure_to_send_request_notification(value));
					pending_request = false;
				} else {
					let json: any = JSON.parse(value);
					let new_response: Response = {
						status: json.status,
						size: json.size,
						body: json.content_type.includes('application/json')
							? JSON.stringify(JSON.parse(json.body), null, 2)
							: json.body,
						headers: json.headers,
						elapsed: json.elapsed,
						content_type: json.content_type
					};
					response.update((value) => value.set(request.name, new_response));
					toastStore.trigger(get_request_sent_notification());
					pending_request = false;
				}
			}
		});
	}

	function toggle_body() {
		request.body.enabled = !request.body.enabled;
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

	function get_url_preview(baseUrl: string, queryParams: QueryParam[]): string {
		try {
			let url = new URL(baseUrl);
			queryParams
				.filter((param) => param.enabled)
				.forEach((param) => {
					url.searchParams.set(param.name, param.value);
				});
			return url.toString();
		} catch (e) {
			// TODO - url cannot be parsed at this point - crashes application
			return '';
		}
	}

	let url_preview = request.url;
	$: {
		url_preview = get_url_preview(request.url, request.query_params);
	}
</script>

<div class="m-5 grid min-h-max grid-cols-10">
	<div class="col-span-4 mr-2">
		<UrlMethodInput bind:request on:update={(e) => updateRequest(e.detail)} />
		<div class="card m-0 mb-2 p-0 text-left outline-black">
			<header class="card-header text-center"><b>URL Preview</b></header>
			<section class="p-4">{url_preview}</section>
		</div>
		<TabGroup>
			<Tab bind:group={current_tab} name="tab1" value={0}>Body</Tab>
			<Tab bind:group={current_tab} name="tab2" value={1}>Parameters</Tab>
			<Tab bind:group={current_tab} name="tab3" value={2}>Headers</Tab>
			<Tab bind:group={current_tab} name="tab4" value={3}>Scripts</Tab>
			<svelte:fragment slot="panel">
				<div hidden={current_tab !== 0} class="mt-2">
					<SlideToggle name="slider-label" bind:checked={request.body.enabled}
						>Include Body</SlideToggle
					>
					<div id="body" class={request.body.enabled ? '' : 'hidden'}>
						<button
							on:click={format_body}
							type="button"
							class="text variant-filled btn mb-2 rounded bg-blue-500 px-4 py-2 text-white"
						>
							Format JSON
						</button>
					</div>
				</div>
				<div hidden={current_tab !== 1}>
					<QueryParamsForm bind:request />
				</div>
				<div hidden={current_tab !== 2}>
					<HeadersForm bind:request />
				</div>
				<div hidden={current_tab !== 4}>Scripts</div>
			</svelte:fragment>
		</TabGroup>
		<button on:click={send_request} type="button" class="text variant-filled btn mt-5">
			<Send />
			<span class="font-bold">{pending_request ? 'Sending' : 'Send'}</span>
		</button>
	</div>
	<div class="col-span-6">
		{#if $response.get(request.name) !== undefined}
			<ResponseView response={$response.get(request.name) || defaultResponse} />
		{:else}
			<div class="card m-5 p-4 text-center text-xl text-white">
				<section class="p-4">
					<kbd class="kbd">⌘ + Enter</kbd> to send a request.
					<br />
					<kbd class="kbd">⌘ + E</kbd> to edit environment.
				</section>
			</div>
		{/if}
	</div>
</div>
