export interface Request {
	name: string;
	url: string;
	method: string;
	collection_name: string;
	headers: Header[];
	query_params: QueryParam[];
}

export interface QueryParam {
	name: string;
	value: string;
	enabled: boolean;
}

export interface Header {
	name: string;
	value: string;
	enabled: boolean;
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

export interface History {
	requests: HistoricRequest[];
}

export interface HistoricRequest {
	time: SinceEpochTime;
	url: string;
	method: string;
	response_status: string;
	size: string;
	speed: Duration;
}

export interface SinceEpochTime {
	secs_since_epoch: number,
	nanos_since_epoch: number
}

export interface Duration {
	secs: number,
	nanos: number
}

export interface Response {
	status: string,
	size: string,
	body: string,
	headers: Header[],
	elapsed: Duration,
	content_type: string
}

export function get_scratchpad(): Request {
	return {
		name: 'scratchpad',
		url: 'https://example.com',
		method: "GET",
		collection_name: "scratchpad",
		headers: [],
		query_params: []
	};
}

export function duration_to_string(duration: Duration): string {
	if (duration.nanos > 1000000000) {
		return duration + 's';
	} else {
		return (duration.nanos / 1000000).toFixed() + 'ms';
	}
}