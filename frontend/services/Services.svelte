<svelte:options immutable={true}/>
<script>
    import ServiceJumbotron from './ServiceJumbotron.svelte';
    import Cloud from './Cloud.svelte';
    import Plans from './Plans.svelte';
    import Webpages from './Webpages.svelte';
    import FServices from './FServices.svelte';
</script>

<svelte:head>
    <!-- Primary Meta Tags -->
    <title>UpVent - Servicios</title>
    <meta name="title" content="UpVent - Servicios">
    <meta name="description" content="Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa.">

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://upvent.codes/services/">
    <meta property="og:title" content="UpVent - Servicios">
    <meta property="og:description" content="Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa.">
    <meta property="og:image" content="https://upvent.codes/images/core-services.webp">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="@UpVentMX">
    <meta name="twitter:creator" content="@UpVentMX">
    <meta name="twitter:title" content="UpVent - Servicios">
    <meta name="twitter:description" content="Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa.">
    <meta name="twitter:image" content="https://upvent.codes/images/core-services.webp">
</svelte:head>

<ServiceJumbotron/>
<Cloud/>
<Plans/>
<Webpages/>
<FServices/>
