<script lang="ts">
	import type { Request } from '$lib/Models'
	import { Trash2 } from 'lucide-svelte';

	export let request: Request

	function add_query_param() {
		request.query_params.push({name: "", value: "", enabled: true})
		request.query_params = request.query_params
	}

	function delete_query_params() {
		request.query_params = []
		request.query_params = request.query_params
	}

	function delete_query_param(index: number) {
		request.query_params.splice(index, 1);
		request.query_params = request.query_params.filter(Boolean);
	}
</script>
<div class="variant-filled btn-group mb-5">
	<button type="button" class=" btn-sm" on:click={add_query_param}>Add</button>
	<button type="button" class=" btn-sm" on:click={delete_query_params}>Delete All</button>
</div>
{#each request.query_params as query_param, index}
	<div class="input-group input-group-divider mb-4 flex w-fit space-x-4">
		<input
			class="m-0 w-fit px-1 text-xs"
			bind:value={query_param.name}
			type="text"
			placeholder="name"
			disabled={!query_param.enabled}
		/>
		<input
			class="m-0 w-fit px-1 text-xs"
			bind:value={query_param.value}
			type="text"
			placeholder="value"
			disabled={!query_param.enabled}
		/>
		<div class="input-group-shim flex flex-row space-x-3">
			<input bind:checked={query_param.enabled} class="checkbox" type="checkbox" />
			<button
				on:click={() => delete_query_param(index)}
				type="button"
				class="btn-icon mr-2"
			>
				<Trash2 size="12" />
			</button>
		</div>
	</div>
{/each}