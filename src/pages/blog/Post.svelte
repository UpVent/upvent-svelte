<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    
    // Tinro imports
    import { meta, TinroRouteMeta } from 'tinro';

    // Tinro meta extractor
    const route: TinroRouteMeta = meta();

    // Post Objetct
    let post: any[] = [];

    // Post properties
    let post_title: string;
    let post_description: string;
    let post_image: string;

    // Post API URL
    const api_url: string = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?slug=" + route.params.slug + "&_embed";

    // Fetch current post info
    onMount(async () => {
        // Post request
        const post_res: Response = await fetch(api_url);
        const post_json = await post_res.json();
        post = post_json;
        post_title = post[0].title.rendered;
        post_description = post[0].description;
        post_image = post[0]._embedded["wp:featuredmedia"][0].source_url;
    });

</script>

<section class="container mt-5 mb-5">
    {#await onMount}
        <p class="lead">Cargando post...</p>
    {:then data}
        {#each post as post}
            <div class="container mx-auto text-center">
                <img width="560" height="315" class="img-fluid rounded" src="{post_image}" alt="Título del blog"/>
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
