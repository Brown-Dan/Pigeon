import { type Writable, writable } from 'svelte/store';
import { type Request } from '$lib/Models';

export let open_tabs: Writable<Request[]> = writable([]);
export let current_tab_index: Writable<number> = writable(-1);

export function increment() {
	current_tab_index.update(value => value += 1);
}

export function decrement() {
	current_tab_index.update(value => value -= 1);
}