<svelte:options immutable={true}/>
<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';

    // Lazy Load post images
    import Lazy from 'svelte-lazy';

    // Get API blog posts
    const api_url: string = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?_embed";

    // Blog Posts array
    let posts = [];

    // Get blog posts on component mount
    onMount(async () => {
        // Posts request
        const posts_res = await fetch(api_url);
        const posts_json = await posts_res.json();
        posts = posts_json;
    });
</script>

<svelte:head>
    <!-- Primary Meta Tags -->
    <title>UpVent - El Blog Oficial de UpVent Technologies.</title>
    <meta name="title" content="UpVent - El Blog Oficial de UpVent Technologies.">
    <meta name="description" content="El blog oficial de UpVent. Aquí podras encontrar tutoriales y artículos de todo tipo para mantenerte siempre informado.">

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://upvent.codes/blog">
    <meta property="og:title" content="UpVent - El Blog Oficial de UpVent Technologies.">
    <meta property="og:description" content="El blog oficial de UpVent. Aquí podras encontrar tutoriales y artículos de todo tipo para mantenerte siempre informado.">
    <meta property="og:image" content="https://upvent.codes/images/upvent-logo-new.webp">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="@UpVentMX">
    <meta name="twitter:creator" content="@UpVentMX">
    <meta name="twitter:title" content="UpVent - El Blog Oficial de UpVent Technologies.">
    <meta name="twitter:description" content="El blog oficial de UpVent. Aquí podras encontrar tutoriales y artículos de todo tipo para mantenerte siempre informado.">
    <meta name="twitter:image" content="https://upvent.codes/images/upvent-logo-new.webp">
</svelte:head>


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
                            <a class="fs-2 fw-bold text-primary text-decoration-none" href="/blog/post/{post.slug}">{post.title.rendered}</a>
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
