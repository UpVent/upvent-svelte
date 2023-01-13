<script lang="ts">
 import type { PageData } from "./$types";
 import SEO from "$lib/SEO.svelte";

 export let data: PageData;
 const record = data.records;
 function format_date(date: string): string { return new Date(date).toLocaleDateString('es-MX'); }
</script>

<SEO
    title="Política de privacidad | UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="Conoce la política de privacidad de nuestros servicios en la web."
    canonical = "https://upvent.codes/privacy-policy"
/>

<section class="container">
    <h1>{record.titulo}</h1>
    <span class="badge rounded-pill text-bg-primary">Última revisión: { format_date(record.ultima_revision) }</span>
    <hr>
    <section class="container"> {@html record.contenido } </section>
</section>
