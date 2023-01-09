/**
 * Truncate a string if it is longer than the specified number of characters.
 * @param {string} str - The string to truncate.
 * @param {number} num - The maximum length of the truncated string.
 * @return {string} The truncated string. If the string is shorter than `num`, the original string is returned.
 */
export function truncateString(str: string, num: number): string {
    if (str.length > num) {
        return str.slice(0, num).concat("...");
    } else {
        return str;
    }
}
