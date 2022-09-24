<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { fapi_url, api_user, api_user_pass } from '../../config';
    
    // Tinro imports
    import { meta, type TinroRouteMeta } from 'tinro';

    // Database imports
    import PocketBase from 'pocketbase';

    // Database usage
    const client: PocketBase = new PocketBase(fapi_url);

    // Grid logos for blog showcasing
    let record: any = {};
    let record_img: string;
    const route: TinroRouteMeta = meta();

    // Fetch current post info
    onMount(async () => {
        client.users.authViaEmail(api_user, api_user_pass);
        record = await client.records.getOne('blog_post', route.params.id);
        record_img = client.records.getFileUrl(record, record.imagen_destacada);
        client.authStore.clear();
    });
</script>

<section class="container mt-5 mb-5">
    {#await onMount}
        <p class="lead">Cargando post...</p>
    {:then}
        <div class="container mx-auto text-center">
            <img width="560" height="315" class="img-fluid rounded" alt="Imágen correspondiente al logo" src="{record_img}"/>
        </div>
        <p class="display-6">{record.titulo}</p><hr>
        <div class="container"><p class="text-muted small"><i>{record.extracto}</i></p></div>
        <article class="container mt-3 mb-2">{@html record.contenido}</article>
    {:catch error}
        <p class="text-danger">Error al obtener entrada de blog! Error: {error}</p>
    {/await}
</section>
