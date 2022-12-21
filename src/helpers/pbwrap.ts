import PocketBase from "pocketbase";
import type { Record } from "pocketbase";

/**
 * Wraps the `getFullList` method to reuse on other components.
 *
 * @export
 * @param {PocketBase} pb
 * @param {string} table
 * @return {*}  {Promise<Record[]>}
 */
export async function getTables(pb: PocketBase, table: string): Promise<Record[]> {
    return await pb.collection(table).getFullList();
}