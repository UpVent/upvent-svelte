<script lang="ts">
 /* Svelte / Internal imports */
 import type { PageData } from "./$types";
 import SEO from "$lib/SEO.svelte";

 /* External imports */
 import FileCode from "svelte-bootstrap-icons/lib/FileCode.svelte";
 import Lock from "svelte-bootstrap-icons/lib/Lock.svelte";
 import Download from "svelte-bootstrap-icons/lib/Download.svelte";
 import EmojiFrown from "svelte-bootstrap-icons/lib/EmojiFrown.svelte";

/* Logic */
 export let data: PageData;
 const record = data.records;
</script>

<SEO
    title="Portafolio | UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="Conoce a detalle la escencia de UpVent. ¿Qué esperas para llevar tu empresa a las nubes?."
    canonical="https://upvent.codes/about/portfolio"
/>

<section class="container">
    <img class="img-fluid rounded-4 shadow" src="{record.imagen_destacada}" alt="{record.imagen_destacada}" loading="lazy"/>
    <div class="container my-2">
        <h1 class="fw-bold">{record.nombre}</h1>
        {#if record.sigue_activo}
            <span class="text-success fs-5">◉ Activo</span>
	    {:else}
            <span class="text-danger fs-5">◉ Inactivo</span>
        {/if}
        <hr>
        <p>{@html record.descripcion}</p>
        <hr>
        <h2>Detalles del proyecto:</h2>
        {#if record.es_libre}
	        <span class="fs-5 badge text-bg-primary">Libre y público <FileCode height={24} width={24}/> </span>
            <a rel="noreferrer" target="_blank" href="{record.enlace_de_descarga}">
	            <span class="fs-5 badge text-dark bg-primary-subtle">Descargar <Download height={24} width={24}/></span>
            </a>
        {:else}
	        <span class="fs-5 badge text-bg-danger">Libre y de uso privado <Lock height={24} width={24}/> </span>
	        <span class="fs-5 badge text-dark bg-danger-subtle">
                Código fuente no disponible <EmojiFrown height={24} width={24}/>
            </span>
        {/if}
    </div>
</section>
