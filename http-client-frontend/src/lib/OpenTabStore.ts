import { type Writable, writable } from 'svelte/store';
import { type Request } from '$lib/Models';

export let open_tabs_store: Writable<Request[]> = writable([]);
export let tab_number_store: Writable<number> = writable(1);
