<svelte:options immutable={true}/>

<script>
    // import lazy loading component
    import Lazy from 'svelte-lazy';
    import { ArrowRight } from 'svelte-bootstrap-icons';

    // Import images
    import coins from '../../assets/images/coins.svg';
    import education from '../../assets/images/education.svg';
    import brain from '../../assets/images/brain.svg';
</script>

<style>
    * {
        font-family: 'Poppins', sans-serif;
    }
</style>

<section>
    <div class="container text-center mt5 mb-5">
        <p class="h1">Beneficios</p>
        <p class="lead text-muted small">Conozca los beneficios que UpVent tiene para usted y su empresa</p>
    </div>
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
            <div class="container mt-5">
                <a href="/services">Conozca nuestros servicios <ArrowRight/></a>
            </div>
        </div>
        <div class="col-md-5">
            <Lazy height={202}>
                <img width="276" height="202" class="img-fluid" src="{coins}" alt="Foto mostrando unas monedas y un bolígrafo">
            </Lazy>
        </div>
    </div>

    <hr>

    <div class="row">
        <div class="col-md-7 order-md-2">
            <p class="h2">¡A favor de la educación! <span class="text-muted">Aprenda con nosotros</span></p>
            <p class="lead">UpVent se fundó con la visión de compartir el conocimiento. Tenemos la promesa de educar de forma abierta y gratuita, llevando el conocimiento de nuestro equipo a todos los individuos que podamos.</p>
            <a href="/blog">Visite nuestro blog <ArrowRight/></a>
        </div>
        <div class="col-md-5 order-md-1">
            <Lazy height={193}>
                <img width="276" height="193" class="img-fluid" src="{education}" alt="Imágen mostrando a una persona leyendo un libro, sentada debajo de un árbol">
            </Lazy>
        </div>
    </div>

    <hr>

    <div class="row">
        <div class="col-md-7">
            <p class="h2">La estabilidad primero. <span class="text-muted">Puede confiar en UpVent</span></p>
            <p class="lead">UpVent mejorará constantemente sus tecnologías actuales, siempre procurando la estabilidad por encima de cualquier otra cualidad que podamos ofrecer, esto se hace con el objetivo de fortalecer su relación con los individuos que lo conforman.
            </p>
            <a href="/about">Conozca nuestro ciclo de desarrollo <ArrowRight/></a>
        </div>
        <div class="col-md-5">
            <Lazy height={182}>
                <img width="276" height="182" class="img-fluid" src="{brain}" alt="Imágen mostrando un cerebro y una bombilla de fondo">
            </Lazy>
        </div>
    </div>
</section>
