<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { uptime_api, whatsapp_link,
         facebook_link, twitter_link, instagram_link, linkedin_link, github_link,
          ascard_link, email_link } from '../config';

    // Social Media Icons
    import Facebook from 'svelte-bootstrap-icons/lib/Facebook.svelte';
    import Twitter from 'svelte-bootstrap-icons/lib/Twitter.svelte';
    import Instagram from 'svelte-bootstrap-icons/lib/Instagram.svelte';
    import Linkedin from 'svelte-bootstrap-icons/lib/Linkedin.svelte';
    import Github from 'svelte-bootstrap-icons/lib/Github.svelte';
    import PostcardHeart from 'svelte-bootstrap-icons/lib/PostcardHeart.svelte';
    import CircleFill from 'svelte-bootstrap-icons/lib/CircleFill.svelte';
    import Whatsapp from 'svelte-bootstrap-icons/lib/Whatsapp.svelte';
    import Envelope from 'svelte-bootstrap-icons/lib/Envelope.svelte';
    import Share from 'svelte-bootstrap-icons/lib/Share.svelte';

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
            mode: 'cors'
        });
        monitors = await monitors_res.json();
    });

    async function shar() {
        if (navigator.canShare) {
            navigator.share({
                title: 'UpVent Technologies',
                text: '¡Visita UpVent!',
                url: 'https://upvent.codes/'
            });
        }
    }
</script>

