<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { TabGroup, Tab, TabAnchor } from '@skeletonlabs/skeleton';
	import { CodeBlock } from '@skeletonlabs/skeleton';

	let response: JSON;
	let tabSet: number = 0;

	function send_request() {
		const url: string =  (<HTMLInputElement>document.getElementById("url")).value;
		console.log(url)
		 invoke("send_request", { url }).then(value => {
			 if (typeof value === 'string') {
				 response = JSON.parse(value);
			 }
			 console.log(value);
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
					<div class="border-white bg-slate-200 min-w-screen min-h-screen text-black rounded-2xl text-wrap p-5">
						{#if response}
							<CodeBlock language="json" code={response.body}></CodeBlock>
						{/if}
					</div>
				{:else if tabSet === 1}
					<CodeBlock language="html" code={`<div>This is meta</div>`}></CodeBlock>
				{:else if tabSet === 2}
					(tab panel 3 contents)
				{/if}
			</svelte:fragment>
		</TabGroup>
	</div>
</div>


