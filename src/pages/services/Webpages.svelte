<script>
    import LightningCharge from 'svelte-bootstrap-icons/lib/LightningCharge.svelte';
    import Brush from 'svelte-bootstrap-icons/lib/Brush.svelte';
    import responsive from '../../assets/images/responsive.svg';
    import ecommerce from '../../assets/images/ecommerce.svg';

    import PersonHearts from 'svelte-bootstrap-icons/lib/PersonHearts.svelte';
    import Laptop from 'svelte-bootstrap-icons/lib/Laptop.svelte';
    import CodeSlash from 'svelte-bootstrap-icons/lib/CodeSlash.svelte';
    import Award from 'svelte-bootstrap-icons/lib/Award.svelte';
</script>

<style>
    .upvent-background {
        background-color: #007BFC;
    }
</style>

<section class="container mt-5 mb-5">
    <div class="row text-right align-items-center">
        <div class="col-12 col-lg-6 col-xl-5 m-lg-auto text-left">
            <h1 class="text-center fw-bold display-3">PWA's</h1>
            <h6 class="text-center">(Aplicaciones Web Progresivas)</h6>
            <div class="row pt-5">
                <div class="col-3"> <Brush width={90} height={90}/> </div>
                <div class="col-9">
                    <h2>Desarrollo Personalizado</h2>
                    <p class="text-muted text-wrap">
                        Desarrollamos su aplicación web de acuerdo a sus necesidades, sea una pequeña empresa, freelancer o una corporación.<br/>
                        UpVent estará ahí para ofrecerle una solución única.
                    </p>
                </div>
            </div>

            <div class="row pt-5">
                <div class="col-9 text-right">
                    <h2>Velocidad y Seguridad</h2>
                    <p class="text-muted text-wrap">
                        Creamos aplicaciones web veloces y disponibles incluso sin internet, todo esto utilizando una cuidadosa mezcla de tecnologías propias y tecnologías libres.<br />
                        Gracias a la tecnología de UpVent podemos ofrecerle sitios seguros y con un tiempo de carga casi instantáneo.
                    </p>
                </div>
                <div class="col-3"><LightningCharge width={90} height={90}/></div>
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
            <h1 class="fw-bold display-3">Mejora de sitios</h1>
            <p class="lead">Si ya tiene un sitio web y desea mejorarlo tenemos la solución ideal para usted.
                <br>
                Con UpVent podrá elegir entre convertirlo a una PWA o aumentar su velocidad hasta 20* veces más.</p>
        </div>
    </div>
</section>

<section class="container-fluid upvent-background text-light p-4 mt-2 mb-2">
    <div class="container">
        <div class="row">
            <div class="col text-center">
                <div class="container p-2">
                    <PersonHearts height={48} width={48}/>
                    <p class="h2 fw-bold">+20</p>
                    <hr>
                    Clientes Satisfechos.
                </div>
            </div>


            <div class="col text-center">
                <div class="container p-2">
                    <Laptop height={48} width={48}/>
                    <p class="h2 fw-bold">+10</p>
                    <hr>
                    Páginas Web Realizadas.
                </div>
            </div>

            <div class="col text-center">
                <div class="container p-2">
                    <CodeSlash height={48} width={48}/>
                    <p class="h2 fw-bold">+1,000,000</p>
                    <hr>
                    De líneas de código libre.
                </div>
            </div>

            <div class="col text-center">
                <div class="container p-2">
                    <Award height={48} width={48}/>
                    <p class="h2 fw-bold">+4</p>
                    <hr>
                    Casos de éxito en México.
                </div>
            </div>

        </div>
    </div>
</section>