<script lang="ts">
 /* Svelte / Internal imports */
 import SEO from '$lib/SEO.svelte';

 /* Asset imports */
 import logo from '$lib/assets/upvent-main.webp';
 import universal from '$lib/assets/universal.svg';
 import secure from '$lib/assets/secure.svg';
 import escalable from '$lib/assets/escalable.svg';
 import rapido from '$lib/assets/rapido.svg';
 import gnu from '$lib/assets/gnu.webp';
 import coins from '$lib/assets/coins.svg';
 import education from '$lib/assets/education.svg';
 import brain from '$lib/assets/brain.svg';

 /* External imports */
 import ArrowRight from "svelte-bootstrap-icons/lib/ArrowRight.svelte";
 import WrenchAdjustableCircle from "svelte-bootstrap-icons/lib/WrenchAdjustableCircle.svelte";
</script>

<style>
    .waves {
        background: url('/src/lib/assets/waves.webp');
        background-size: contain;
        background-repeat: repeat-x;
        height: 450px;
    }
    .text-glow-white { text-shadow: 1px 1px 1px #010101; }
    .upvent-description {
        background: rgba( 255, 255, 255, 0.35 );
        box-shadow: 0 8px 32px 0 rgba( 31, 38, 135, 0.37 );
        -webkit-backdrop-filter: blur( 5.5px );
        backdrop-filter: blur( 5.5px );
        border-radius: 10px;
        border: 1px solid rgba( 255, 255, 255, 0.18 );
    }
</style>

<!-- SEO component -->
<SEO/>

<!-- Jumbotron Section -->
<section class="pt-4 mb-2 pt-md-11 waves">
    <div class="container">
        <div class="row">
            <div class="col-12 col-md-5 col-lg-6 order-md-2 d-none d-sm-block mt-xs-0">
                <img src="{logo}" height="270" width="270" class="shadow-sm rounded-circle mb-6 mb-md-0 " alt="logo" loading="lazy">
            </div>
            <div class="col-12 col-md-7 col-lg-6 upvent-description order-md-1">
                <h1 class="display-4 fw-bold text-center text-md-start">
                    Nube <span class="text-light text-glow-white">inteligente.</span>
                    <br>
                    Para negocios inteligentes.
                </h1>
                <p class="link-dark text-center text-md-start mb-6 mb-lg-8">
                    Impulse la innovación en su negocio con soluciones sencillas y éticas de software libre. Conozca por que las pequeñas y medianas empresas Mexicanas confían en UpVent.
                </p>
            <div class="text-center text-md-start">
              <a href="/about" class="btn btn-primary mx-auto my-2 btn-lg me-1">Portafolio de Trabajo <ArrowRight/></a>
            </div>
          </div>
        </div>
    </div>
</section>
<section>
    <div class="container mt-2 mb-4">
        <div class="row justify-content-center">
            <div class="col-12 col-md-10 col-lg-8 text-center">
                <h2 class="fw-bold"> Soluciones en la nube <span class="text-primary">para todo tipo de negocios</span>.</h2>
                <p class="fs-5 text-muted mb-9">
                    UpVent es una empresa de soluciones creadas con software libre para
                    su proyecto, empresa o equipo de trabajo. Soluciones, Infraestructura. ¡Lo tenemos cubierto!
                </p>
            </div>
        </div>
        <div class="row">
            <div class="col-12 col-lg-6">
                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <img width="216" height="189" src="{universal}" alt="Gráfico mostrando a una persona junto a un teléfono celular enorme" class="mt-1 img-fluid" loading="lazy">
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">Universales</h3>
                        <p class="text-muted mb-0">
                            Haga uso de su solución personalizada en computadoras, tabletas y teléfonos.
                        </p>
                    </div>
                </div>

                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5">
                        <img width="216" height="154" src="{secure}" alt="..." class="mt-1 img-fluid" loading="lazy">
                    </div>
                    <div class="col-8 col-lg-7 mt-2 mb-2">
                        <h3 class="fw-bold mb-2">Seguros</h3>
                        <p class="text-muted mb-0">
                            Su solución empresarial estará programada para ser segura desde el inicio.
                        </p>
                    </div>
                </div>
            </div>

            <div class="col-12 col-lg-6">
                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <img width="216" height="137" src="{escalable}" alt="..." class="mb-1 img-fluid" loading="lazy">
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">Escalables</h3>
                        <p class="text-muted mb-0">
                            Manténgase siempre a la vanguardia con software escalable desde el primer día.
                        </p>
                    </div>
                </div>

                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <img width="216" height="169" src="{rapido}" alt="..." class="mt-1 img-fluid" loading="lazy">
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">Rápidos</h3>
                        <p class="text-muted mb-0">
                            Programación cautelosa de los productos desde su raíz. ¡No más software lento!
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</section>

<section class="container mt-5 mb-5 text-center">
    <img class="d-block mx-auto mb-4" src="{gnu}" alt="Logo de UpVent circular" width="150" height="150" loading="lazy"/>
    <p class="h2 fw-bold">Todo lo que hacemos es de</p> <span class="h2 fw-bold text-primary">código libre</span>
    <p>
        Nuestro modelo de desarrollo consiste totalmente de componentes de código
        libre y se basa en múltiples principios éticos y prácticos para su conveniencia como empresa.
    </p>
</section>

<section class="container text-center mt-5 mb-5">
        <p class="h1">Beneficios</p>
        <p class="lead text-muted">Conozca los beneficios que UpVent tiene para usted y su empresa</p>
</section>

<section class="container mt-5 mb-5">
    <div class="row">
        <div class="col-md-7">
            <p class="h2">¡Somos Low Cost! <span class="text-muted">¡Siempre!</span></p>
            <p class="lead">
                UpVent ofrece múltiples servicios a las empresas, todos y cada uno de ellos son económicos, sencillos y siguen un riguroso proceso para cumplir con la calidad esperada para nuestros clientes.
                <br />
                Esto nos hace una opción ideal para startups y pequeñas empresas que desean digitalizarse con poca inversión.
            </p>
        </div>
        <div class="col-md-5">
            <img width="276" height="202" class="img-fluid" src="{coins}" alt="Foto mostrando unas monedas y un bolígrafo" loading="lazy">
        </div>
    </div>
    <hr>
    <div class="row">
        <div class="col-md-7 order-md-2">
            <p class="h2">¡A favor de la educación! <span class="text-muted">Aprenda con nosotros</span></p>
            <p class="lead">UpVent se fundó con la visión de compartir el conocimiento. Tenemos la promesa de educar de forma abierta y gratuita, llevando nuestro conocimiento a todos los individuos que podamos.</p>
        </div>
        <div class="col-md-5 order-md-1">
            <img width="276" height="193" class="img-fluid" src="{education}" alt="Imágen mostrando a una persona leyendo un libro, sentada debajo de un árbol" loading="lazy">
        </div>
    </div>
    <hr>
    <div class="row">
        <div class="col-md-7">
            <p class="h2">La estabilidad primero. <span class="text-muted">Puede confiar en UpVent</span></p>
            <p class="lead">UpVent mejorará constantemente sus tecnologías actuales, siempre procurando la estabilidad por encima de cualquier otra cualidad que podamos ofrecer.</p>
        </div>
        <div class="col-md-5">
            <img width="276" height="182" class="img-fluid" src="{brain}" alt="Imágen mostrando un cerebro y una bombilla de fondo" loading="lazy">
        </div>
    </div>
</section>
<section class="px-4 py-5 my-5 text-center">
    <img class="d-block mx-auto mb-4 rounded-circle shadow-sm" src="{logo}" alt="Logo de UpVent circular" width="100" height="100" loading="lazy"/>
    <p class="h1 display-5 fw-bold">Comenzar a trabajar con <span class="h1 display-5 fw-bold text-primary">UpVent.</span></p>
    <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">Potencie su negocio ahora mismo, mire nuestro portafolio de trabajo o visite nuestra tienda de servicios para comenzar hoy mismo.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="/store" class="btn btn-primary btn-lg px-4 me-sm-3">Comprar Ahora</a>
            <a href="/about" class="btn btn-outline-primary fw-bold btn-lg px-4">Ver Portafolio</a>
        </div>
    </div>
</section>

<section class="text-center">
    <WrenchAdjustableCircle height={48} width={48} class="text-primary"/>
    <p class="h2 fw-bold m-2">Tecnología que nos inspira</p>
    <div class="container mt-3 mb-2">
        <div class="container" id="startnowtechs">
            <div class="row row-cols-4">
                <div class="col">
                    <a aria-label="Svelte" rel="noopener noreferrer" href="https://svelte.dev">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 32 32"><path fill="#ff3e00" d="M26.5 5.7a9 9 0 0 0-11.8-2.5L8 7.4a7.5 7.5 0 0 0-3.5 5 7.7 7.7 0 0 0 .8 5 7.4 7.4 0 0 0-1.3 3 7.8 7.8 0 0 0 1.4 6 9 9 0 0 0 11.8 2.5l6.7-4.2a7.5 7.5 0 0 0 3.5-5 7.7 7.7 0 0 0-.8-5 7.3 7.3 0 0 0 1.2-2.9 7.8 7.8 0 0 0-1.4-6"/><path fill="#fff" d="M14 26.6a5.4 5.4 0 0 1-5.7-2 4.7 4.7 0 0 1-.8-3.6 4.3 4.3 0 0 1 .1-.7l.1-.3.4.2a8.8 8.8 0 0 0 2.6 1.3h.3v.3a1.4 1.4 0 0 0 .2 1 1.6 1.6 0 0 0 1.7.6 1.5 1.5 0 0 0 .5-.2l6.7-4.1a1.4 1.4 0 0 0 .6-1 1.4 1.4 0 0 0-.3-1 1.6 1.6 0 0 0-1.7-.7 1.5 1.5 0 0 0-.4.2l-2.6 1.6a5 5 0 0 1-1.3.6 5.4 5.4 0 0 1-5.8-2 4.7 4.7 0 0 1-.8-3.7 4.5 4.5 0 0 1 2.1-3l6.7-4.2a5 5 0 0 1 1.4-.5 5.4 5.4 0 0 1 5.7 2 4.7 4.7 0 0 1 .8 3.6 4.3 4.3 0 0 1-.1.6l-.1.4-.4-.2a8.7 8.7 0 0 0-2.6-1.3H21v-.3a1.4 1.4 0 0 0-.2-1 1.6 1.6 0 0 0-1.7-.6 1.5 1.5 0 0 0-.4.2l-6.8 4.1a1.4 1.4 0 0 0-.6 1 1.4 1.4 0 0 0 .3 1 1.6 1.6 0 0 0 1.7.6 1.5 1.5 0 0 0 .4-.1l2.6-1.6a4.9 4.9 0 0 1 1.3-.6 5.4 5.4 0 0 1 5.7 2 4.7 4.7 0 0 1 .9 3.7 4.5 4.5 0 0 1-2.1 3l-6.7 4.2a4.9 4.9 0 0 1-1.4.5"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a aria-label="Rust" rel="noopener noreferrer" href="https://rust-lang.org">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 32 32"><path fill="#050505" d="M15.1 5.3a.8.8 0 1 1 .9.8.8.8 0 0 1-.9-.8m-9.9 7.5a.8.8 0 1 1 .8.9.8.8 0 0 1-.8-.9m19.9 0a.8.8 0 1 1 .8.9.8.8 0 0 1-.8-.8M7.6 14a.8.8 0 0 0 .4-1l-.4-.8h1.5v6.5H6a10.2 10.2 0 0 1-.3-3.9Zm6 .2v-2h3.5c.2 0 1.3.3 1.3 1s-.8 1-1.5 1ZM9 24.6a.8.8 0 1 1 .8.8.8.8 0 0 1-.8-.8m12.3 0a.8.8 0 1 1 .8.8.8.8 0 0 1-.8-.8m.2-1.9a.8.8 0 0 0-.9.6l-.4 2a10.2 10.2 0 0 1-8.5 0l-.4-2a.8.8 0 0 0-1-.6l-1.7.3a10.2 10.2 0 0 1-.8-1H16l.2-.1v-3h-2.6v-2h2.6a1.7 1.7 0 0 1 1.7 1.5c0 .4.3 1.7.5 2.1.1.5.8 1.5 1.4 1.5h4.4a10.2 10.2 0 0 1-1 1Zm4.7-7.8a10.3 10.3 0 0 1 0 1.8h-1l-.2.1v.5c0 1.2-.6 1.4-1.2 1.5-.5 0-1.1-.3-1.2-.6a3.6 3.6 0 0 0-1.7-2.8c1-.6 2.1-1.6 2.1-2.9a3.3 3.3 0 0 0-1.6-2.7 4.6 4.6 0 0 0-2.2-.7H8.4A10.2 10.2 0 0 1 14 5.8l1.3 1.4a.8.8 0 0 0 1 0L18 5.8a10.2 10.2 0 0 1 7 5l-1 2.3a.8.8 0 0 0 .4 1Zm2.4 0v-.3l1-1a.4.4 0 0 0 0-.6.4.4 0 0 0-.1 0l-1.3-.5-.1-.4.8-1a.4.4 0 0 0-.1-.7.4.4 0 0 0-.2 0l-1.3-.2-.2-.3.6-1.3a.4.4 0 0 0-.2-.5.4.4 0 0 0-.2 0H26l-.2-.3.3-1.3a.4.4 0 0 0-.3-.5.4.4 0 0 0-.2 0l-1.3.3-.3-.2V4.7a.4.4 0 0 0-.4-.4.4.4 0 0 0-.1 0L22 5l-.3-.2-.2-1.3a.4.4 0 0 0-.5-.4.4.4 0 0 0-.1.1l-1.1.8h-.4L19 2.4a.4.4 0 0 0-.5-.2.4.4 0 0 0-.1 0l-1 1h-.3l-.8-1.1a.4.4 0 0 0-.6 0l-.8 1.2h-.3l-1-1a.4.4 0 0 0-.5 0 .4.4 0 0 0-.1.1l-.5 1.3-.3.1L11 3a.4.4 0 0 0-.6.1.4.4 0 0 0 0 .2l-.2 1.3-.3.2-1.3-.6a.4.4 0 0 0-.5.3.4.4 0 0 0 0 .1V6l-.3.2L6.5 6a.4.4 0 0 0-.5.3.4.4 0 0 0 0 .2l.3 1.3-.2.3H4.7a.4.4 0 0 0-.4.5L5 10l-.2.3-1.3.2a.4.4 0 0 0-.4.5.4.4 0 0 0 .1.1l.8 1.1v.4l-1.4.5a.4.4 0 0 0-.2.5.4.4 0 0 0 0 .1l1 1v.3l-1.1.8a.4.4 0 0 0 0 .6l1.2.8v.3l-1 1a.4.4 0 0 0 0 .6.4.4 0 0 0 .1 0l1.3.5.1.3-.8 1.2a.4.4 0 0 0 .1.6.4.4 0 0 0 .2 0l1.3.2.2.3-.6 1.3a.4.4 0 0 0 .3.5.4.4 0 0 0 .1 0h1.4l.2.3-.3 1.3a.4.4 0 0 0 .3.5.4.4 0 0 0 .2 0l1.3-.3.3.2v1.4a.4.4 0 0 0 .4.4.4.4 0 0 0 .1 0L10 27l.3.2.2 1.3a.4.4 0 0 0 .5.4.4.4 0 0 0 .1-.1l1.1-.8h.4l.5 1.4a.4.4 0 0 0 .5.2.4.4 0 0 0 .1 0l1-1h.3l.8 1.1a.4.4 0 0 0 .5.1.4.4 0 0 0 .1 0l.8-1.3h.3l1 1a.4.4 0 0 0 .5 0 .4.4 0 0 0 .1-.1l.5-1.3.3-.1 1.2.8a.4.4 0 0 0 .5-.1.4.4 0 0 0 .1-.2l.2-1.3.3-.2 1.3.6a.4.4 0 0 0 .5-.3.4.4 0 0 0 0-.1v-1.4l.3-.2 1.3.3a.4.4 0 0 0 .5-.3.4.4 0 0 0 0-.2l-.3-1.3.2-.3h1.4a.4.4 0 0 0 .4-.4.4.4 0 0 0 0-.1L27 22l.2-.3 1.3-.2a.4.4 0 0 0 .4-.5.4.4 0 0 0-.1-.1l-.8-1.1v-.4l1.4-.5a.4.4 0 0 0 .2-.5.4.4 0 0 0 0-.1l-1-1V17l1.1-.8a.4.4 0 0 0 0-.6Z"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a aria-label="Debian" rel="noopener noreferrer" href="https://debian.org">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 48 48"><path fill="#E91E63" d="M26.8 24.5c-.7 0 0 .4.9.5a9.6 9.6 0 0 0 .6-.5h-1.5m3.3-.8c.3-.5.6-1 .7-1.6l-.5 1.1c-1.2.8-.1-.4 0-.8-1.3 1.5-.2.9-.2 1.3m1.2-3.1c0-1.1-.3-.8-.3-.4 0 0 .2.8.3.4M24.4 5.5c.4 0 .7 0 .7.2.3-.1.4-.2-.7-.2m.7.2h-.2.2M35.3 21c0 1-.3 1.5-.6 2.3l-.5.3c-.5.8 0 .5-.3 1.2-.7.6-2 1.9-2.5 2-.3 0 .2-.4.3-.5-.9.6-.7.9-2.1 1.3v-.1c-3.5 1.6-8.2-1.6-8.2-6l-.1.4c-.2-2.3 1-4.5 3-5.4 2-1 4.4-.6 5.8.7-.8-1-2.3-2.1-4.2-2-1.8 0-3.5 1.2-4 2.4-1 .6-1.1 2.3-1.5 2.6-.6 4 1 5.7 3.7 7.8.4.2 0 .3.1.5a7.2 7.2 0 0 1-2.3-1.8c.3.5.7 1 1.2 1.4-.8-.3-2-2-2.3-2 1.5 2.5 5.9 4.4 8.2 3.5-1.1 0-2.5 0-3.6-.4-.5-.3-1.2-.8-1.1-1a9 9 0 0 0 9-1.2c.8-.6 1.5-1.5 1.7-1.5-.3.5 0 .2-.2.7.7-1.1-.3-.5.7-2l.4.6c-.1-1 1.2-2 1-3.5.3-.5.4.5 0 1.5.5-1.1.2-1.3.3-2.3 0 .4.2.7.3 1-.3-1 .3-1.8.5-2.5-.2 0-.5.5-.5-.8 0-.5.1-.3.2-.4-.1 0-.4-.5-.6-1.3.1-.2.3.5.5.5l-.3-1.7c-.5-1-.2.2-.6-.4-.5-1.7.5-.4.5-1.2.8 1.2 1.3 3 1.5 3.8a15.5 15.5 0 0 0-.7-2.7c.2.1-.4-1.9.3-.6a12 12 0 0 0-5.7-6.7l.6.6c-1.2-.7-1-.7-1.2-1-1-.4-1 0-1.6 0-1.8-1-2.1-.9-3.8-1.5l.1.4c-1.2-.4-1.4.1-2.6 0l.8-.3c-1.2.2-1.1-.2-2.2 0l.8-.4c-1 0-2.2.5-1.8 0C20.2 6 17.5 7 16 8.5v-.3c-.8.8-3.1 2.5-3.3 3.6h-.2c-.4.6-.6 1.3-.9 2-.4.8-.7.3-.6.4-1 1.9-1.4 3.4-1.8 4.7.3.5 0 2.6.1 4.3a19 19 0 0 0 13 18.5c1 .4 2.5.4 3.8.4-1.6-.4-1.8-.2-3.3-.7-1-.6-1.3-1.1-2-1.8l.3.5c-1.5-.5-.9-.6-2.1-1l.3-.4c-.5 0-1.3-.8-1.5-1.3h-.5c-.6-.7-1-1.3-1-1.7l-.1.3c-.2-.3-2.4-3-1.2-2.3-.3-.2-.5-.3-.8-.9l.2-.3c-.5-.6-1-1.5-1-1.8.3.4.5.4.7.5-1.3-3.3-1.4-.2-2.4-3.4h.2l-.4-.8v-1c-.9-1-.2-4.7 0-6.7 0-.8.8-1.7 1.3-3l-.3-.1c.6-1.1 3.6-4.5 5-4.3.6-.8-.2 0-.3-.2 1.5-1.5 2-1 3-1.3 1-.7-1 .2-.5-.3 1.9-.5 1.3-1 3.7-1.3.3.1-.6.2-.8.4 1.6-.7 5-.6 7 .4 2.6 1.2 5.4 4.7 5.5 8h.1c0 1.3.2 2.8-.2 4.1l.3-.6M20 25.4l-.1.4c.4.6.7 1.2 1.2 1.6l-1.1-2m1 0-.6-.8.7 1.3-.2-.5M38 21.7l-.1.2a11 11 0 0 1-1 3.4 10.7 10.7 0 0 0 1-3.6M24.7 5.2C25 5 25.6 5 26 5c-.6 0-1.1 0-1.7.2h.3M10 12.9c.1.9-.7 1.3.2.7.4-1-.2-.3-.2-.7M9 17l.3-1.3c-.5.7-.2.9-.3 1.3"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a aria-label="Bootstrap" rel="noopener noreferrer" href="https://getbootstrap.com/">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="39.9" viewBox="0 0 256 204"><path fill="#7E13F8" d="M53 0C39 0 28 13 28 27c1 13 0 30-4 44-5 14-12 23-24 24v13c12 2 19 11 24 25 4 14 5 31 4 44 0 14 11 27 25 27h150c14 0 25-13 25-27-1-13 0-30 4-44 5-14 12-23 24-25V95c-12-1-19-10-24-24-4-14-5-31-4-44 0-14-11-27-25-27H53Zm121 126c0 19-15 30-38 30H95a4 4 0 0 1-4-4V52a4 4 0 0 1 4-4h40c20 0 33 10 33 27 0 11-9 22-20 23v1c15 2 26 12 26 27Zm-43-65h-23v33h20c15 0 23-6 23-17 0-10-7-16-20-16Zm-23 46v35h24c16 0 24-6 24-18 0-11-9-17-25-17h-23Z"/></svg>
                    </a>
                </div>
            </div>
        </div>
    </div>
</section>
