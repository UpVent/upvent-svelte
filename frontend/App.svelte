<script>
    // External imports
    import { Button, Modal } from 'sveltestrap';
    import { Route } from 'tinro';

    // CSR MetaTags
    import { JsonLd } from 'svelte-meta-tags';

    // Page Imports
    import Home from './home/Home.svelte';
    import Blog from './blog/Blog.svelte';
    import About from './about/About.svelte';
    import Services from './services/Services.svelte';
    // Additional Pages
    import PrivacyPolicy from './home/PrivacyPolicy.svelte';

    // Component imports
    import Footer from './Footer.svelte';

    // Icon imports
    import House from 'svelte-bootstrap-icons/lib/House';
    import Laptop from 'svelte-bootstrap-icons/lib/Laptop';
    import Pen from 'svelte-bootstrap-icons/lib/Pen';
    import People from 'svelte-bootstrap-icons/lib/People';
    import Phone from 'svelte-bootstrap-icons/lib/Phone';

    const logo = 'images/logo-grey.png';
    const form_id = 'moqrekzb';

    // Modal variable
    let open = false;
    const toggle = () => (open = !open);
</script>

<style>
    @supports (backdrop-filter: none) {
	    .navbar-upvent {
            backdrop-filter: saturate(100%) blur(100px);
            -webkit-backdrop-filter: saturate(100%) blur(100px);
	    }
    }

    @supports not (backdrop-filter: none) {
        .background {
            background-color: #FAFAFA;
        }
    }
</style>

<!-- Meta Tags -->
<JsonLd
    schema = {{
    '@type': 'Organization',
    name: 'UpVent Technologies',
    url: 'https://upvent.codes',
    address: 'Guanajuato, Guanajuato',
    sameAs: ["https://facebook.com/UpVentMX","https://twitter.com/UpVentMX","https://www.linkedin.com/company/upvent-technologies/","https://www.instagram.com/UpVentMX/"]
    }}
/>


<!-- Navbar component -->
<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-0 sticky-md-top navbar-upvent">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
        <img class="mx-auto img-fluid" width="148" height="51" src="{logo}" alt="UpVent Logo">
    </a>

    <ul class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="fs-5 nav-link px-2 text-muted" href="/">Inicio <House/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/blog">Blog <Pen/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/about">Nosotros <People/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/services">Servicios <Laptop/></a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <Button size="lg" color="primary" on:click={toggle} >Contacto <Phone/></Button>
    </div>
</header>

<!-- Modal de contacto -->
<Modal id="contactform" body header="¡Gracias por contactarnos!" isOpen={open} {toggle}>
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

<!-- Home Page -->
<Route path="/"><Home/></Route>
<Route path="/blog"><Blog/></Route>
<Route path="/about"><About/></Route>
<Route path="/services"><Services/></Route>

<!-- Footer links -->
<Route path="/privacy-policy"><PrivacyPolicy/></Route>

<!-- Footer component -->
<Footer/>
