<script>
    // Import social media icons
    import Facebook from 'svelte-bootstrap-icons/lib/Facebook';
    import Twitter from 'svelte-bootstrap-icons/lib/Twitter';
    import Instagram from 'svelte-bootstrap-icons/lib/Instagram';
    import Linkedin from 'svelte-bootstrap-icons/lib/Linkedin';
    import Github from 'svelte-bootstrap-icons/lib/Github';
    import PostcardHeart from 'svelte-bootstrap-icons/lib/PostcardHeart';

    // Get current year
    let year = new Date().getFullYear();
</script>

<style>
    .bg-light-gray {
        background-color: rgb(249, 250, 251);
    }
</style>

<footer class="text-center text-muted bg-light-gray">
    <div class="container p-4">
        <section class="mt-5">
            <div class="row">
                <div class="col-lg-3 col-md-6 mb-4 mb-md-0">
                    <div class="container">
                        <img class="img-fluid mb-1" width="116" height="47" src="/images/logo-grey.webp" alt="UpVent Logo"/>
                    </div>
                    <p>Todos los derechos reservados © - UpVent Technologies 2020 - {year} . Todos los logos son marcas registradas de sus respectivos dueños.</p>
                    <div class="container mt-2 mb-2">
                        <a class="text-muted px-2" href="https://facebook.com/UpVentMX"><Facebook width={24} height={24}/></a>
                        <a class="text-muted px-2" href="https://twitter.com/UpVentMX"><Twitter width={24} height={24}/></a>
                        <a class="text-muted px-2" href="https://instagram.com/UpVentMX"><Instagram width={24} height={24}/></a>
                        <a class="text-muted px-2" href="https://www.linkedin.com/company/upvent-technologies/"><Linkedin width={24} height={24}/></a>
                        <a class="text-muted px-2" href="https://github.com/UpVent"><Github width={24} height={24}/></a>
                        <a class="text-muted px-2" href="https://me.as-card.com/accounts/profile/UpVentMX/"><PostcardHeart width={24} height={24}/></a>
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
        <p>
            El <a href="https://github.com/UpVent/upvent.codes">código fuente</a> de esta página se encuentra bajo la <a href="https://www.gnu.org/licenses/agpl-3.0.html">Licencia Pública General de Affero (GNU) versión 3</a>. Excepto donde se indique lo <a href="https://creativecommons.org/policies#license">contrario</a>, el trabajo escrito, blogs, opiniones y parte del contenido visual se encuentra bajo la <a href="https://creativecommons.org/licenses/by-nd/3.0/deed.es">Licencia Creative Commons Atribución-SinDerivadas 3.0 No portada (CC BY-ND 3.0)</a>
        </p>
    </section>

    <section class="text-center border-top p-3">
        © 2020 - {year} Copyright:
        <a href="https://upvent.codes/">UpVent Technologies</a> -
        <a href="/privacy-policy"> Política de privacidad</a> -
        <a href="https://github.com/UpVent/upventrs/issues/new">Reportar un problema con este sitio</a>
    </section>
</footer>
