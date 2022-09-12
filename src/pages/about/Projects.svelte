<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { api_url } from '../../stores/store';
    import type { Project } from './About';
    
    // Svelte Bootstrap Icons
    import { Download, XCircle, StarFill } from 'svelte-bootstrap-icons';
    
    // Projects array
    let projects: Project[] = [];
    
    // Get API projects
    const url: string = api_url + "proyecto";
    
    onMount(async () => {
        const projects_res: Response = await fetch(url);
        projects = await projects_res.json();
    });
</script>

<style>
    * { font-family: 'Poppins', sans-serif; }
</style>

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
            {#each projects as project}
            <div class="col">
                <figure>
                    <div class="card h-75 position-relative border-0 shadow-sm p-2">
                            {#if project.es_libre == "0"}
                            <span class="btn btn-danger shadow-sm pe-none position-absolute top-0 start-0 translate-middle-y ms-4"><XCircle/></span>
                            {:else}
                            <span class="btn btn-warning shadow-sm pe-none position-absolute top-0 start-0 translate-middle-y ms-4"><StarFill/></span>
                            {/if}
                        <blockquote class="card-body">
                            <div class="container img-container"><img height="100" class="img-fluid m-1 shadow-sm rounded-3" src="{project.imagen_del_proyecto.guid}" alt="proyecto de UpVent"></div>
                            <p class="lead fw-bold">{project.title.rendered}</p>
                            {#if project.sigue_activo == "1" } <span class="badge rounded-pill text-bg-success">Activo</span> {:else}
                                <span class="badge rounded-pill text-bg-danger">Inactivo</span>
                            {/if}
                            <p class="mb-2 text-wrap">{@html project.descripcion_corta }</p>
                            <div class="container">
                                {#if project.es_libre == "0"} <a class="btn btn-danger" href="http://"> Código fuente no disponible <XCircle/></a> {:else}
                                <a class="btn btn-primary" href="{project.link_de_descarga}"> Descargar <Download/></a>
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
