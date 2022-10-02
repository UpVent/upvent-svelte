import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

/** Stored function for string truncation */
export function truncate_str(str: string, limit: number): string {
     return str.length > limit ? str.slice(0, limit) + '...' : str;
}

/** Stored array for pages with a single fetch component */
export const api_result: Writable<any[]> = writable([]);