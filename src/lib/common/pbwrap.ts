import type PocketBase from "pocketbase";
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


/**
 * Wraps the `getOne` method to reuse on other components.
 *
 * @export
 * @param {PocketBase} pb
 * @param {string} table
 * @param {string} id
 * @return {*}  {Promise<Record>}
 */
export async function getOneTable(pb: PocketBase, table: string, id: string): Promise<Record> {
    return await pb.collection(table).getOne(id);
}
