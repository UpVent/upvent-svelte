<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { fapi_url, api_user, api_user_pass } from '../../config';

    /** Database Imports */
    import PocketBase from 'pocketbase';
    import type { Record } from 'pocketbase';

    // Svelte Bootstrap Icons
    import { Download } from 'svelte-bootstrap-icons';

    /** Database Connect */
    const client: PocketBase = new PocketBase(fapi_url);
    let records: Record[] = [];

    // Image imports
    import oneplace from '../../assets/images/oneplace.webp';

    onMount(async () => {
        const user_auth_data = await client.users.authViaEmail(api_user, api_user_pass);
        records = await client.records.getFullList('proyectos_libres', 10);
        client.authStore.clear();
    });
</script>

<section class="container">
    <div class="container text-center mt-5 mb-5">
        <h2 class="fw-bold display-3">Software <span class="text-primary text-glow">libre</span> al alcance de su mano.</h2>
        <p class="text-muted"> Conozca los proyectos de Software Libre que UpVent ha preparado para ustedy su empresa.</p>
    </div>
    <div class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
        {#await onMount}
            <p class="text-muted lead">Cargando los productos de software libre para usted...</p>
        {:then}
            {#each records as record}
                <div class="col">
                    <figure>
                        <div class="card h-75 position-relative border-0 shadow-sm p-2">
                            <img height="100" width="100" class="img-fluid m-1 shadow-md rounded-circle" src="{client.records.getFileUrl(record, record.imagen)}" alt="Producto de software libre">
                            <p class="lead fw-bold">{record.nombre}</p>
                        </div>
                        <blockquote>
                            <div class="container">
                                <p class="card-text text-muted text-wrap">{record.descripcion}</p>
                                <a class="btn btn-primary" href="{record.enlace}">Descargar <Download/></a>
                            </div>
                        </blockquote>
                    </figure>
                </div>
            {/each}
        {:catch error}
            <p class="text-danger">Error al obtener los productos de software libre. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}</p>
        {/await}
    </div>
</section>

<section class="container mt-5 py-16 py-sm-24 py-lg-32">
    <div class="mx-auto max-w-md px-4 text-center">
        <h2 class="text-primary">SOFTWARE ESTABLE, LISTO PARA USARSE</h2>
        <p class="mt-2 text-3lx text-muted">UpVent construye software eficiente desde el primer día, listo para uso empresarial y proporcionando una estabilidad sólida con actualizaciones periódicas gratuitas.</p>
    </div>
    <div class="container"><img class="img-fluid" src={oneplace} alt=""/></div>
    <p class="text-center display-5  text-glow text-primary fw-bold">¡Todo en un solo lugar!</p>
    <p class="text-center lead">Ofrecemos una variedad de servicios pagados. ¿Necesita de un CRM, un e-commerce o una solución personalizada? Para UpVent no hay obstáculos, ¡Lo tenemos cubierto en cualquier necesidad tecnológica!</p>
</section>
