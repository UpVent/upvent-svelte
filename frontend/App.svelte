<script>
    import { MetaTags } from 'svelte-meta-tags';

    import Navbar from './Navbar.svelte';
    import IndexJumbotron from './IndexJumbotron.svelte';
    import Footer from './Footer.svelte';

    export let name;
</script>

<svelte:head>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
</svelte:head>

<MetaTags
    title = "UpVent - Soluciones en la nube para tu negocio. Low Cost, siempre listas."
    description = "UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos."
    canonical = "https://upvent.codes/"

    openGraph = {{
        url: 'https://upvent.codes/',
        title: "UpVent - Soluciones en la nube para tu negocio. Low Cost, siempre listas.",
        description: "UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.",
         images: [
            {
                url: 'https://www.example.ie/og-image-01.jpg',
                width: 800,
                height: 600,
                alt: 'Og Image Alt'
            },
            {
                url: 'https://www.example.ie/og-image-02.jpg',
                width: 900,
                height: 800,
                alt: 'Og Image Alt Second'
            },
            { url: 'https://www.example.ie/og-image-03.jpg' },
            { url: 'https://www.example.ie/og-image-04.jpg' }
        ],
        site_name: 'SiteName'
    }}

/>

<Navbar/>
<IndexJumbotron/>
<Footer/>
