export interface Request {
	name: string;
	url: string;
	method: string;
	collection_name: String;
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
	requests: HistoricRequest[]
}

export interface HistoricRequest {
	time: SinceEpochTime
	url: string;
	method: string;
	response_status: string;
	size: string;
	speed: Duration
}

export interface SinceEpochTime {
	secs_since_epoch: number,
	nanos_since_epoch: number
}

export interface Duration {
	secs: number,
	nanos: number
}

export function duration_to_string(duration: Duration): string {
	if (duration.nanos > 1000000000) {
		return duration + "s"
	} else {
		return (duration.nanos / 1000000).toFixed() + "ms"
	}
}