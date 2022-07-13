<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';

    // Lazy Load Import
    import Lazy from 'svelte-lazy';

    // Svelte Icons import
    import { Laptop, Book, WrenchAdjustableCircle } from 'svelte-bootstrap-icons';

    // Import upvent logo
    import logo from '../../assets/images/upvent-logo-new.webp';

    // Grid logos for technology showcasing
    let technologies = [];

    // Get API technologies
    const api_url: string = "https://wpapi.upvent.codes/wp-json/wp/v2/tecnologa/";

    // Get Projects from Wordpress API
    onMount(async () => {
        // Projects request
        const technologies_res: Response = await fetch(api_url);
        const technologies_json = await technologies_res.json();
        technologies = technologies_json;
    });
</script>

<section class="px-4 py-5 my-5 text-center">
    <Lazy height={100}>
        <img class="d-block mx-auto mb-4 rounded-circle shadow" src="{logo}" alt="Logo de UpVent circular" width="100" height="100"/>
    </Lazy>
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
    <WrenchAdjustableCircle height={48} width={48} class="text-primary"/>
    <p class="h2 fw-bold m-2">Tecnología que nos inspira.</p>
    <small class="small text-muted">Tecnologías que utilizamos</small>

    <div class="container mt-3 mb-2">
        <div class="container" id="startnowtechs">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-4">
                {#await onMount}
                    <p class="text-muted lead">Cargando las tecnologías usadas por UpVent...</p>
                {:then}
                    {#each technologies as technology}
                        <div class="col">
                            <a href="{technology.link_de_la_tecnologia}" aria-label="{technology.link_de_la_tecnologia}" target="_blank">
                                <Lazy height={50}>
                                    <img height="50" width="60" class="img-fluid" src="{technology.logo.guid}" alt="{technology.slug}"/>
                                </Lazy>
                            </a>
                        </div>
                    {/each}
                {:catch error}
                    <p class="text-danger">Error al obtener las tecnoologias usadas. Si ves este mensaje reportalo por favor con el siguiente código de error: Error No55: {error}</p>
                {/await}
            </div>
        </div>
    </div>
</section>
