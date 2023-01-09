<script lang="ts">
 /* Asset imports */
 import work from "$lib/assets/work.svg";
 import construction from "$lib/assets/construction.svg";
 import programmer from '$lib/assets/programmer.svg';
 import desk from '$lib/assets/desk.svg';
 import partners from '$lib/assets/partners.webp';
 import security from '$lib/assets/security.svg';
 import logo from '$lib/assets/logo-grey.webp';
 import app from '$lib/assets/app.svg';

 /* External imports */
 import BuildingCheck from "svelte-bootstrap-icons/lib/BuildingCheck.svelte";
 import LightningChargeFill from 'svelte-bootstrap-icons/lib/LightningChargeFill.svelte';
 import CloudArrowUpFill from 'svelte-bootstrap-icons/lib/CloudArrowUpFill.svelte';
 import PhoneFill from 'svelte-bootstrap-icons/lib/PhoneFill.svelte';
 import Stack from 'svelte-bootstrap-icons/lib/Stack.svelte';
</script>

<style>
    .jumbotron {
        background-image: url("/src/lib/assets/core-about.webp");
        background-size: cover;
        height: 80%;
    }
</style>

<section class="p-5 text-center jumbotron">
    <div class="d-flex justify-content-center align-items-center m-5 h-100">
        <div class="text-white">
            <h1 class="text-center display-3 mb-3">Llevando empresas a la nube desde nuestros inicios.</h1>
            <p class="text-center text-wrap fs-5 mb-3">
                UpVent es una compañía de tecnología y software empresarial<br /> que sigue el modelo del software libre enfocado a las micro, pequeñas y medianas empresas.
            </p>
        </div>
    </div>
</section>

<section class="container mt-5 mb-5">
    <div class="text-center">
        <h1>Portafolio de Trabajo</h1>
        <p class="lead text-muted"> Conozca los trabajos de UpVent y la conformidad de sus clientes con los mismos. Además, también podrá ver nuestros proyectos de código libre en favor de la comunidad. </p>
        <a class="btn btn-primary btn-lg" href="/about/portfolio">Ver Portafolio <BuildingCheck height={24} width={24}/></a>
    </div>
</section>

<section class="pt-12 pt-md-14 mt-5 mb-5">
      <div class="container">
        <div class="row align-items-center text-center text-md-left">
          <div class="col-12 col-md-6">
            <img src="{work}" alt="..." class="img-fluid float-right mb-md-0">
          </div>
          <div class="col-12 col-md-6">
            <h1 class="display-1 fw-bold">
                <div>Trabajo <span class="text-primary">Inteligente</span></div>
            </h1>
            <p class="fs-4 text-muted mb-0">
                Las herramientas de productividad pueden ayudarlo o entorpecer su trabajo.
                UpVent tiene como objetivo eliminar todas las barreras entre
                usted y la tecnología. Nuestros productos están diseñados con la facilidad de uso y accesibilidad en mente, siempre poniendo al usuario y su comodidad como prioridad.
            </p>
          </div>
        </div>
      </div>
</section>

<section class="container mt-5 mb-5">
    <div class="container">
        <div class="row align-items-center pb-xl-5">
            <div class="col-12 col-md-7 col-xl-5">
                <h1>Razones para elegirnos</h1>
                <p class="lead text-muted">
                    UpVent es una organización comprometida con sus clientes. Planeamos el desarrollo de las soluciones de principio a fin, mantenemos la calidad y mejoramos constantemente para entregar productos pulidos y únicos.
                </p>
            </div>
            <div class="col-12 col-sm-6 col-md-4 m-sm-auto mr-md-0 ml-md-auto pt-4 pt-md-0">
                <img alt="Imágen mostrando a tres personas construyendo un sitio web" class="img-fluid" src="{construction}">
            </div>
        </div>

        <div class="row pt-5">
            <div class="col-12 col-sm-6 col-md-3 text-center">
                <h4 class="fw-bold">Soluciones Ligeras <LightningChargeFill width={24} height={24} class="text-primary"/></h4>
                <p>Creamos nuestras soluciones personalizadas desde cero, todo el proceso de elaboración se realiza con código optimizado, rápido y flexible.
                </p>
            </div>

            <div class="col-12 col-sm-6 col-md-3 pt-3 pt-sm-0 text-center">
                <h4 class="fw-bold">Mejoras Constantes <CloudArrowUpFill width={24} height={24} class="text-primary"/></h4>
                <p>Todas nuestras soluciones personalizadas son actualizables a versiones con mejoras de seguridad o de rendimiento. ¡No mas obsolecencia programada!</p>
            </div>

            <div class="col-12 col-sm-6 col-md-3 pt-3 pt-md-0 text-center">
                <h4 class="fw-bold">Desarrollo Universal <PhoneFill width={24} height={24} class="text-primary"/></h4>
                <p>Las soluciones ofrecidas por nosotros funcionan en todo tipo de dispositivos de uso común, desde laptops hasta tablets.</p>
            </div>

            <div class="col-12 col-sm-6 col-md-3 pt-3 pt-md-0 text-center">
                <h4 class="fw-bold">Estabilidad Confiable <Stack width={24} height={24} class="text-primary"/></h4>
                <p>UpVent sigue un ciclo de desarrollo enfocado en la estabilidad y la escalabilidad del software, nuestras soluciones personalizadas no se romperán entre actualizaciones.</p>
            </div>
        </div>
    </div>
