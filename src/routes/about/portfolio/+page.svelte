<script lang="ts">
 /* Svelte / Internal imports */
 import SEO from "$lib/SEO.svelte";
 import type { PageData } from "./$types";

/* Logic */
 export let data: PageData;
 const records = data.projects;
</script>

<SEO/>

<section class="container text-center py-3">
    <h1>¿Nuestra experiencia?</h1>
    <h2 class="text-muted">Conoce nuestro portafolio de trabajo.</h2>
    <p class="text-muted">Aquí encontrarás todos los proyectos en los que UpVent ha participado como empresa.</p>
</section>

<section class="container py-2">
    <div class="row row-cols-2">
        {#each records as record}
            <div class="col px-2 gy-3">
                <div class="container">
                    <img height="100" class="img-fluid m-1 p-1 shadow-sm rounded-3" src="{record.img}" alt="proyecto de UpVent">
                    <p class="fw-bold">{record.nombre}</p>
                    <p class="small">{@html record.descripcion}</p>
                    <div class="d-grid">
                        <a class="btn btn-primary" href="/about/portfolio/{record.id}">Ver proyecto</a>
                    </div>
                <hr />
                </div>
            </div>
        {/each}
    </div>
</section>
