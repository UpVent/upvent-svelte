<script lang="ts">
    // Svelte imports
    import { whatsapp_link, facebook_link, twitter_link, instagram_link, linkedin_link, github_link, ascard_link, email_link } from '../config';

    // Social Media Icons
    import Facebook from 'svelte-bootstrap-icons/lib/Facebook.svelte';
    import Twitter from 'svelte-bootstrap-icons/lib/Twitter.svelte';
    import Instagram from 'svelte-bootstrap-icons/lib/Instagram.svelte';
    import Linkedin from 'svelte-bootstrap-icons/lib/Linkedin.svelte';
    import Github from 'svelte-bootstrap-icons/lib/Github.svelte';
    import PostcardHeart from 'svelte-bootstrap-icons/lib/PostcardHeart.svelte';
    import Whatsapp from 'svelte-bootstrap-icons/lib/Whatsapp.svelte';
    import Envelope from 'svelte-bootstrap-icons/lib/Envelope.svelte';
    import Share from 'svelte-bootstrap-icons/lib/Share.svelte';

    // Import upvent logo for footer
    import logo from '../assets/images/logo-grey.webp';

    async function shar(): Promise<void> {
        if (navigator.canShare) { navigator.share({ title: 'UpVent Technologies', text: '¡Visita UpVent!', url: 'https://upvent.codes/' }); }
    }
</script>

<style>.bg-light-gray { background-color: #f9fafb; }</style>

<footer class="text-center text-muted bg-light-gray">
    <div class="container p-4">
            <div class="row mt-5">
                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <div class="container">
                        <img class="img-fluid mb-1" width="116" height="47" src="{logo}" alt="UpVent Logo" loading="lazy"/>
                    </div>
                    <p class="link-dark">Todos los derechos reservados © - UpVent Technologies 2020 - {new Date().getFullYear()} . Todos los logos son marcas registradas de sus respectivos dueños.</p>
                    <div class="container mt-2 mb-2">
                        <a aria-label="Facebook" class="text-muted px-2" href="{facebook_link}"><Facebook width={24} height={24}/></a>
                        <a aria-label="Twitter" class="text-muted px-2" href="{twitter_link}"><Twitter width={24} height={24}/></a>
                        <a aria-label="Instagram" class="text-muted px-2" href="{instagram_link}"><Instagram width={24} height={24}/></a>
                        <a aria-label="LinkedIn" class="text-muted px-2" href="{linkedin_link}"><Linkedin width={24} height={24}/></a>
                        <a aria-label="GitHub" class="text-muted px-2" href="{github_link}"><Github width={24} height={24}/></a>
                        <a aria-label="As-Card" class="text-muted px-2" href="{ascard_link}"><PostcardHeart width={24} height={24}/></a>
                        <a aria-label="Whatsapp" class="text-muted px-2" href="{whatsapp_link}"><Whatsapp width={24} height={24}/></a>
                        <a aria-label="Mail" class="text-muted px-2" href="{email_link}"><Envelope width={24} height={24}/></a>
                        <button aria-label="Compartir" on:click={shar} class="btn text-muted px-2"><Share/></button>
                    </div>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 text-uppercase link-dark">Enlaces Rápidos</p>
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
                            <a href="/contacto" class="text-decoration-none link-dark">Contacto</a>
                        </li>
                        <li class="m-2">
                            <a href="/pwa" class="text-decoration-none link-dark">PWA Store</a>
                        </li>
                    </ul>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 text-uppercase link-dark">Enlaces Importantes</p>
                    <ul class="list-unstyled mb-0">
                        <li class="m-2">
                            <a rel="nofollow" href="https://github.com/UpVent" class="text-decoration-none link-dark">Repositorio de Código Libre en GitHub</a>
                        </li>
                        <li class="m-2">
                            <a rel="nofollow" href="/privacy-policy" class="text-decoration-none link-dark">Términos y Condiciones</a>
                        </li>
                        <li class="m-2">
                            <a rel="nofollow" href="/" class="text-decoration-none link-dark">Recompensas <span class="badge rounded-pill text-bg-warning">¡Próximamente!</span> </a>
                        </li>
                    </ul>
                </div>
            </div>
    </div>

    <section class="container mt-5 mb-5 text-center-p-4 link-dark small">
        <p>El código fuente de esta página se encuentra bajo la <a class="text-decoration-none link-dark" href="https://www.gnu.org/licenses/agpl-3.0.html">Licencia Pública General de Affero (GNU) versión 3</a>. Excepto donde se indique lo contrario, el trabajo escrito, blogs, opiniones y parte del contenido visual se encuentra bajo la <a class="text-decoration-none link-dark" href="https://creativecommons.org/licenses/by-nd/3.0/deed.es">Licencia Creative Commons Atribución-SinDerivadas 3.0 No portada (CC BY-ND 3.0)</a></p>
    </section>

    <section class="text-center link-dark border-top p-3">
        © 2020 - {new Date().getFullYear()} Copyright:
        <a class="link-dark" href="https://upvent.codes/">UpVent Technologies</a> -
        <a class="link-dark" href="/privacy-policy"> Política de privacidad</a> -
        <a class="link-dark" rel="nofollow" href="https://github.com/UpVent/upvent-svelte/issues/new">Reportar un problema con este sitio</a>
    </section>
</footer>
