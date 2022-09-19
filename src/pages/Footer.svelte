<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { uptime_api } from '../config';

    // Social Media Icons
    import { Facebook, Twitter, Instagram, Linkedin, Github, PostcardHeart, CircleFill } from 'svelte-bootstrap-icons';

    // Lazy loader
    import Lazy from 'svelte-lazy';

    // Uptime robot variables
    let monitors: any = [];

    // Import upvent logo for footer
    import logo from '../assets/images/logo-grey.webp';

    // Get uptimerobot monitor status
    onMount(async () => {
        const monitors_res: Response = await fetch(uptime_api, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json'},
            mode: 'cors'
        });
        const monitors_json = await monitors_res.json();
        monitors = monitors_json;
    });
</script>

<style>
    .bg-light-gray { background-color: #f9fafb; }
</style>

<footer class="text-center text-muted bg-light-gray">
    <div class="container p-4">
        <section class="mt-5">
            <div class="row">
                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <div class="container">
                        <Lazy height={47}>
                            <img class="img-fluid mb-1" width="116" height="47" src="{logo}" alt="UpVent Logo"/>
                        </Lazy>
                    </div>
                    <p>Todos los derechos reservados © - UpVent Technologies 2020 - {new Date().getFullYear()} . Todos los logos son marcas registradas de sus respectivos dueños.</p>
                    <div class="container mt-2 mb-2">
                        <a aria-label="Facebook" class="text-muted px-2" href="https://facebook.com/UpVentMX"><Facebook width={24} height={24}/></a>
                        <a aria-label="Twitter" class="text-muted px-2" href="https://twitter.com/UpVentMX"><Twitter width={24} height={24}/></a>
                        <a aria-label="Instagram" class="text-muted px-2" href="https://instagram.com/UpVentMX"><Instagram width={24} height={24}/></a>
                        <a aria-label="LinkedIn" class="text-muted px-2" href="https://www.linkedin.com/company/upvent-technologies/"><Linkedin width={24} height={24}/></a>
                        <a aria-label="GitHub" class="text-muted px-2" href="https://github.com/UpVent"><Github width={24} height={24}/></a>
                        <a aria-label="As-Card" class="text-muted px-2" href="https://me.as-card.com/accounts/profile/UpVentMX/"><PostcardHeart width={24} height={24}/></a>
                    </div>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 text-uppercase">Enlaces Rápidos</p>
                    <ul class="list-unstyled mb-0">
                        <li class="m-2">
                            <a href="/" class="text-decoration-none text-muted">Inicio</a>
                        </li>
                        <li class="m-2">
                            <a href="/blog" class="text-decoration-none text-muted">Blog</a>
                        </li>
                        <li class="m-2">
                            <a href="/about" class="text-decoration-none text-muted">Nosotros</a>
                        </li>
                        <li class="m-2">
                            <a href="/services" class="text-decoration-none text-muted">Servicios</a>
                        </li>
                        <li class="m-2">
                            <a href="/contacto" class="text-decoration-none text-muted">Contacto</a>
                        </li>
                    </ul>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 text-uppercase">Enlaces Importantes</p>
                    <ul class="list-unstyled mb-0">
                        <li class="m-2">
                            <a rel="nofollow" href="https://github.com/UpVent" class="text-decoration-none text-muted">Repositorio de Código Libre en GitHub <i class="bi bi-github"></i></a>
                        </li>
                    </ul>
                </div>
            </div>
        </section>
    </div>

    <section class="container mt-5 mb-5 text-center-p-4">
        <p>El <a href="https://github.com/UpVent/upvent-svelte">código fuente</a> de esta página se encuentra bajo la <a href="https://www.gnu.org/licenses/agpl-3.0.html">Licencia Pública General de Affero (GNU) versión 3</a>. Excepto donde se indique lo <a href="https://creativecommons.org/policies#license">contrario</a>, el trabajo escrito, blogs, opiniones y parte del contenido visual se encuentra bajo la <a href="https://creativecommons.org/licenses/by-nd/3.0/deed.es">Licencia Creative Commons Atribución-SinDerivadas 3.0 No portada (CC BY-ND 3.0)</a></p>
    </section>

    <section class="container">
        {#await onMount}
            <p class="text-muted">Cargando estatus de UpVent...</p>
        {:then}
            {#if monitors.stat == 'ok' }
                <p class="text-muted"> Estatus de UpVent: <CircleFill class="text-success"/><a href="https://stats.uptimerobot.com/qXywYt1lg9"> Todos los sistemas funcionales</a></p>
            {:else}
                <p class="text-muted"> Estatus de UpVent: <CircleFill class="text-danger"/><a href="https://stats.uptimerobot.com/qXywYt1lg9"> Rendimiento degradado</a></p>
            {/if}
        {:catch _}
            <p class="text-danger">Error al obtener el estátus de UpVent. Si ves esta pantalla, reporta el incidente con <a href="mailto:contacto@upvent.codes">el equipo de soporte de UpVent</a></p>
        {/await}
    </section>

    <section class="text-center border-top p-3">
        © 2020 - {new Date().getFullYear()} Copyright:
        <a href="https://upvent.codes/">UpVent Technologies</a> -
        <a href="/privacy-policy"> Política de privacidad</a> -
        <a href="https://github.com/UpVent/upvent-svelte/issues/new">Reportar un problema con este sitio</a>
    </section>
</footer>
