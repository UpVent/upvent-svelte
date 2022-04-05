<svelte:options immutable={true}/>
<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Svelte icons
    import Facebook from 'svelte-bootstrap-icons/lib/Facebook';
    import Twitter from 'svelte-bootstrap-icons/lib/Twitter';
    import Whatsapp from 'svelte-bootstrap-icons/lib/Whatsapp';
    import Envelope from 'svelte-bootstrap-icons/lib/Envelope';

    // Tinro imports
    import { meta } from 'tinro';

    // Tinro meta extractor
    const route = meta();

    // Post Objetct
    let post = [];

    // Post properties
    let post_title;
    let post_description;

    // Post API URL
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?slug=" + route.params.slug;

    // Fetch current post info
    onMount(async () => {
        // Post request
        const post_res = await fetch(api_url);
        const post_json = await post_res.json();
        post = post_json;
        post_title = post[0].title.rendered;
        post_description = post[0].description;
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
    <meta property="og:image" content="https://upvent.codes/images/upvent-logo-new.webp">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:url" content="@UpVentMX">
    <meta name="twitter:title" content="{post_title}">
    <meta name="twitter:description" content="{post_description}">
    <meta name="twitter:image" content="https://upvent.codes/images/upvent-logo-new.webp">
</svelte:head>

<section class="container mt-5 mb-5">
    {#await onMount}
        <p class="lead">Cargando post...</p>
    {:then data}
        {#each post as post}
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
