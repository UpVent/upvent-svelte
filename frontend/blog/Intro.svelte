<script>

    // External imports
    import Book from 'svelte-bootstrap-icons/lib/Book';

    // Get API blog posts
    const api_url = 'http://127.0.0.1:8000/api/posts';
    // Get api info
    const fetchPosts = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()
</script>

<style>
    
</style>

<section class="container">
    <h1 class="text-center">Blog</h1>
    <p class="text-center text-muted">Visite el blog de UpVent y aprenda algo nuevo junto a nosotros.</p>
    <hr>

    <div id="posts" class="container">
        {#await fetchPosts}
            <p class="text-muted lead">Cargando publicaciones del blog...</p>
        {:then data}

            {#each data.result as post}
                <div class="container">
                    <div class="card border-0 rounded-3 shadow-sm mt-5 mb-5">
                        <div class="card-body border-0">
                            <a class="fs-2 fw-bold text-primary text-decoration-none" href="#">{post.title}</a>
                        </div>
                        <div class="card-footer border-0">
                            <p class="text-muted small">{post.description}</p>
                            <hr>
                            <span class="text-center text-wrap text-break align-middle badge rounded-pill bg-primary shadow-sm"><Book/> Tiempo de lectura: x minutos.</span>
                            <span class="badge rounded-pill bg-secondary shadow-sm">Categoría: {post.category}</span>
                        </div>
                    </div>
                </div>
            {/each}

        {:catch error}
            <p>Error al obtener las últimas publicaciones del blog.</p>
        {/await}
    </div>

</section>

