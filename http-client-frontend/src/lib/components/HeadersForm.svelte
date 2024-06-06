<script lang="ts">
	import type { Request } from '$lib/Models'
	export let request: Request;

	function add_header() {
		request.headers.push({name: "", value: "", enabled: true})
		request.headers = request.headers
	}

	function delete_headers() {
		request.headers = []
		request.headers = request.headers
	}
</script>

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