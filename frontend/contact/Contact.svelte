<script>
    const form_id = 'mdoblozk';
</script>

<style>

</style>

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
            </fieldset>
            <input class="btn btn-primary mt-3 mb-3" type="submit" value="Enviar ✉">
        </form>
    </section>
    
</section>
