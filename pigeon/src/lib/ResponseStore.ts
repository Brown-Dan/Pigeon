import { type Writable, writable } from 'svelte/store';
import { type Response } from '$lib/Models';

export let response: Writable<Map<String, Response>> = writable(new Map());
