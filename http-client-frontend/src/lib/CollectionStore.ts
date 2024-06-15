import { type Writable, writable } from 'svelte/store';
import type { Collection, CollectionMap, Collections, Request, Requests } from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';

export const collections_store: Writable<Collections> = writable();

invoke('get_collections', {}).then((value) => <Requests>value)
	.then((value) => {
		collections_store.set(map_requests_to_collections(value));
	});

function map_requests_to_collections(requests: Requests): Collections {
	return {
		collections: new Map(requests.collections
			.map(collection => [collection.name, map_collection_to_collection_map(collection)])),
		orphan_requests: new Map(requests.orphaned_requests.map(request => [request.name, request]))
	}
}

function map_collection_to_collection_map(collection: Collection): CollectionMap {
	let request_map: Map<String, Request> = new Map(collection.requests.map(request => [request.name, request]));
	return {
		name: collection.name,
		description: collection.description,
		requests: request_map
	};
}
