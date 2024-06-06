<script lang="ts">
	import type { Request } from '$lib/Models'

	export let request: Request

	function add_query_param() {
		request.query_params.push({name: "", value: "", enabled: true})
		request.query_params = request.query_params
	}

	function delete_query_params() {
		request.query_params = []
		request.query_params = request.query_params
	}
</script>
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