<script>
    // Meta tags import
    import { MetaTags } from 'svelte-meta-tags';

    import ServiceJumbotron from './ServiceJumbotron.svelte';
    import Cloud from './Cloud.svelte';
    import Plans from './Plans.svelte';
    import Webpages from './Webpages.svelte';
    import FServices from './FServices.svelte';
</script>

<!-- Meta Tags -->
<MetaTags
    title="UpVent - Servicios"
    description="Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa."
    canonical="https://upvent.codes/services"
    openGraph={{
        type: 'website',
        title: 'UpVent - Servicios',
        description: 'Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa.',
        image: 'https://upvent.codes/images/core-services.webp'
    }}
    twitter={{
        handle: '@UpVentMX',
        site: '@UpVentMX',
        cardType: 'summary_large_image',
        title: 'UpVent - Servicios',
        description: 'Conozca la amplia variedad de servicios que UpVent puede ofrecerle a usted y a su empresa.',
        image: 'https://upvent.codes/images/core-services.webp',
        imageAlt: 'Twitter image alt'
    }}
/>

<ServiceJumbotron/>
<Cloud/>
<Plans/>
<Webpages/>
<FServices/>

