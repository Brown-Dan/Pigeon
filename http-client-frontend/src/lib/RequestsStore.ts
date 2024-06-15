import { type Writable, writable } from 'svelte/store';
import type { Collection, CollectionMap, Request, Requests } from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';

export const requests: Writable<Requests> = writable();

export const collections: Writable<Map<String, CollectionMap>> = writable();

invoke('get_collections', {}).then((value) => <Requests>value)
	.then((value) => {
		requests.set(value);
		collections.set(new Map(value.collections
			.map(collection => [collection.name, map_collection_to_collection_map(collection)])));
	});

function map_collection_to_collection_map(collection: Collection): CollectionMap {
	let request_map: Map<String, Request> = new Map(collection.requests.map(request => [request.name, request]));
	return {
		name: collection.name,
		description: collection.description,
		requests: request_map
	};
}