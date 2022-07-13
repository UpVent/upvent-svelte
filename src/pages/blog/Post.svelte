<svelte:options immutable={true}/>
<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Svelte icons
    import { Facebook, Twitter, Whatsapp, Envelope } from 'svelte-bootstrap-icons';
    
    // Tinro imports
    import { meta } from 'tinro';

    // Tinro meta extractor
    const route = meta();

    // Post Objetct
    let post = [];

    // Post properties
    let post_title;
    let post_description;
    let post_image;

    // Post API URL
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?slug=" + route.params.slug + "&_embed";

    // Fetch current post info
    onMount(async () => {
        // Post request
        const post_res = await fetch(api_url);
        const post_json = await post_res.json();
        post = post_json;
        post_title = post[0].title.rendered;
        post_description = post[0].description;
        post_image = post[0]._embedded["wp:featuredmedia"][0].source_url;
    });

</script>

<svelte:head>
    <!-- Primary Meta Tags -->
    <title>UpVent - {post_title}.</title>
    <meta name="title" content="{post_title}">
    <meta name="description" content="{post_description}">

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="twitter:site" content="@UpVentMX">
    <meta property="twitter:creator" content="@UpVentMX">
    <meta property="og:title" content="{post_title}">
    <meta property="og:description" content="{post_description}">
    <meta property="og:image" content="{post_image}">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:url" content="@UpVentMX">
    <meta name="twitter:title" content="{post_title}">
    <meta name="twitter:description" content="{post_description}">
    <meta name="twitter:image" content="{post_image}">
</svelte:head>

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
