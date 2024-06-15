<script lang="ts">
	import type { Request } from '$lib/Models';

	export let request: Request;

</script>

<div class="input-group input-group-divider grid-cols-[1fr_auto] mb-5">
	<input bind:value={request.url} type="text" placeholder="https://example.com/" id="url" />
	<select bind:value={request.method} id="method">
		<option value="GET">GET</option>
		<option value="PUT">PUT</option>
		<option value="PATCH">PATCH</option>
		<option value="DELETE">DELETE</option>
		<option value="POST">POST</option>
	</select>
</div>