<script lang="ts">
 import SEO from '$lib/SEO.svelte';

    let nombre = "",
    correo = "",
    mensaje = "";

    function handle_submit(): void {
        const url = `https://api.whatsapp.com/send?phone=7295542482&text=Hola, mi nombre es ${nombre} quiero contactarme con ustedes con el mensaje de: ${mensaje}. Mi correo electrónico de contacto es ${correo}`;
        window.open(url);
        (document.getElementById("fs-frm") as HTMLFormElement).reset();
    }
</script>

<SEO
    title="Contacto | UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="¿Necesitas contactarnos? Envíanos un mensaje por WhatsApp y solicita la cotización de tu proyecto."
    canonical = "https://upvent.codes/contact"
/>

<section class="container">
    <h1>Contactános</h1>
    <section class="mx-auto border-0 shadow-sm-sm m-3 rounded border-1 p-3">
        <form class="form" method="post">
            <fieldset id="fs-frm-inputs">
                <label class="form-label" for="full-name">Nombre Completo</label>
                <input bind:value={nombre} class="form-control" type="text" name="ful-name" placeholder="Su nombre aquí..." required>
                <label class="form-label" for="email-address">Correo Electronico</label>
                <input bind:value={correo} class="form-control" type="email" name="email-address" placeholder="email@dominio.com" required>
                <label class="form-label mt-2 mb-1" for="message">Mensaje</label>
                <textarea class="form-control" rows="3" name="message" placeholder="Ingrese su mensaje aquí" required></textarea>
                <input bind:value={mensaje} class="form-control" type="hidden" name="_subject">
                <h-captcha class="form-control border-0 m-2 p-2" id="contactCaptcha" site-key="44e5fa13-d8d3-4d8b-9dbf-887f16065702" size="normal" onVerified="onCaptchaVerified"></h-captcha>
            </fieldset>
            <input class="btn btn-primary" type="submit" on:submit={handle_submit} value="Enviar">
        </form>
    </section>
</section>
