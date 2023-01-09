<script lang="ts">
 import { fapi_url } from "$lib/common/settings";
 import { getTables } from "$lib/common/pbwrap";
 import SEO from "$lib/SEO.svelte";

 import PocketBase from "pocketbase";
 import type { Record } from "pocketbase";

 const pb: PocketBase = new PocketBase(fapi_url);
 const records: Promise<Record[]> = getTables(pb, 'politica_privacidad');
 function format_date(date: string): string { return new Date(date).toLocaleDateString('es-MX'); }
</script>

<SEO
    title="Política de privacidad | UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="Conoce la política de privacidad de nuestros servicios en la web."
    canonical = "https://upvent.codes/privacy-policy"
/>

<section class="container">
    {#await records} <p>Cargando política de privacidad...</p>{:then data}
    {#each data as record}
        <h1>{record.titulo}</h1>
        <span class="badge rounded-pill text-bg-primary">Última revisión: { format_date(record.ultima_revision) }</span>
        <hr>
        <section class="container"> {@html record.contenido } </section>
    {/each}
    {/await}
</section>
