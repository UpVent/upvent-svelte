<script>
    // Meta tags import
    import { MetaTags } from 'svelte-meta-tags';

    import IndexJumbotron from './IndexJumbotron.svelte';
    import Features from './Features.svelte';
    import Libre from './Libre.svelte';
    import Benefits from './Benefits.svelte';
    import StartNow from './StartNow.svelte';
</script>

<!-- Meta Tags -->
<MetaTags
    title="UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos."
    canonical="https://upvent.codes/"
    openGraph={{
        type: 'website',
        title: 'UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas.',
        description: 'UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.',
        image: 'https://upvent.codes/images/core-services.webp'
    }}
    twitter={{
        handle: '@UpVentMX',
        site: '@UpVentMX',
        cardType: 'summary_large_image',
        title: 'UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas.',
        description: 'UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.',
        image: 'https://upvent.codes/images/core-services.webp',
        imageAlt: 'Twitter image alt'
    }}
/>

<!-- Index jumbotron component -->
<IndexJumbotron/>

<!-- Features component -->
<Features/>

<!-- Libre Software component -->
<Libre/>

<!-- Benefits component -->
<Benefits/>

<!-- StartNow component -->
<StartNow/>

