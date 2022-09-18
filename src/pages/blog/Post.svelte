<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { api_url } from '../../stores/store';
    import type { Post } from './Blog';
    
    // Tinro imports
    import { meta, type TinroRouteMeta } from 'tinro';

    // Tinro meta extractor
    const route: TinroRouteMeta = meta();

    // Post Objetct
    let post: Post[] = [];
    
    // Post API URL
    const url: string = api_url + "posts?slug=" + route.params.slug + "&_embed";

    // Fetch current post info
    onMount(async () => {
        // Post request
        const post_res: Response = await fetch(url);
        post = await post_res.json();
    });
</script>

<section class="container mt-5 mb-5">
    {#await onMount}
        <p class="lead">Cargando post...</p>
    {:then data}
        {#each post as post}
            <div class="container mx-auto text-center">
                <img width="560" height="315" class="img-fluid rounded" src="{post._embedded["wp:featuredmedia"][0].source_url}" alt="{post._embedded["wp:featuredmedia"][0].alt_text}"/>
            </div>
            <p class="display-6">{post.title.rendered}</p>
            <hr>

            <div class="container">
                <p class="text-muted small"><i>{@html post.excerpt.rendered}</i></p>
            </div>

            <article class="container mt-3 mb-2">
                {@html post.content.rendered}
            </article>
        {/each}
    {:catch error}
        <p class="text-danger">Error al obtener entrada de blog! Error: {error}</p>
    {/await}
</section>
