import { getOneTable } from "$lib/common/pbwrap";

import { env } from '$env/dynamic/private';
import PocketBase from "pocketbase";
import type { Record } from "pocketbase";

export const load = ( async () => {
    const pb: PocketBase = new PocketBase(env.VITE_DBASE_URL);

    await pb.collection('users').authWithPassword(env.VITE_DBASE_USER, env.VITE_DBASE_USER_PASSWORD);
    const records: Record = await getOneTable(pb, 'politica_privacidad', 'mprk14ec3apayvr');

    records.imagen_destacada = pb.getFileUrl(records, records.imagen_destacada);

    pb.authStore.clear();

    return {
        records: JSON.parse(JSON.stringify(records))
    };
})
