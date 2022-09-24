<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { fapi_url, api_user, api_user_pass } from '../../config';
    import { truncate_str } from '../../stores/store';

    // Lazy Load post images
    import Lazy from 'svelte-lazy';

    // Database imports
    import PocketBase from 'pocketbase';
    import type { Record } from 'pocketbase';

    // Database usage
    const client: PocketBase = new PocketBase(fapi_url);

    // Grid logos for blog showcasing
    let records: Record[] = [];

    // Get blog posts on component mount
    onMount(async () => {
        const user_auth_data = await client.users.authViaEmail(api_user, api_user_pass);
        records = await client.records.getFullList('blog_post', 200, { sort: '-created' });
        records.forEach(e => delete e.contenido) 
        client.authStore.clear();
    });
</script>

<section class="container">
    <h1 class="text-center">El blog de UpVent Technologies</h1>
    <p class="text-center text-muted">Visite el blog de UpVent y aprenda algo nuevo junto a nosotros.</p>
    <hr>
    <div id="posts" class="container">
        {#await onMount}
            <p class="text-muted lead">Cargando publicaciones del blog...</p>
        {:then}
            {#each records as record}
                <div class="container">
                    <div class="card border-0 rounded-3 mt-5 mb-5 text-center">
                        <Lazy height={315}>
                            <img height="315" width="560" class="img-fluid mx-auto rounded-3" src={client.records.getFileUrl(record, record.imagen_destacada)} alt="Imágen de blog">
                        </Lazy>
                        <div class="card-body border-0">
                            <a class="fs-2 text-primary text-decoration-none" href="/blog/post/{record.id}">{record.titulo}</a>
                        </div>
                        <div class="card-footer border-top">
                            <p class="text-muted small">{truncate_str(record.extracto, 150)}</p>
                        </div>
                    </div>
                </div>
            {/each}
        {:catch error}
            <p>Error al obtener las últimas publicaciones del blog. {error}</p>
        {/await}
    </div>
</section>