<style>.bg-light-gray { background-color: #f9fafb; }</style>

<footer class="text-center text-muted bg-light-gray">
    <div class="container p-4">
        <section class="mt-5">
            <div class="row">
                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <div class="container">
                        <Lazy height={47}><img class="img-fluid mb-1" width="116" height="47" src="{logo}" alt="UpVent Logo"/></Lazy>
                    </div>
                    <p>Todos los derechos reservados © - UpVent Technologies 2020 - {new Date().getFullYear()} . Todos los logos son marcas registradas de sus respectivos dueños.</p>
                    <div class="container mt-2 mb-2">
                        <a aria-label="Facebook" class="text-muted px-2" href="{facebook_link}"><Facebook width={24} height={24}/></a>
                        <a aria-label="Twitter" class="text-muted px-2" href="{twitter_link}"><Twitter width={24} height={24}/></a>
                        <a aria-label="Instagram" class="text-muted px-2" href="{instagram_link}"><Instagram width={24} height={24}/></a>
                        <a aria-label="LinkedIn" class="text-muted px-2" href="{linkedin_link}"><Linkedin width={24} height={24}/></a>
                        <a aria-label="GitHub" class="text-muted px-2" href="{github_link}"><Github width={24} height={24}/></a>
                        <a aria-label="As-Card" class="text-muted px-2" href="{ascard_link}"><PostcardHeart width={24} height={24}/></a>
                        <a aria-label="Whatsapp" class="text-muted px-2" href="{whatsapp_link}"><Whatsapp width={24} height={24}/></a>
                        <a aria-label="Mail" class="text-muted px-2" href="{email_link}"><Envelope width={24} height={24}/></a>
                        <button on:click={shar} class="btn text-muted px-2"><Share/></button>
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
                        <li class="m-2">
                            <a href="/pwa" class="text-decoration-none text-muted">PWA Store</a>
                        </li>
                    </ul>
                </div>

                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <p class="h4 text-uppercase">Enlaces Importantes</p>
                    <ul class="list-unstyled mb-0">
                        <li class="m-2">
                            <a rel="nofollow" href="https://github.com/UpVent" class="text-decoration-none text-muted">Repositorio de Código Libre en GitHub</a>
                        </li>
                    </ul>
                </div>
            </div>
        </section>
    </div>

    <section class="container mt-5 mb-5 text-center-p-4">
        <p>El <a rel="nofollow" href="https://github.com/UpVent/upvent-svelte">código fuente</a> de esta página se encuentra bajo la <a href="https://www.gnu.org/licenses/agpl-3.0.html">Licencia Pública General de Affero (GNU) versión 3</a>. Excepto donde se indique lo <a href="https://creativecommons.org/policies#license">contrario</a>, el trabajo escrito, blogs, opiniones y parte del contenido visual se encuentra bajo la <a href="https://creativecommons.org/licenses/by-nd/3.0/deed.es">Licencia Creative Commons Atribución-SinDerivadas 3.0 No portada (CC BY-ND 3.0)</a></p>
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
        {:catch}
            <p class="text-danger">Error al obtener el estátus de UpVent. Si ves esta pantalla, reporta el incidente con <a href="mailto:contacto@upvent.codes">el equipo de soporte de UpVent</a></p>
        {/await}
    </section>

    <section class="container text-center fw-bold">
        <p class="text-dark">
            Hecho con
            <a rel="nofollow" href="https://svelte.dev/">
                <svg xmlns="http://www.w3.org/2000/svg" style="vertical-align: -0.125em;" width="20" height="24.07" preserveAspectRatio="xMidYMid meet" viewBox="0 0 256 308"><path fill="#FF3E00" d="M239.682 40.707C211.113-.182 154.69-12.301 113.895 13.69L42.247 59.356a82.198 82.198 0 0 0-37.135 55.056a86.566 86.566 0 0 0 8.536 55.576a82.425 82.425 0 0 0-12.296 30.719a87.596 87.596 0 0 0 14.964 66.244c28.574 40.893 84.997 53.007 125.787 27.016l71.648-45.664a82.182 82.182 0 0 0 37.135-55.057a86.601 86.601 0 0 0-8.53-55.577a82.409 82.409 0 0 0 12.29-30.718a87.573 87.573 0 0 0-14.963-66.244"/><path fill="#FFF" d="M106.889 270.841c-23.102 6.007-47.497-3.036-61.103-22.648a52.685 52.685 0 0 1-9.003-39.85a49.978 49.978 0 0 1 1.713-6.693l1.35-4.115l3.671 2.697a92.447 92.447 0 0 0 28.036 14.007l2.663.808l-.245 2.659a16.067 16.067 0 0 0 2.89 10.656a17.143 17.143 0 0 0 18.397 6.828a15.786 15.786 0 0 0 4.403-1.935l71.67-45.672a14.922 14.922 0 0 0 6.734-9.977a15.923 15.923 0 0 0-2.713-12.011a17.156 17.156 0 0 0-18.404-6.832a15.78 15.78 0 0 0-4.396 1.933l-27.35 17.434a52.298 52.298 0 0 1-14.553 6.391c-23.101 6.007-47.497-3.036-61.101-22.649a52.681 52.681 0 0 1-9.004-39.849a49.428 49.428 0 0 1 22.34-33.114l71.664-45.677a52.218 52.218 0 0 1 14.563-6.398c23.101-6.007 47.497 3.036 61.101 22.648a52.685 52.685 0 0 1 9.004 39.85a50.559 50.559 0 0 1-1.713 6.692l-1.35 4.116l-3.67-2.693a92.373 92.373 0 0 0-28.037-14.013l-2.664-.809l.246-2.658a16.099 16.099 0 0 0-2.89-10.656a17.143 17.143 0 0 0-18.398-6.828a15.786 15.786 0 0 0-4.402 1.935l-71.67 45.674a14.898 14.898 0 0 0-6.73 9.975a15.9 15.9 0 0 0 2.709 12.012a17.156 17.156 0 0 0 18.404 6.832a15.841 15.841 0 0 0 4.402-1.935l27.345-17.427a52.147 52.147 0 0 1 14.552-6.397c23.101-6.006 47.497 3.037 61.102 22.65a52.681 52.681 0 0 1 9.003 39.848a49.453 49.453 0 0 1-22.34 33.12l-71.664 45.673a52.218 52.218 0 0 1-14.563 6.398"/></svg>
            </a> 
            y
            <a rel="nofollow" href="https://pocketbase.io/">
                <svg xmlns="http://www.w3.org/2000/svg" style="vertical-align: -0.125em;" width="20" height="20" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="#010101" d="M5.684 12a.632.632 0 0 1-.631-.632V4.421c0-.349.282-.632.631-.632h2.37c.46 0 .889.047 1.287.139c.407.084.758.23 1.053.44c.303.202.541.475.715.82c.173.335.26.75.26 1.246c0 .479-.092.894-.273 1.247a2.373 2.373 0 0 1-.715.869a3.11 3.11 0 0 1-1.053.503c-.398.11-.823.164-1.273.164h-.46a.632.632 0 0 0-.632.632v1.52a.632.632 0 0 1-.632.631Zm1.279-4.888c0 .349.283.632.632.632h.343c1.04 0 1.56-.437 1.56-1.31c0-.428-.135-.73-.404-.907c-.26-.176-.645-.264-1.156-.264h-.343a.632.632 0 0 0-.632.631Zm6.3 13.098a.632.632 0 0 1-.631-.631v-6.947a.63.63 0 0 1 .631-.632h2.203c.44 0 .845.034 1.216.1c.38.06.708.169.984.328c.276.16.492.37.647.63c.164.26.246.587.246.982c0 .185-.03.37-.09.554a1.537 1.537 0 0 1-.26.516a1.857 1.857 0 0 1-1.076.7a.031.031 0 0 0-.023.03c0 .015.01.028.025.03c.591.111 1.04.32 1.346.626c.311.31.466.743.466 1.297c0 .42-.082.78-.246 1.083a2.153 2.153 0 0 1-.685.755a3.4 3.4 0 0 1-1.036.441a5.477 5.477 0 0 1-1.268.139zm1.271-5.542c0 .349.283.631.632.631h.21c.465 0 .802-.088 1.009-.264c.207-.176.31-.424.31-.743c0-.302-.107-.516-.323-.642c-.207-.135-.535-.202-.984-.202h-.222a.632.632 0 0 0-.632.632Zm0 3.463c0 .349.283.631.632.631h.39c1.019 0 1.528-.369 1.528-1.108c0-.36-.125-.621-.376-.78c-.241-.16-.625-.24-1.152-.24h-.39a.632.632 0 0 0-.632.632zM1.389 0C.629 0 0 .629 0 1.389V15.03a1.4 1.4 0 0 0 1.389 1.39H8.21a.632.632 0 0 0 .63-.632a.632.632 0 0 0-.63-.63H1.389c-.078 0-.125-.05-.125-.128V1.39c0-.078.047-.125.125-.125H15.03c.078 0 .127.047.127.125v6.82a.632.632 0 0 0 .631.63a.632.632 0 0 0 .633-.63V1.389A1.4 1.4 0 0 0 15.032 0ZM15.79 7.578a.632.632 0 0 0-.632.633a.632.632 0 0 0 .631.63h6.822c.078 0 .125.05.125.128V22.61c0 .078-.047.125-.125.125H8.97c-.077 0-.127-.047-.127-.125v-6.82a.632.632 0 0 0-.631-.63a.632.632 0 0 0-.633.63v6.822A1.4 1.4 0 0 0 8.968 24h13.643c.76 0 1.389-.629 1.389-1.389V8.97a1.4 1.4 0 0 0-1.389-1.39Z"/></svg> 
            </a> 
            por <a class="text-decoration-none link-dark" rel="nofollow" href="https://upvent.codes/">UpVent Technologies.</a>
        </p>
    </section>

    <section class="text-center border-top p-3">
        © 2020 - {new Date().getFullYear()} Copyright:
        <a href="https://upvent.codes/">UpVent Technologies</a> -
        <a href="/privacy-policy"> Política de privacidad</a> -
        <a  rel="nofollow" href="https://github.com/UpVent/upvent-svelte/issues/new">Reportar un problema con este sitio</a>
    </section>
</footer>
