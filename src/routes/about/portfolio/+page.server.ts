import { getTables } from "$lib/common/pbwrap";

import { env } from '$env/dynamic/private';
import PocketBase from "pocketbase";
import type { Record } from "pocketbase";

export const load = async () => {
    const pb: PocketBase = new PocketBase(env.VITE_DBASE_URL);

    await pb.collection('users').authWithPassword(env.VITE_DBASE_USER, env.VITE_DBASE_USER_PASSWORD);
    const records: Record[] = await getTables(pb, 'proyecto_portafolio');

    const imgPromises: Promise<string>[] = records.map(async (r) => {
        return pb.getFileUrl(r, r.imagen_destacada);
    });

    const imgUrls: string[] = await Promise.all(imgPromises);

     // Agregar las url de las imágenes al objeto
    const projects = records.map((r, i) => {
        const project = r.export();
        project.img = imgUrls[i];
        return project;
    });

    pb.authStore.clear();

    return { projects };
}
