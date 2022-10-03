import { writable } from "svelte/store";
import type { Writable } from "svelte/store";

/** Stored array for pages with a single fetch component */
export const api_result: Writable<any[]> = writable([]);