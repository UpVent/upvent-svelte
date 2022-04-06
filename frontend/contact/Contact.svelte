<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // Import hcaptcha from svelte
    import "vanilla-hcaptcha";

    const form_id = 'mdoblozk';

    // Handle HCaptcha on mount
    onMount(() => {
        const contactCaptcha = document.getElementById('contactCaptcha');
        contactCaptcha.addEventListener('verified', (e) => {
        });
    });
</script>

<svelte:head>
    <!-- Primary Meta Tags -->
    <title>UpVent - Contacto</title>
    <meta name="title" content="UpVent - Contacto">
    <meta name="description" content="¿Necesita asesoramiento o enviarnos un mensaje? En la siguiente página podrá encontrar más información para comunicarse con nosotros.">

    <!-- Open Graph / Facebook -->
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://upvent.codes/services/">
    <meta property="og:title" content="UpVent - Contacto">
    <meta property="og:description" content="¿Necesita asesoramiento o enviarnos un mensaje? En la siguiente página podrá encontrar más información para comunicarse con nosotros.">
    <meta property="og:image" content="https://upvent.codes/images/core-services.webp">

    <!-- Twitter -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:site" content="@UpVentMX">
    <meta name="twitter:creator" content="@UpVentMX">
    <meta name="twitter:title" content="UpVent - Contacto">
    <meta name="twitter:description" content="¿Necesita asesoramiento o enviarnos un mensaje? En la siguiente página podrá encontrar más información para comunicarse con nosotros.">
    <meta name="twitter:image" content="https://upvent.codes/images/core-services.webp">
</svelte:head>

<section class="container">
    <h1>Contactános</h1>

    <section class="mx-auto border m-3 rounded border-1 p-3">
        <form class="form" id="fs-frm" name="simple-contact-form" accept-charset="utf-8" action="https://formspree.io/f/{form_id}" method="post">
            <fieldset id="fs-frm-inputs">
                <label class="form-label" for="full-name">Nombre Completo</label>
                <input class="form-control" type="text" name="name" id="full-name" placeholder="Su nombre aquí..." required="">
                <label class="form-label" for="email-address">Correo Electronico</label>
                <input class="form-control" type="email" name="_replyto" id="email-address" placeholder="email@dominio.com" required="">
                <label class="form-label" for="message">Mensaje</label>
                <textarea class="form-control" rows="5" name="message" id="message" placeholder="Aenean lacinia bibendum nulla sed consectetur. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor. Donec ullamcorper nulla non metus auctor fringilla nullam quis risus." required=""></textarea>
                <input class="form-control" type="hidden" name="_subject" id="email-subject" value="Contact Form Submission">
                <h-captcha
                    id="contactCaptcha"
                    site-key="44e5fa13-d8d3-4d8b-9dbf-887f16065702"
                    size="normal"
                    onVerified="onCaptchaVerified"
                ></h-captcha>
            </fieldset>
            <input class="btn btn-primary mt-3 mb-3" type="submit" value="Enviar ✉">
        </form>
    </section>
    
</section>
