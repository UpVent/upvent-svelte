/* Store for API Calls - This is similar to an angular service */
import { readable } from "svelte/store";

export const api_url = readable('https://wpapi.upvent.codes/wp-json/wp/v2');