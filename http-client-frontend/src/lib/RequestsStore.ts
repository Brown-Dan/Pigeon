import { type Writable, writable } from 'svelte/store';
import type { Requests } from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';

export const requests: Writable<Requests> = writable()

invoke('get_collections', {}).then((value) => <Requests>value)
	.then(value => requests.set(value));

