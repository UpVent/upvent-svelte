<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Sveltestrap imports
    import {
        Button,
        Modal,
        ModalBody,
        ModalFooter,
    } from 'sveltestrap';

    // Icons import
    import XCircle from 'svelte-bootstrap-icons/lib/XCircle';

    // Modal handlers
    let open = false;
    let size;
    // Handle size from modal
    const toggleXl = () => {
        size = 'xl';
        open = !open;
    };

    // Get API blog posts
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?_embed";

    // Blog Posts array
    let posts = [];

    // Get blog posts on component mount
    onMount(async () => {
        // Posts request
        const posts_res = await fetch(api_url);
        const posts_json = await posts_res.json();
        posts = posts_json;
        console.log(posts);
    });
</script>

<section class="container">
    <h1 class="text-center">Blog</h1>
    <p class="text-center text-muted">Visite el blog de UpVent y aprenda algo nuevo junto a nosotros.</p>
    <hr>

    <div id="posts" class="container">
        {#await onMount}
            <p class="text-muted lead">Cargando publicaciones del blog...</p>
        {:then data}
            {#each posts as post, i}
                <div class="container">
                    <div class="card border-0 rounded-3 mt-5 mb-5 text-center">
                        <img class="img-fluid mx-auto rounded-3" src={post._embedded['wp:featuredmedia'][i].media_details.sizes.full.source_url} alt="{post._embedded['wp:featuredmedia'][i].alt_text}">
                        <div class="card-body border-0">
                            <a class="fs-2 fw-bold text-primary text-decoration-none" href={null} on:click={toggleXl}>{post.title.rendered}</a>
                        </div>
                        <div class="card-footer border-top">
                            <p class="text-muted small">{@html post.excerpt.rendered}</p>
                        </div>
                    </div>
                </div>

                <!-- Modal With Blog Text -->
                <Modal isOpen={open} {toggleXl} {size} scrollable>
                    <ModalBody>
                        <h2 class="display-5">{@html post.title.rendered}</h2>
                        {@html post.content.rendered}
                    </ModalBody>
                    <ModalFooter>
                        <Button color="danger" on:click={toggleXl}>Cerrar <XCircle/></Button>
                    </ModalFooter>
                </Modal>
            {/each}
        {:catch error}
            <p>Error al obtener las últimas publicaciones del blog. {error}</p>
        {/await}
    </div>
</section>
