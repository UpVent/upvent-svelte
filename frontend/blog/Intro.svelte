<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Get API blog posts
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts";

    // Arreglo de posts de blog
    let posts = [];

    // Obtener los posts del blog de wordpress cuando se monte el componente
    onMount(async () => {
        const res = await fetch(api_url);
        const json = await res.json();
        posts = json;
        console.log(posts);
    })

</script>

<section class="container">
    <h1 class="text-center">Blog</h1>
    <p class="text-center text-muted">Visite el blog de UpVent y aprenda algo nuevo junto a nosotros.</p>
    <hr>

    <div id="posts" class="container">
        {#await onMount}
            <p class="text-muted lead">Cargando publicaciones del blog...</p>
        {:then data}
            {#each posts as post}
                <div class="container">
                    <div class="card border-0 rounded-3 shadow-sm mt-5 mb-5">
                        <div class="card-body border-0">
                            <a class="fs-2 fw-bold text-primary text-decoration-none" href="#">{post.title.rendered}</a>
                        </div>
                        <div class="card-footer border-top">
                            <p class="text-muted small">{@html post.excerpt.rendered}</p>
                            <hr>
                        </div>
                    </div>
                </div>
            {/each}

        {:catch error}
            <p>Error al obtener las últimas publicaciones del blog.</p>
        {/await}
    </div>

</section>

