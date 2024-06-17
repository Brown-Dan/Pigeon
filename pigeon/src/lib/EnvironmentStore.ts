import { writable, type Writable } from 'svelte/store';
import { type Environment, type Environments, get_default_environment } from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';

export const environments_store: Writable<Map<String, Environment>> = writable();

export const current_environment: Writable<Environment> = writable(get_default_environment());

invoke('get_environments', {}).then((value) => <Environments>value)
	.then((value) => {
		environments_store.set(new Map(value.environments.map(val => [val.name, val])));
	});