</section>

<section class="container col-xxl-8 px-4 py-5">
    <div class="row flex-lg-row-reverse align-items-center g-5 py-5">
        <div class="col-10 col-sm-8 col-lg-6">
            <img src="{programmer}" class="d-block mx-lg-auto img-fluid" alt="Imágen mostrando una persona programando" width="700" height="500" loading="lazy">
        </div>
        <div class="col-lg-6">
            <h1 class="display-5 fw-normal lh-1 mb-3">Tecnología libre para las mentes libres</h1>
            <p class="lead">UpVent entrega soluciones de código libre que logran hacer que las empresas trabajen de forma eficiente en entornos conectados. Todos nuestros servicios operan de forma transparente y ética en el mercado laboral, sin telemetría, letra pequeña o restricciones sobre licencias.</p>
        </div>
    </div>
</section>

<section class="container">
    <div class="row justify-content-center pb-5">
        <div class="col-12 text-center">
            <h1 class="fw-bold">Líderes en soluciones de codigo libre en la industria</h1>
        </div>
    </div>

    <div class="row text-left align-items-center pt-5 pb-md-5">
        <div class="col-4 col-md-5">
            <img width="276" height="260" alt="Imágen de una persona sentada en un escritorio junto a una computadora" class="img-fluid" src="{desk}" loading="lazy">
        </div>

    <div class="col-12 col-md-5 m-md-auto">
        <h2><b>Siempre libres</b></h2>
            <p class="lead">UpVent mantendrá siempre la libertad en sus productos, siguiendo los lineamientos éticos que vienen desde la raíz del software libre, entregaremos productos y servicios que preserven la ética y estabilidad del software en la industria.</p>
        </div>
    </div>

    <div class="row text-left align-items-center pt-5 pb-md-5">
        <div class="col-4 col-md-5 m-md-auto order-md-5">
            <img alt="Imágen mostrando los logos de los aliados de UpVent" class="img-fluid" src="{partners}" loading="lazy">
        </div>

        <div class="col-12 col-md-5">
            <h2 class="fw-bold">Clientes beneficiados.</h2>
            <p class="lead">Nuestros clientes llevan productos al mundo todos los días utilizando la tecnología libre de UpVent. Estas organizaciones son la prueba de que el software libre y la innovación van de la mano en el mundo tecnológico.</p>
        </div>
    </div>

    <div class="row text-left align-items-center pt-5">
        <div class="col-4 col-md-5">
            <img alt="Imágen mostrando una computadora y un escudo" class="img-fluid" src="{security}" loading="lazy">
        </div>

        <div class="col-12 col-md-5 m-md-auto">
            <h2 class="fw-bold">Expertos en Seguridad</h2>
            <p class="lead">El equipo de UpVent siempre estará al pendiente de las actualizaciones, parches y mejoras de seguridad de sus productos. Olvide el software viejo y el riesgo de ataques informáticos, nosotros le tenemos cubierto.
            </p>
        </div>
    </div>
</section>

<section class="container mt-5 mb-5 align-items-end justify-content-center d-flex">
    <div class="row align-items-top text-left">
        <div class="col-12 col-md-6 col-lg-5">
            <p class="mb-5 mt-5"><img alt="Logo de UpVent" height="35" width="98" src="{logo}" loading="lazy"></p>
            <h1>Permítanos solucionar sus problemas con software libre...</h1>
            <p class="mt-4"><a href="/services" class="btn btn-primary fw-bold">Explorar nuestras soluciones</a></p>
        </div>

        <div class="col-12 col-sm-4 col-md-6 col-lg-4 m-auto pt-5">
            <img alt="Imágen mostrando una persona mirando a un teléfono" class="img-fluid" src="{app}" loading="lazy">
        </div>
    </div>
</section>
