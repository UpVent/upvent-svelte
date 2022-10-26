<script lang="ts">
    import { onMount } from 'svelte';
    import "vanilla-hcaptcha";

    onMount(() => {
        const contactCaptcha: HTMLElement = document.getElementById('contactCaptcha');
        contactCaptcha.addEventListener('verified', (_) => {
        });
    });

    function handle_submit(): void { (document.getElementById("fs-frm") as HTMLFormElement).reset(); }
</script>

<section class="container">
    <h1>Contactános</h1>
    <section class="mx-auto border-0 shadow-sm m-3 rounded border-1 p-3">
        <form class="form" id="fs-frm" name="simple-contact-form" action="https://formspree.io/f/mdoblozk" method="post">
            <fieldset id="fs-frm-inputs">
                <label class="form-label" for="full-name">Nombre Completo</label>
                <input class="form-control" type="text" name="name" id="full-name" placeholder="Su nombre aquí..." required>
                <label class="form-label" for="email-address">Correo Electronico</label>
                <input class="form-control" type="email" name="_replyto" id="email-address" placeholder="email@dominio.com" required>
                <label class="form-label mt-2 mb-1" for="message">Mensaje</label>
                <textarea class="form-control" rows="5" name="message" id="message" placeholder="Aenean lacinia bibendum nulla sed consectetur. Vivamus sagittis lacus vel augue laoreet rutrum faucibus dolor auctor." required></textarea>
                <input class="form-control" type="hidden" name="_subject" id="email-subject" value="Contact Form Submission">
                <h-captcha class="form-control border-0 m-2 p-2" id="contactCaptcha" site-key="44e5fa13-d8d3-4d8b-9dbf-887f16065702" size="normal" onVerified="onCaptchaVerified"></h-captcha>
            </fieldset>
            <input class="btn btn-primary mt-3 mb-3" type="submit" on:submit={handle_submit} value="Enviar">
        </form>
    </section>
</section>
