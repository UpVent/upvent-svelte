<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';

    // Import Svelte Carousel
    import Carousel from 'svelte-carousel';

    // Svelte Bootstrap Icons
    import { Download } from 'svelte-bootstrap-icons';

    // Image imports
    import oneplace from '../../assets/images/oneplace.webp';

    // Products array
    let products = [];

    // Get API products
    const api_url:string = "https://wpapi.upvent.codes/wp-json/wp/v2/software_libre";

    // Get projects from wordpress API
    onMount(async () => {
        // Projects request
        const projects_res = await fetch(api_url);
        const projects_json = await projects_res.json();
        products = projects_json;
    });
</script>

<section class="container">
    <div class="container text-center mt-5 mb-5">
        <h2 class="fw-bold display-3">Software <span class="text-primary text-glow">libre</span> al alcance de su mano.</h2>
        <p class="text-muted">
            Conozca los proyectos de Software Libre que UpVent ha preparado para ustedy su empresa.
        </p>
    </div>

    <div class="container">

        {#await onMount}
            <p class="text-muted lead">
                Cargando los productos de software libre para usted...
            </p>
        {:then}
            <Carousel autoplay>
                {#each products as product}
                    <div class="container text-center">
                        <div class="card border border-0 mx-auto shadow-sm rounded">
                            <img class="img-fluid p-2 mx-auto" height="200" width=200 src="{product.imagen.guid}" alt="Imágen del producto de software libre">
                            <div class="card-body">
                                <p class="h5 mt-2 fw-bold">{product.nombre}</p>
                                <hr>
                                <p class="card-text text-muted small text-wrap text-break">{product.descripcion}</p>
                                <div class="container">
                                    <a class="btn btn-primary" href="{product.link}">Descargar <Download/></a>
                                </div>
                            </div>
                        </div>
                    </div>
                {/each}
            </Carousel>
        {:catch error}
            <p class="text-danger">
                Error al obtener los productos de software libre. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}
            </p>
        {/await}
    </div>
</section>

<section class="container mt-5 py-16 py-sm-24 py-lg-32">
    <div class="mx-auto max-w-md px-4 text-center">
        <h2 class="text-primary">SOFTWARE ESTABLE, LISTO PARA USARSE</h2>
        <p class="mt-2 text-3lx text-muted">UpVent construye software eficiente desde el primer día, listo para uso empresarial y proporcionando una estabilidad sólida con actualizaciones periódicas gratuitas.</p>
    </div>
    <div class="container">
        <img class="img-fluid" src="{oneplace}" alt=""/>
    </div>
    <p class="text-center display-5  text-glow text-primary fw-bold">¡Todo en un solo lugar!</p>
    <p class="text-center lead">Ofrecemos una variedad de servicios pagados. ¿Necesita de un CRM, un e-commerce o una solución personalizada? Para UpVent no hay obstáculos, ¡Lo tenemos cubierto en cualquier necesidad tecnológica!</p>
</section>
