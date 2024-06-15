import { writable } from 'svelte/store';
import { get_scratchpad } from '$lib/Models';


export let request = writable(get_scratchpad());
