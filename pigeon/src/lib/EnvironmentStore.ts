import { writable, type Writable } from 'svelte/store';
import type { Environment, Environments } from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';

export const collections_store: Writable<Map<String, Environment>> = writable();

invoke('get_environments', {}).then((value) => <Environments>value)
	.then((value) => {
		collections_store.set(new Map(value.environments.map(val => [val.name, val])));
	});