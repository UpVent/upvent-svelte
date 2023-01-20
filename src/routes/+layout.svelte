<script lang="ts">
 /* Top level imports */
 import "bootstrap/dist/css/bootstrap.min.css";

 /* Svelte / Project Imports */
 import { share } from "$lib/common/share";
 import { facebook_link, whatsapp_link, twitter_link, instagram_link, linkedin_link, github_link, email_link } from "$lib/common/settings";

 /* Assets */
 import logo from '$lib/assets/logo-grey.webp';

/* External imports */
import Facebook from 'svelte-bootstrap-icons/lib/Facebook.svelte';
import Twitter from 'svelte-bootstrap-icons/lib/Twitter.svelte';
import Instagram from 'svelte-bootstrap-icons/lib/Instagram.svelte';
import Linkedin from 'svelte-bootstrap-icons/lib/Linkedin.svelte';
import Github from 'svelte-bootstrap-icons/lib/Github.svelte';
import Whatsapp from 'svelte-bootstrap-icons/lib/Whatsapp.svelte';
import Envelope from 'svelte-bootstrap-icons/lib/Envelope.svelte';
import Share from 'svelte-bootstrap-icons/lib/Share.svelte';

 /* Logic */
 const fecha: number = new Date().getFullYear();
</script>

<style>
    .navbar-upvent { background-color: #FAFAFA; }
    @supports (backdrop-filter: none) {
        .navbar-upvent {
            background-color:rgba(254, 245, 254, 0.6);
            backdrop-filter: blur(20px);
        }
    }
</style>

<!-- Navbar  -->
<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-2 mb-0 sticky-md-top navbar-upvent">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 link-dark">
        <img class="mx-auto" width="148" height="51" src="{logo}" alt="UpVent Logo" loading="lazy">
    </a>

    <ul class="nav col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="fs-5 nav-link px-2 link-dark" href="/">Inicio</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/about">Nosotros</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/services">Servicios</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/store">Store</a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <a class="btn btn-primary btn-lg" href="/contact" >Contacto</a>
    </div>
</header>

<!-- Page content -->
<slot></slot>

<footer class="text-center text-muted">
    <div class="container p-4">
            <div class="row mt-5">
                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <div class="container">
                        <img class="img-fluid mb-1" width="116" height="47" src="{logo}" alt="UpVent Logo" loading="lazy"/>
                    </div>
                    <p class="link-dark">Todos los derechos reservados © - UpVent Technologies 2020 - {fecha} . Todos los logos son marcas registradas de sus respectivos dueños.</p>
                    <div class="container mt-2 mb-2">
                        <a aria-label="Facebook" class="text-muted px-2" href="{facebook_link}"><Facebook width={24} height={24}/></a>
                        <a aria-label="Twitter" class="text-muted px-2" href="{twitter_link}"><Twitter width={24} height={24}/></a>
                        <a aria-label="Instagram" class="text-muted px-2" href="{instagram_link}"><Instagram width={24} height={24}/></a>
                        <a aria-label="LinkedIn" class="text-muted px-2" href="{linkedin_link}"><Linkedin width={24} height={24}/></a>
                        <a aria-label="GitHub" class="text-muted px-2" href="{github_link}"><Github width={24} height={24}/></a>
                        <a aria-label="Whatsapp" class="text-muted px-2" href="{whatsapp_link}"><Whatsapp width={24} height={24}/></a>
                        <a aria-label="Mail" class="text-muted px-2" href="{email_link}"><Envelope width={24} height={24}/></a>
                        <button aria-label="Compartir" on:click={() => share('¡Visita UpVent en su sitio web!')} class="btn text-muted px-2"><Share/></button>
                    </div>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 link-dark">ENLACES RÁPIDOS</p>
                    <ul class="list-unstyled mb-0">
                        <li class="m-2">
                            <a href="/" class="text-decoration-none link-dark">Inicio</a>
                        </li>
                        <li class="m-2">
                            <a href="/about" class="text-decoration-none link-dark">Nosotros</a>
                        </li>
                        <li class="m-2">
                            <a href="/services" class="text-decoration-none link-dark">Servicios</a>
                        </li>
                        <li class="m-2">
                            <a href="/contact" class="text-decoration-none link-dark">Contacto</a>
                        </li>
                        <li class="m-2">
                            <a href="/store" class="text-decoration-none link-dark">PWA Store</a>
                        </li>
                    </ul>
                </div>
            </div>
    </div>

    <section class="container mt-5 mb-5 text-center-p-4 link-dark">
        <p>El código fuente de esta página se encuentra bajo la Licencia Pública General de Affero (GNU) versión 3. Excepto donde se indique lo contrario, el trabajo escrito, blogs, opiniones y parte del contenido visual se encuentra bajo la Licencia Creative Commons Atribución-SinDerivadas 3.0 No portada (CC BY-ND 3.0)</p>
    </section>

    <section class="text-center link-dark border-top p-3">
        © 2020 - {fecha} Copyright:
        UpVent Technologies -
        <a class="link-dark" href="/privacy-policy"> Política de privacidad</a> -
        <a class="link-dark" rel="nofollow" href="https://github.com/UpVent/upvent-svelte/issues/new">Reportar un problema con este sitio</a>
    </section>
</footer>
