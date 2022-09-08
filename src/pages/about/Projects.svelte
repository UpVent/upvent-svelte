<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { api_url } from '../../stores/store';
    import type { Project } from './About';

    // Import Svelte Carousel
    import Carousel from 'svelte-carousel';

    // Svelte Bootstrap Icons
    import { Download, XCircle } from 'svelte-bootstrap-icons';

    // Projects array
    let projects: Project[] = [];

    // Get API projects
    const url: string = api_url + "proyecto";

    // Get Projects from Wordpress API
    onMount(async () => {
        // Projects request
        const projects_res: Response = await fetch(url);
        const projects_json: Project[] = await projects_res.json() as Project[];
        projects = projects_json;
        console.log(projects)
    });
</script>

<style>
    * {
        font-family: 'Poppins', sans-serif;
    }
</style>

<section class="container mt-5 mb-5">
    <div class="container text-center">
        <h1>Portafolio de Trabajo</h1>
        <p class="lead text-muted">
            Conozca los trabajos de UpVent y la conformidad de sus clientes con los mismos. Además, también podrá ver nuestros proyectos de código libre en favor de la comunidad.
        </p>
    </div>
</section>

<section class="album py-5 bg-light">
    <div id="projects" class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await onMount}
                <p class="text-muted lead">Cargando los proyectos realizados por UpVent...</p>
            {:then}
            <Carousel>
                {#each projects as project}
                    <div class="col">
                        <div class="card shadow-sm rounded-3">
                            <img class="card-img-top" alt="Imágen mostrando un proyecto creado por UpVent" src={project.imagen_del_proyecto.guid}/>
                            <div class="card-body">
                                <h5 class="fs-2 fw-bold">{ project.title.rendered }</h5>
                                <p class="card-text">{@html project.descripcion_corta }</p>
                                <div class="d-flex justify-content-center align-items-center">
                                    {#if project.es_libre == "0" }
                                        <a class="btn btn-primary" href="https://upvent.codes/">Código fuente no disponible <XCircle/></a>
                                    {:else}
                                        <a class="btn btn-primary" href="{project.link_de_descarga}">Descargar código fuente <Download/></a>
                                    {/if}
                                </div>
                            </div>
                        </div>
                    </div>
                {/each}
            </Carousel>
            {:catch error}
                <p class="text-danger">Error al obtener los proyectos. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}</p>
            {/await}
        </div>
    </div>
</section>
