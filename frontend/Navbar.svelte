<script>
    import Bag from 'svelte-bootstrap-icons/lib/Bag';
    import Envelope from 'svelte-bootstrap-icons/lib/Envelope';
    import House from 'svelte-bootstrap-icons/lib/House';
    import Laptop from 'svelte-bootstrap-icons/lib/Laptop';
    import Pen from 'svelte-bootstrap-icons/lib/Pen';
    import People from 'svelte-bootstrap-icons/lib/People';
    import Phone from 'svelte-bootstrap-icons/lib/Phone';


    import { Button, Modal } from 'sveltestrap';

    const logo = 'images/logo-grey.png';
    const form_id = 'moqrekzb';

    // Modal variable
    let open = false;
    const toggle = () => (open = !open);
</script>

<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-0">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
    <img class="mx-auto"  width="148" height="48" src="{logo}" alt="UpVent Logo">
    </a>

    <ul class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="nav-link px-2 text-muted" href="/">Inicio <House/></a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Blog <Pen/></a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Nosotros <People/></a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Servicios <Laptop/></a></li>
        <li><a class="nav-link px-2 text-muted" href="/">MarketCloud <Bag/></a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <Button size="lg" color="primary" on:click={toggle} >Contacto <Phone/></Button>
    </div>
</header>

<!-- Modal de contacto -->
<Modal body header="¡Gracias por contactarnos!" isOpen={open} {toggle}>
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
  <input class="btn btn-primary mt-3 mb-3" type="submit" value="Enviar ✉️">
</form>
</Modal>
