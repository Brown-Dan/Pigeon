import { type Writable, writable } from 'svelte/store';
import { get_scratchpad, type Request } from '$lib/Models';
import { request } from '$lib/RequestStore';

export let open_tabs: Writable<Request[]> = writable([]);
export let current_tab_index: Writable<number> = writable(-1);

export function increment() {
	current_tab_index.update(value => value += 1);
}

export function decrement() {
	current_tab_index.update(value => value -= 1);
}

current_tab_index.subscribe(value => {
	const currentTabIndex = value;
	open_tabs.subscribe(tabs => {
		if (tabs && tabs.length > currentTabIndex && currentTabIndex >= 0) {
			request.set(tabs[currentTabIndex]);
		} else {
			request.set(get_scratchpad());
		}
	});
});