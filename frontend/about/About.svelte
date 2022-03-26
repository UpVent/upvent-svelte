<script>
    // Meta tags import
    import { MetaTags } from 'svelte-meta-tags';

    import Aboutjumbotron from './AboutJumbotron.svelte';
    import Tools from './Tools.svelte';
    import Reasons from './Reasons.svelte';
    import Projects from './Projects.svelte';
    import FreeTech from './FreeTech.svelte';
</script>

<!-- Meta Tags -->
<MetaTags
    title="UpVent - Nosotros"
    description="UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas."
    canonical="https://upvent.codes/about"
    openGraph={{
        type: 'website',
        title: 'UpVent - Nosotros',
        description: 'UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas.',
        image: 'https://upvent.codes/images/core-about.webp'
    }}
    twitter={{
        handle: '@UpVentMX',
        site: '@UpVentMX',
        cardType: 'summary_large_image',
        title: 'UpVent - Nosotros',
        description: 'UpVent es una empresa desarrolladora de software con el modelo Libre, enfocada a la digitalización de empresas Mexicanas.',
        image: 'https://upvent.codes/images/core-about.webp',
        imageAlt: 'Twitter image alt'
    }}
/>

<!-- Jumbotron About -->
<Aboutjumbotron/>
<Tools/>
<Reasons/>
<Projects/>
<FreeTech/>
