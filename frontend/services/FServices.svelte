<script>
    // Import Svelte Carousel
    import Carousel from 'svelte-carousel';

    // Sveltestrap components
    import { Card, CardBody, CardText } from 'sveltestrap';

    // Svelte Bootstrap Icons
    import Download from 'svelte-bootstrap-icons/lib/Download';

    // Products array (Not worth saving on a database)
    let products = [{
            name: 'Orchid',
            image: '/images/orchid.svg',
            description: 'Orchid es un script de navaja suiza de Debian/Ubuntu Server para automatizar las tareas del servidor',
            link: 'https://github.com/UpVent/orchid',
        },
        {
            name: 'Weeping Moon',
            image: '/images/weepingmoon.webp',
            description: 'Pequeño script bash para fortalecer las nuevas instalaciones del servidor Ubuntu.',
            link: 'https://github.com/UpVent/weeping-moon',
        },
        {
            name: 'Orchid EPEL',
            image: '/images/orchidepel.webp',
            description: 'Orchid es un script de navaja suiza de Red Hat Enterprise Linux para automatizar las tareas del servidor.',
            link: 'https://github.com/UpVent/orchid-EPEL',
        },
    ];
</script>

<section class="container">
    <div class="container text-center mt-5 mb-5">
        <h2 class="fw-bold display-3">Software <span class="text-primary text-glow">libre</span> al alcance de su mano.</h2>
        <p class="text-muted">
            Conozca los proyectos de Software Libre que UpVent ha preparado para ustedy su empresa.
        </p>
    </div>

    <div class="container">
        <Carousel autoplay>
            {#each products as product}
                <div class="container text-center">
                    <Card class="border border-0 mx-auto shadow-sm rounded">
                        <img class="img-fluid p-2 mx-auto" height="200" width=200 src="{product.image}" alt="Imágen del producto de software libre">
                        <CardBody>
                            <p class="h5 mt-2 fw-bold">{product.name}</p>
                            <hr>
                            <CardText class="text-muted small text-wrap text-break">{product.description}</CardText>
                            <div class="container">
                                <a class="btn btn-primary" href="{product.link}">Descargar <Download/></a>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            {/each}
        </Carousel>
    </div>
</section>

<section class="container mt-5 py-16 py-sm-24 py-lg-32">
    <div class="mx-auto max-w-md px-4 text-center">
        <h2 class="text-primary">SOFTWARE ESTABLE, LISTO PARA USARSE</h2>
        <p class="mt-2 text-3lx text-muted">UpVent construye software eficiente desde el primer día, listo para uso empresarial y proporcionando una estabilidad sólida con actualizaciones periódicas gratuitas.</p>
    </div>
    <div class="container">
        <img class="img-fluid" src="/images/oneplace.webp" alt=""/>
    </div>
    <p class="text-center display-5  text-glow text-primary fw-bold">¡Todo en un solo lugar!</p>
    <p class="text-center lead">Ofrecemos una variedad de servicios pagados. ¿Necesita de un CRM, un e-commerce o una solución personalizada? Para UpVent no hay obstáculos, ¡Lo tenemos cubierto en cualquier necesidad tecnológica!</p>
</section>
