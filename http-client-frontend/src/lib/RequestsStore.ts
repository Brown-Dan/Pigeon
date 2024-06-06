import { type Writable, writable } from 'svelte/store';
import type { Requests } from '$lib/Models';

export const requests: Writable<Requests> = writable()

