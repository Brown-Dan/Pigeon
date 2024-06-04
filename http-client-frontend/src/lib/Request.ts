export interface Request {
	name: string;
	url: string;
	method: string;
}

export interface Collection {
	name: string;
	description: string;
	requests: Request[];
}

export interface Requests {
	collections: Collection[],
	orphaned_requests: Request[]
}