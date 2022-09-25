/** Stored function for string truncation */
export function truncate_str(str: string, limit: number): string {
     return str.length > limit ? str.slice(0, limit) + '...' : str;
}