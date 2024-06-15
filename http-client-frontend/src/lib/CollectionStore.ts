import { type Writable, writable } from 'svelte/store';
import {
	type Collection,
	type CollectionMap,
	type Collections, isNotScratchpad,
	isOrphan,
	type Request,
	type Requests
} from '$lib/Models';
import { invoke } from '@tauri-apps/api/tauri';
import { open_tabs } from '$lib/TabStore';

export const collections_store: Writable<Collections> = writable();

invoke('get_collections', {}).then((value) => <Requests>value)
	.then((value) => {
		collections_store.set(map_requests_to_collections(value));
	});

open_tabs.subscribe((open_tabs) => {
	open_tabs.forEach(request => {
			collections_store.update((collections) => {
				if (isOrphan(request)) {
					collections.orphan_requests.set(request.name, request)
				} else if (isNotScratchpad(request)) {
					console.log(request.collection_name)
					console.log(collections)
					collections.collections.get(request.collection_name)!.requests.set(request.name, request)
				}
				return collections;
			})
	})
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
