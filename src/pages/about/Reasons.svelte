<script>
    import LightningChargeFill from 'svelte-bootstrap-icons/lib/LightningChargeFill.svelte';
    import CloudArrowUpFill from 'svelte-bootstrap-icons/lib/CloudArrowUpFill.svelte';
    import PhoneFill from 'svelte-bootstrap-icons/lib/PhoneFill.svelte';
    import Stack from 'svelte-bootstrap-icons/lib/Stack.svelte';
    import construction from '../../assets/images/construction.svg';
</script>

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
