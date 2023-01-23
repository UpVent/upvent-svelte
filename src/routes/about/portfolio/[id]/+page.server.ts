import { getOneTable } from "$lib/common/pbwrap";

import { env } from '$env/dynamic/private';
import PocketBase from "pocketbase";
import type { Record } from "pocketbase";

import type { PageServerLoad } from './$types';

export const load = ( async ({ params }) => {
    const pb: PocketBase = new PocketBase(env.VITE_DBASE_URL);

    await pb.collection('users').authWithPassword(env.VITE_DBASE_USER, env.VITE_DBASE_USER_PASSWORD);
    const records: Record = await getOneTable(pb, 'proyecto_portafolio', params.id);

    records.imagen_destacada = pb.getFileUrl(records, records.imagen_destacada);

    pb.authStore.clear();

    return {
        records: JSON.parse(JSON.stringify(records))
    };

}) satisfies PageServerLoad;
