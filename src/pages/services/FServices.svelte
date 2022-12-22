<script lang="ts">
    // Svelte imports
    import { fapi_url } from '../../config';
    import { getTables } from '../../helpers/pbwrap';

    // Image imports
    import oneplace from '../../assets/images/oneplace.webp';

    /** Database Imports */
    import PocketBase from 'pocketbase';
    import type { Record } from 'pocketbase';

    // Svelte Bootstrap Icons
    import Download from 'svelte-bootstrap-icons/lib/Download.svelte';

    /** Database Connect */
    const pb: PocketBase = new PocketBase(fapi_url);
    const records: Promise<Record[]> = getTables(pb, 'proyectos_libres');
</script>

<section class="container">
    <div class="container text-center mt-5 mb-5">
        <h2 class="fw-bold display-3">Software <span class="text-primary">libre</span> al alcance de su mano.</h2>
        <p class="text-muted"> Conozca los proyectos de Software Libre que UpVent ha preparado para ustedy su empresa.</p>
    </div>
    <div class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await records}
                <p>Cargando proyectos libres...</p>
            {:then data}     
            {#each data as record}
                <div class="col">
                    <figure>
                        <div class="card h-75 position-relative border-0 shadow-sm-sm p-2">
                            <img height="100" width="100" class="img-fluid m-1 shadow-sm-md rounded-circle" src="{pb.getFileUrl(record, record.imagen)}" alt="Producto de software libre">
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
            {/await}
    </div>
</section>

<section class="container mt-5 py-16 py-sm-24 py-lg-32">
    <div class="mx-auto max-w-md px-4 text-center">
        <h2 class="text-primary">SOFTWARE ESTABLE, LISTO PARA USARSE</h2>
        <p class="mt-2 text-3lx text-muted">UpVent construye software eficiente desde el primer día, listo para uso empresarial y proporcionando una estabilidad sólida con actualizaciones periódicas gratuitas.</p>
    </div>
    <div class="container"><img class="img-fluid" src={oneplace} alt=""/></div>
    <p class="text-center display-5 text-primary fw-bold">¡Todo en un solo lugar!</p>
    <p class="text-center lead">Ofrecemos una variedad de servicios y software pre-hecho. ¿Necesita de un CRM, un e-commerce o una solución personalizada? Para UpVent no hay obstáculos, ¡Lo tenemos cubierto en cualquier necesidad tecnológica!</p>
</section>