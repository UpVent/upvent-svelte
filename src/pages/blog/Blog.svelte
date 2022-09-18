<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { api_url } from '../../stores/store';
    import type { Posts } from './Blog';

    // Lazy Load post images
    import Lazy from 'svelte-lazy';

    // Get API blog posts
    const url: string = api_url + "posts?_embed";

    // Blog Posts array
    let posts: Posts[] = [];

    // Get blog posts on component mount
    onMount(async () => {
        // Posts request
        const posts_res: Response = await fetch(url);
        posts = await posts_res.json();
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
            {#each posts as post}
                <div class="container">
                    <div class="card border-0 rounded-3 mt-5 mb-5 text-center">
                        <Lazy height={315}>
                            <img height="315" width="560" class="img-fluid mx-auto rounded-3" src={post._embedded['wp:featuredmedia'][0].media_details.sizes.full.source_url} alt="{post._embedded['wp:featuredmedia'][0].alt_text}">
                        </Lazy>
                        <div class="card-body border-0">
                            <a class="fs-2 text-primary text-decoration-none" href="/blog/post/{post.slug}">{post.title.rendered}</a>
                        </div>
                        <div class="card-footer border-top">
                            <p class="text-muted small">{@html post.excerpt.rendered}</p>
                        </div>
                    </div>
                </div>
            {/each}
        {:catch error}
            <p>Error al obtener las últimas publicaciones del blog. {error}</p>
        {/await}
    </div>
</section>
