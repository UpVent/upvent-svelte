<script lang="ts">
    // Lazy load images
    import Lazy from 'svelte-lazy';

    // Svelte bootstrap icons
    import Envelope from 'svelte-bootstrap-icons/lib/Envelope.svelte';

    // Include page images
    import programmer from '../../assets/images/programmer.svg';
    import desk from '../../assets/images/desk.svg';
    import partners from '../../assets/images/partners.webp';
    import security from '../../assets/images/security.svg';
    import logo from '../../assets/images/logo-grey.webp';
    import app from '../../assets/images/app.svg';
</script>

<section class="container col-xxl-8 px-4 py-5">
    <div class="row flex-lg-row-reverse align-items-center g-5 py-5">
        <div class="col-10 col-sm-8 col-lg-6">
            <Lazy height={500}>
                <img src="{programmer}" class="d-block mx-lg-auto img-fluid" alt="Imágen mostrando una persona programando" width="700" height="500">
            </Lazy>
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
            <Lazy height={260}>
                <img width="276" height="260" alt="Imágen de una persona sentada en un escritorio junto a una computadora" class="img-fluid" src="{desk}">
            </Lazy>
        </div>

    <div class="col-12 col-md-5 m-md-auto">
        <h2><b>Siempre libres</b></h2>
            <p class="lead">UpVent mantendrá siempre la libertad en sus productos, siguiendo los lineamientos éticos que vienen desde la raíz del software libre, entregaremos productos y servicios que preserven la ética y estabilidad del software en la industria.</p>
        </div>
    </div>

    <div class="row text-left align-items-center pt-5 pb-md-5">
        <div class="col-4 col-md-5 m-md-auto order-md-5">
            <img alt="Imágen mostrando los logos de los aliados de UpVent" class="img-fluid" src="{partners}">
        </div>

        <div class="col-12 col-md-5">
            <h2 class="fw-bold">Clientes beneficiados.</h2>
            <p class="lead">Nuestros clientes llevan productos al mundo todos los días utilizando la tecnología libre de UpVent. Estas organizaciones son la prueba de que el software libre y la innovación van de la mano en el mundo tecnológico.</p>
        </div>
    </div>

    <div class="row text-left align-items-center pt-5">
        <div class="col-4 col-md-5">
            <img alt="Imágen mostrando una computadora y un escudo" class="img-fluid" src="{security}">
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
            <p class="mb-5 mt-5"><img alt="Logo de UpVent" height="35" width="98" src="{logo}"></p>
            <h1>Permítanos solucionar sus problemas con software libre...</h1>
            <p class="mt-4"><a href="/services" class="btn btn-primary fw-bold">Explorar nuestras soluciones</a></p>
        </div>

        <div class="col-12 col-sm-4 col-md-6 col-lg-4 m-auto pt-5">
            <img alt="Imágen mostrando una persona mirando a un teléfono" class="img-fluid" src="{app}">
        </div>
    </div>
</section>
