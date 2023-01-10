<script lang="ts">
 /* Svelte / Internal imports */
 import { fapi_url } from "$lib/common/settings";
 import { getTables } from "$lib/common/pbwrap";
 import SEO from "$lib/SEO.svelte";
 /* External imports */
 import PocketBase from "pocketbase";
 import type { Record } from "pocketbase";

 /* Logic */
 const pb: PocketBase = new PocketBase(fapi_url);
 const records: Promise<Record[]> = getTables(pb, 'proyecto_portafolio');
</script>

<SEO/>

<section class="container text-center py-3">
    <h1>¿Nuestra experiencia?</h1>
    <h2 class="text-muted">Conoce nuestro portafolio de trabajo.</h2>
    <p class="text-muted">Aquí encontrarás todos los proyectos en los que UpVent ha participado como empresa.</p>
</section>

<section class="container py-2">
    <div class="row row-cols-2">
        {#await records}
            <div class="col placeholder-glow">
                <svg class="bd-placeholder-img card-img-top" width="100%" height="180" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder" preserveAspectRatio="xMidYMid slice" focusable="false"><title>Placeholder</title><rect width="100%" height="100%" fill="#868e96"></rect></svg>                <span class="placeholder col-7"></span>
                <span class="placeholder col-4"></span>
                <span class="placeholder col-4"></span>
                <span class="placeholder col-6"></span>
                <span class="placeholder col-8"></span>
            </div>

            <div class="col placeholder-glow">
                <svg class="bd-placeholder-img card-img-top" width="100%" height="180" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Placeholder" preserveAspectRatio="xMidYMid slice" focusable="false"><title>Placeholder</title><rect width="100%" height="100%" fill="#868e96"></rect></svg>                <span class="placeholder col-7"></span>
                <span class="placeholder col-4"></span>
                <span class="placeholder col-4"></span>
                <span class="placeholder col-6"></span>
                <span class="placeholder col-8"></span>
            </div>
        {:then records}
            {#each records as record}
                <div class="col px-2 gy-3">
                    <div class="container">
                        <img height="100" class="img-fluid m-1 p-1 shadow-sm rounded-3" src="{ pb.getFileUrl(record, record.imagen_destacada) }" alt="proyecto de UpVent">
                        <p class="fw-bold">{record.nombre}</p>
                        <p class="small">{@html record.descripcion}</p>
                        <a class="btn btn-primary" href="/about/portfolio/{record.id}">Ver proyecto</a>
                    <hr />
                    </div>
                </div>
            {/each}
        {/await}
    </div>
</section>
