<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Import Svelte Carousel
    import Carousel from 'svelte-carousel';

    // Svelte Bootstrap Icons
    import Download from 'svelte-bootstrap-icons/lib/Download';
    import XCircle from 'svelte-bootstrap-icons/lib/XCircle';

    // Sveltestrap imports
    import {
        Button,
        Modal,
        ModalBody,
        ModalFooter
    } from 'sveltestrap';

    // Modal options
    let open = false;
    const toggle = () => (open != open);


    // Projects array
    let projects = [];

    // Get API projects
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/proyecto";

    // Get Projects from Wordpress API
    onMount(async () => {
        // Projects request
        const projects_res = await fetch(api_url);
        const projects_json = await projects_res.json();
        projects = projects_json;
    });
</script>

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
            {:then data}
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
                                        <Button color="danger" on:click={toggle}>Código fuente no disponible <XCircle/></Button>
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


<Modal isOpen={open} {toggle}>
    <ModalBody>
        <p class="h2">¿Por qué el código fuente no está disponible?</p>
        <p class="small text-muted">Hacemos lo posible para que nuestros clientes acepten hacer público el código fuente de los proyectos que hacemos para ellos.</p>
        <br>
        <p class="h2">¿Esto significa que este proyecto es privativo?</p>
        <p class="small text-muted"><b>No</b>, esto significa que el proyecto es software libre, sin embargo el código fuente no es público, es de uso privado. Esto quiere decir que los pocos usuarios que lo utilizan tienen sus 4 libertades del software integras. Si alguna de las personas que poseen el código fuente a la mano decide hacer uso de su libertad de hacer público el código fuente se lo haremos saber pronto :)
        </p>
        <br>
        <p class="small text-muted">Para más información puede consultar los siguientes enlaces:</p>
        <ul>
            <li><a href="https://www.gnu.org/philosophy/free-sw.es.html#selling" target="_blank">¿El software libre puede ser comercial?</a></li>
            <li><a href="https://www.gnu.org/licenses/gpl-faq.es.html#GPLRequireSourcePostedPublic" target="_blank">¿La licencia exige que el código fuente sea publicado?</a></li>
        </ul>
    </ModalBody>
    <ModalFooter>
        <Button color="secondary" on:click={toggle}>Cerrar <XCircle/></Button>
    </ModalFooter>
</Modal>
