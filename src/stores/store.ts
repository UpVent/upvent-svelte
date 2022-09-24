/* Store for API Calls - This is similar to an angular service */
export const api_url: string = import.meta.env.VITE_API_URL;

/** Stored function for string truncation */
export function truncate_str(str: string, limit: number): string {
     return str.length > limit ? str.slice(0, limit) + '...' : str;
}