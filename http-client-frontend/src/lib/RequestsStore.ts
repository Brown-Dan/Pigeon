import { type Writable, writable } from 'svelte/store';
import type { Requests } from '$lib/Request';

export const requests: Writable<Requests> = writable()

