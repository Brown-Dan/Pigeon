import { type Writable, writable } from 'svelte/store';
import { get_scratchpad, type Request } from '$lib/Models';

export let open_tabs_store: Writable<Request[]> = writable([]);
