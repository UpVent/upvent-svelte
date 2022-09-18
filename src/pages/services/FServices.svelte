<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { api_url } from '../../stores/store';

    // Svelte Bootstrap Icons
    import { Download } from 'svelte-bootstrap-icons';

    // Image imports
    import oneplace from '../../assets/images/oneplace.webp';

    // Products array
    let products: any[] = [];

    // Get API products
    const url: string = api_url + "software_libre";

    onMount(async () => {
        const projects_res: Response = await fetch(url);
        products = await projects_res.json();
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
            {#each products as product}
                <div class="col">
                    <figure>
                        <div class="card h-75 position-relative border-0 shadow-sm p-2">
                            <img height="100" width="100" class="img-fluid m-1 shadow-md rounded-circle" src={product.imagen.guid} alt="Imagen del producto de software libre">
                            <p class="lead fw-bold">{product.nombre}</p>
                        </div>
                        <blockquote>
                            <div class="container">
                                <p class="card-text text-muted text-wrap">{product.descripcion}</p>
                                <a class="btn btn-primary" href="{product.link}">Descargar <Download/></a>
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
