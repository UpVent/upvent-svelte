<script>
    import { LightningCharge, Brush } from 'svelte-bootstrap-icons';
    import responsive from '../../assets/images/responsive.svg';
    import ecommerce from '../../assets/images/ecommerce.svg';
</script>

<section class="container mt-5 mb-5">
    <div class="row text-right align-items-center">
        <div class="col-12 col-lg-6 col-xl-5 m-lg-auto text-left">
            <h1 class="text-center fw-bold display-3">Páginas Web</h1>
            <div class="row pt-5">
                <div class="col-3"> <Brush width={100} height={100}/> </div>
                <div class="col-9">
                    <h2>Desarrollo Personalizado</h2>
                    <p class="text-muted text-wrap">
                        Desarrollamos su sitio web de acuerdo a sus necesidades, sea una pequeña empresa, freelancer o una corporación.
                        <br/>
                        UpVent estará ahí para ofrecerle una solución única.
                    </p>
                </div>
            </div>

            <div class="row pt-5">
                <div class="col-9 text-right">
                    <h2>Velocidad y Seguridad</h2>
                    <p class="text-muted text-wrap">
                        Creamos páginas web veloces utilizando una mezcla de tecnologías propias y tecnologías libres.
                        <br />
                        Gracias a la tecnología de LiteSpeed podemos ofrecerle sitios seguros y en producción casi de inmediato.
                    </p>
                </div>
                <div class="col-3"> <LightningCharge width={100} height={100}/> </div>
            </div>
        </div>

        <div class="col-7 col-sm-4 m-auto pt-5 pt-md-0">
            <img alt="Imágen mostrando una computadora" class="img-fluid" src="{responsive}">
        </div>
    </div>
</section>

<section class="container mt-5 mb-5">
    <div class="row align-items-center">
        <div class="col-12 col-md-6 mb-4 mb-md-0">
            <img alt="Imágen mostrando un e-commerce en funcionamiento" class="img-fluid" src="{ecommerce}">
        </div>
        <div class="col-12 col-md-6 col-lg-5 ml-md-auto text-left">
            <h1 class="fw-bold display-3">Ecommerce</h1>
            <p class="lead">Si necesita iniciar su negocio en línea tenemos la solución ideal para usted. Con UpVent podrá elegir entre múltiples opciones de e-commerce, desde <i>SpeedCommerce</i> propio de UpVent o sus motores favoritos como <i>Prestashop</i>, <i>WooCommerce</i> y muchos más.</p>
        </div>
    </div>
</section>
