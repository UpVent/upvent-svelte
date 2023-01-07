<script lang="ts">
 /* Component options */
 export let title = "UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas.";
 export let description = "UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos."
 export let canonical = "https://upvent.codes/"
 /* Component imports */
 import seo from '$lib/assets/seo.png';
</script>
<svelte:head>
    <!-- Primary Meta Tags -->
    <title>{title}</title>
    <meta name="title" content="{title}">
    <meta name="description" content="{description}">
    <meta name="keywords" content="agencia web, agencia web mexico, sitios web méxico, upvent, upvent.codes, upvent nube, upvent mexico, upvent software, upvent technologies, páginas web baratas, páginas web con poca inversión, servicios administrados de ti, en la nube, blog linux"/>
    <meta name="robots" content="index, translate">
    <link rel="canonical" href="{canonical}"/>

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="{canonical}">
    <meta property="og:title" content="{title}">
    <meta property="og:description" content="{description}">
    <meta property="og:image" content="{seo}">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="@UpVentMX">
    <meta name="twitter:creator" content="@UpVentMX">
    <meta name="twitter:title" content="{title}">
    <meta name="twitter:description" content="{description}">
    <meta name="twitter:image" content="{seo}">
</svelte:head>
