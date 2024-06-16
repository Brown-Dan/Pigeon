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

export function limit_chars(name: String, limit: number): string {
	if (name.length > limit) {
		return name.substring(0, limit).trimEnd() + "...";
	}
	// @ts-ignore
	return name;
}