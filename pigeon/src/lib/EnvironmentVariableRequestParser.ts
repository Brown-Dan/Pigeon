import { environments_store, current_environment } from '$lib/EnvironmentStore';
import type { Environment } from '$lib/Models';


export function map_url_to_environment_variable(url: String) {
	let env: Environment;
	current_environment.subscribe(value => env = value);

	// TODO use regex to find and replace - potentially render use <HTML> tags to convert environments variables.
}