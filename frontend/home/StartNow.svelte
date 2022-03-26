<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Svelte Icons import
    import Laptop from 'svelte-bootstrap-icons/lib/Laptop';
    import Book from 'svelte-bootstrap-icons/lib/Book';

    // UpVent Logo
    let logo = 'images/upvent-logo-new.webp';

    // Grid logos for technology showcasing
    let technologies = [];

    // Get API technologies
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/tecnologa/";

    // Get Projects from Wordpress API
    onMount(async () => {
        // Projects request
        const technologies_res = await fetch(api_url);
        const technologies_json = await technologies_res.json();
        technologies = technologies_json;
    });
</script>

<section class="px-4 py-5 my-5 text-center">
    <img class="d-block mx-auto mb-4 rounded-circle shadow" src={logo} alt="Logo de UpVent circular" width="100" height="100"/>
    <p class="h1 display-5 fw-bold">Comenzar a trabajar con <span class="h1 display-5 fw-bold text-primary text-glow">UpVent.</span></p>
    <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">Visite nuestra página de servicios para explorar nuestras ofertas prediseñadas para su empresa o contacte directo con ventas.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="/services" class="btn btn-primary btn-lg px-4 me-sm-3">Ir a Servicios <Laptop/></a>
            <a href="/blog" class="btn btn-outline-primary btn-lg px-4">Leer nuestro blog <Book/></a>
        </div>
    </div>
</section>

<section class="text-center">
    <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" fill="currentColor" class="bi bi-wrench-adjustable-circle text-primary" viewBox="0 0 16 16">
        <path d="M12.496 8a4.491 4.491 0 0 1-1.703 3.526L9.497 8.5l2.959-1.11c.027.2.04.403.04.61Z"/>
        <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0Zm-1 0a7 7 0 1 0-13.202 3.249l1.988-1.657a4.5 4.5 0 0 1 7.537-4.623L7.497 6.5l1 2.5 1.333 3.11c-.56.251-1.18.39-1.833.39a4.49 4.49 0 0 1-1.592-.29L4.747 14.2A7 7 0 0 0 15 8Zm-8.295.139a.25.25 0 0 0-.288-.376l-1.5.5.159.474.808-.27-.595.894a.25.25 0 0 0 .287.376l.808-.27-.595.894a.25.25 0 0 0 .287.376l1.5-.5-.159-.474-.808.27.596-.894a.25.25 0 0 0-.288-.376l-.808.27.596-.894Z"/>
    </svg>
    <p class="h2 fw-bold m-2">Tecnología que nos inspira.</p>
    <small class="small text-muted">Tecnologías que utilizamos</small>

    <div class="container mt-3 mb-2">
        <div class="container" id="startnowtechs">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-4">
                {#await onMount}
                    <p class="text-muted lead">Cargando las tecnologías usadas por UpVent...</p>
                {:then data}
                    {#each technologies as technology}
                        <div class="col"><a href="{technology.link_de_la_tecnologia}" target="_blank"><img height="50" width="60" class="img-fluid" src="{technology.logo.guid}" alt="{technology.slug}"></a></div>
                    {/each}
                {:catch error}
                    <p class="text-danger">Error al obtener las tecnoologias usadas. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}</p>
                {/await}
            </div>
        </div>
    </div>

</section>
