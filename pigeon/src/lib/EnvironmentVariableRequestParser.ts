import { current_environment } from '$lib/EnvironmentStore';
import type { Environment } from '$lib/Models';


export function map_url_to_environment_variable(url: String): string {
	console.log(url);
	let env: Environment;
	current_environment.subscribe(value => env = value);

// Regular expression pattern to find text between curly braces
	const pattern = /\{([^{}]*)\}/g;

// Using match to extract all occurrences
	const matches = url.match(pattern);

	url.replaceAll(pattern, "")

// Processing the matches to remove the curly braces
	const result = matches ? matches.map(match => match.slice(1, -1)) : [];

	let url_mapped: string = "";
	result.forEach(value => {
		let idx = env.values.map(val => val.name).indexOf(value, 0)
		if (idx >= 0) {
			url_mapped = url_mapped + env.values[idx].value
		}
	});
	console.log(url_mapped)
	return url_mapped.toString();
}