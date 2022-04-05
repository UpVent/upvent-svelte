<script>
    import Aboutjumbotron from './AboutJumbotron.svelte';
    import Tools from './Tools.svelte';
    import Reasons from './Reasons.svelte';
    import Projects from './Projects.svelte';
    import FreeTech from './FreeTech.svelte';
</script>

<svelte:head>
    <!-- Primary Meta Tags -->
    <title>UpVent - Nosotros</title>
    <meta name="title" content="UpVent - Nosotros">
    <meta name="description" content="UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas.">

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://upvent.codes/about/">
    <meta property="og:title" content="UpVent - Nosotros">
    <meta property="og:description" content="UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas.">
    <meta property="og:image" content="https://upvent.codes/images/core-about.webp">

    <!-- Twitter -->
    <meta property="twitter:card" content="summary_large_image">
    <meta property="twitter:site" content="@UpVentMX">
    <meta property="twitter:creator" content="@UpVentMX">
    <meta property="twitter:title" content="UpVent - Nosotros">
    <meta property="twitter:description" content="UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas.">
    <meta property="twitter:image" content="https://upvent.codes/images/core-about.webp">
</svelte:head>

<!-- Jumbotron About -->
<Aboutjumbotron/>
<Tools/>
<Reasons/>
<Projects/>
<FreeTech/>
