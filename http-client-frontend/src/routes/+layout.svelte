<script lang="ts">
	import '../app.postcss';
	import { arrow, autoUpdate, computePosition, flip, offset, shift } from '@floating-ui/dom';
	import {
		AppBar,
		AppShell,
		initializeStores,
		storePopup,
		Toast,
		TreeView,
		TreeViewItem
	} from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { Requests } from '$lib/Request';
	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	function open_request_tab(request: Request) {
		window.dispatchEvent(new CustomEvent('requestBarClick', { detail: request}));

	}

	let requests: Promise<Requests> = invoke('get_collections', {}).then((value) => <Requests>value);
</script>
<Toast />
{#await requests}
{:then files}
	<AppShell>
		<svelte:fragment slot="header">
			<AppBar>
				<svelte:fragment slot="lead">
					<img class="ml-3 max-h-12 max-w-28" src="http://www.i2clipart.com/cliparts/f/5/c/a/clipart-pigeon-64x64-f5ca.png" />
				</svelte:fragment>
				<svelte:fragment slot="trail">
					<div class="btn-group variant-filled">
						<button type="button" class="btn-sm variant-filled">
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z" />
							</svg>
						</button>
						<button type="button" class="btn-sm variant-filled">
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
							</svg>
						</button>
						<button type="button" class="btn-sm variant-filled">
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
							</svg>
						</button>
						<button type="button" class="btn-sm variant-filled">
							<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
									 stroke="currentColor" class="size-6">
								<path stroke-linecap="round" stroke-linejoin="round"
											d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 0 1 1.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.559.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.894.149c-.424.07-.764.383-.929.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 0 1-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.398.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 0 1-.12-1.45l.527-.737c.25-.35.272-.806.108-1.204-.165-.397-.506-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.108-1.204l-.526-.738a1.125 1.125 0 0 1 .12-1.45l.773-.773a1.125 1.125 0 0 1 1.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894Z" />
								<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
							</svg>
						</button>
					</div>
				</svelte:fragment>
				<svelte:fragment slot="headline">
					Pigeon
				</svelte:fragment>
			</AppBar>
		</svelte:fragment>
		<svelte:fragment slot="sidebarLeft">
			<TreeView class="m-5 hidden lg:block min-w-72">
				{#each files.collections as collection}
					<TreeViewItem>
						{collection.name}
						<svelte:fragment slot="children">
							{#each collection.requests as request}
								<TreeViewItem class="text-center" on:click={() => open_request_tab(request)}>
									{request.name}<span class="ml-4 badge variant-filled-success">{request.method}</span>
								</TreeViewItem>
							{/each}
						</svelte:fragment>
					</TreeViewItem>
				{/each}
				{#each files.orphaned_requests as request}
					<TreeViewItem on:click={() => open_request_tab(request)}>
						{request.name}<span class="ml-4 badge variant-filled-warning">{request.method}</span>
					</TreeViewItem>
				{/each}
			</TreeView>
		</svelte:fragment>
		<slot />
	</AppShell>
{:catch error}
{/await}
