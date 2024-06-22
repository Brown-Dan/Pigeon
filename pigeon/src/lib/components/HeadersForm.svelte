<script lang="ts">
	import type { Request } from '$lib/Models';
	import { Trash2 } from 'lucide-svelte';
	export let request: Request;

	function add_header() {
		request.headers.push({ name: '', value: '', enabled: true });
		request.headers = request.headers;
	}

	function delete_headers() {
		request.headers = [];
		request.headers = request.headers;
	}

	function delete_header(index: number) {
		request.headers.splice(index, 1);
		request.headers = request.headers.filter(Boolean);
	}
</script>

<div class="variant-filled btn-group mb-5">
	<button type="button" class="btn-sm" on:click={add_header}>Add</button>
	<button type="button" class="btn-sm" on:click={delete_headers}>Delete All</button>
</div>
{#each request.headers as header, index}
	<div class="input-group input-group-divider mb-4 flex w-fit space-x-4">
		<input
			class="m-0 w-fit px-1 text-xs"
			bind:value={header.name}
			type="text"
			placeholder="name"
			disabled={!header.enabled}
		/>
		<input
			class="m-0 w-fit px-1 text-xs"
			bind:value={header.value}
			type="text"
			placeholder="value"
			disabled={!header.enabled}
		/>
		<div class="input-group-shim flex flex-row space-x-3">
			<input bind:checked={header.enabled} class="checkbox" type="checkbox" />
			<button
				on:click={() => delete_header(index)}
				type="button"
				class="variant-filled btn-icon mr-2"
			>
				<Trash2 size="12" />
			</button>
		</div>
	</div>
{/each}
