<script lang="ts">
    // Svelte imports
    import { onMount, onDestroy } from 'svelte';
    import { api_user, api_user_pass, fapi_url } from '../../config';
    import { api_result } from '../../stores/store';

    /** Database Imports */
    import PocketBase from 'pocketbase';
    
    // Svelte Bootstrap Icons
    import Download from 'svelte-bootstrap-icons/lib/Download.svelte';
    import XCircle from 'svelte-bootstrap-icons/lib/XCircle.svelte';
    import StarFill from 'svelte-bootstrap-icons/lib/StarFill.svelte';
    
    /** Database Connect */
    const client: PocketBase = new PocketBase(fapi_url);

    onMount(async () => {
        client.users.authViaEmail(api_user, api_user_pass);
        $api_result = await client.records.getFullList('proyecto_portafolio', 10);
        client.authStore.clear();
    });

    onDestroy(() => { $api_result.length = 0; });
</script>

<section class="container mt-5 mb-5">
    <div class="container text-center">
        <h1>Portafolio de Trabajo</h1>
        <p class="lead text-muted"> Conozca los trabajos de UpVent y la conformidad de sus clientes con los mismos. Además, también podrá ver nuestros proyectos de código libre en favor de la comunidad. </p>
    </div>
</section>

<section class="album py-5 bg-light">
    <div id="projects" class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await onMount}
            <p class="text-muted lead">Cargando los proyectos realizados por UpVent...</p>
            {:then}
            {#each $api_result as record}
            <div class="col">
                <figure>
                    <div class="card h-75 position-relative border-0 shadow-sm p-2">
                            {#if !record.es_libre }
                                <span class="btn btn-danger shadow-sm pe-none position-absolute top-0 start-0 translate-middle-y ms-4"><XCircle/></span>
                            {:else}
                                <span class="btn btn-warning shadow-sm pe-none position-absolute top-0 start-0 translate-middle-y ms-4"><StarFill/></span>
                            {/if}
                        <blockquote class="card-body">
                            <div class="container img-container"><img height="100" class="img-fluid m-1 shadow-sm rounded-3" src="{ client.records.getFileUrl(record, record.imagen_destacada) }" alt="proyecto de UpVent"></div>
                            <p class="lead fw-bold">{record.nombre}</p>
                            {#if !record.sigue_activo}
                               <span class="badge rounded-pill text-bg-danger">Inactivo</span> 
                            {:else}
                               <span class="badge rounded-pill text-bg-success">Activo</span> 
                            {/if}
                            <p class="mb-2 text-wrap">{@html record.descripcion }</p>
                            <div class="container">
                                {#if !record.es_libre } <a class="btn btn-danger" href="/"> Código fuente no disponible <XCircle/></a> {:else}
                                <a class="btn btn-primary" href="{record.enlace_de_descarga}"> Descargar <Download/></a>
                                {/if}
                            </div>
                        </blockquote>
                    </div>
                </figure>
            </div> 
            {/each}                
            {:catch error}
            <p class="text-danger">Error al obtener los proyectos. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}</p>
            {/await}
        </div>
    </div>
</section>
