<script lang="ts">
    // Svelte imports
    import { onMount, onDestroy } from 'svelte'; 
    import { fapi_url, api_user, api_user_pass } from '../../config';
    import { api_result } from '../../stores/store';

    // Svelte Icons import
    import WrenchAdjustableCircle from 'svelte-bootstrap-icons/lib/WrenchAdjustableCircle.svelte';

    // Import upvent logo
    import logo from '../../assets/images/upvent-main.webp';

    // Database imports
    import PocketBase from 'pocketbase';
    
    // Database usage
    const client: PocketBase = new PocketBase(fapi_url);

    onMount(async () => {
        client.users.authViaEmail(api_user, api_user_pass);
        $api_result = await client.records.getFullList('tecnologias', 4, {
            sort: '-created',
        });
        client.authStore.clear();
    });

    onDestroy(() => { $api_result.length = 0; });
</script>

<section class="px-4 py-5 my-5 text-center">
        <img class="d-block mx-auto mb-4 rounded-circle shadow" src="{logo}" alt="Logo de UpVent circular" width="100" height="100" loading="lazy"/>
    <p class="h1 display-5 fw-bold">Comenzar a trabajar con <span class="h1 display-5 fw-bold text-primary">UpVent.</span></p>
    <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">Potencie su negocio ahora mismo, mire nuestro portafolio de trabajo o visite nuestra tienda de servicios para comenzar hoy mismo.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="/pwa" class="btn btn-primary btn-lg px-4 me-sm-3">Comprar Ahora</a>
            <a href="/about" class="btn btn-outline-primary fw-bold btn-lg px-4">Ver Portafolio</a>
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
                {#each $api_result as record}
                    <div class="col">
                        <a rel="noopener noreferrer" href="{record.enlace}" aria-label="{record.enlace}" target="_blank">
                            <img height="50" width="60" class="img-fluid" src="{ client.records.getFileUrl(record, record.imagen_destacada) }" alt="{record.nombre}" loading="lazy"/>
                        </a>
                    </div>
                {/each}
            </div>
        </div>
    </div>
</section>
