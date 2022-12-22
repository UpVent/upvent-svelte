<script lang="ts">
    // Svelte imports
    import { fapi_url } from '../../config';
    import { getTables } from '../..//helpers/pbwrap';
    
    /** Database Imports */
    import PocketBase from 'pocketbase';
    import type { Record } from 'pocketbase';
    
    // Svelte Bootstrap Icons
    import Download from 'svelte-bootstrap-icons/lib/Download.svelte';
    import XCircle from 'svelte-bootstrap-icons/lib/XCircle.svelte';
    
    /** Database Connect */
    const pb: PocketBase = new PocketBase(fapi_url);
    const records: Promise<Record[]> = getTables(pb, 'proyecto_portafolio');
</script>

<section class="container mt-5 mb-5">
    <div class="container text-center">
        <h1>Portafolio de Trabajo</h1>
        <p class="lead text-muted"> Conozca los trabajos de UpVent y la conformidad de sus clientes con los mismos. Además, también podrá ver nuestros proyectos de código libre en favor de la comunidad. </p>
    </div>
</section>

<section class="py-5 bg-light">
    <div id="projects" class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await records}
            <p>Cargando portafolio...</p>
            {:then data}     
            {#each data as record}
            <div class="col">
                <figure>
                    <div class="card h-75 position-relative border-0 shadow-sm-sm p-2">
                        <blockquote class="card-body">
                            <div class="container img-container"><img height="100" class="img-fluid m-1 shadow-sm-sm rounded-3" src="{ pb.getFileUrl(record, record.imagen_destacada) }" alt="proyecto de UpVent"></div>
                            <p class="lead fw-bold">{record.nombre}</p>
                            {#if !record.sigue_activo}
                            <span class="badge rounded-pill text-bg-danger">Inactivo</span> 
                            {:else}
                            <span class="badge rounded-pill text-bg-success">Activo</span> 
                            {/if}
                            <p class="mb-2 text-wrap">{@html record.descripcion }</p>
                            <div class="container">
                                {#if !record.es_libre } <button class="btn btn-danger"> Código fuente no disponible <XCircle/></button> {:else}
                                <a class="btn btn-primary" href="{record.enlace_de_descarga}"> Descargar <Download/></a>
                                {/if}
                            </div>
                        </blockquote>
                    </div>
                </figure>
            </div> 
            {/each}                
            {/await}
        </div>
    </div>
</section>
