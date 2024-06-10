export const method_to_colour: Map<string, string> = new Map([
	["GET", "variant-filled-success"],
	["POST", "variant-filled-secondary"],
	["PUT", "variant-filled-tertiary"],
	["DELETE", "variant-filled-primary"],
	["PATCH", "variant-filled-warning"]
])

export const method_to_abb: Map<string, string> = new Map([
	["GET", "GET"],
	["POST", "PST"],
	["PUT", "PUT"],
	["DELETE", "DEL"],
	["PATCH", "PTC"]
])

export function limit_request_chars(name: string): string {
	if (name.length > 15) {
		return name.substring(0, 12) + "...";
	}
	return name;
}