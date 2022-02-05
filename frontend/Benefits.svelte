<script>
    let brain = 'images/brain.jpg';
    let coins = 'images/coins.jpg';
    let education = 'images/education.jpg';
</script>

<style>
</style>

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
                <a href="#">Conozca nuestros servicios <i class="bi bi-arrow-right"></i></a>
            </div>
        </div>
        <div class="col-md-5">
            <img class="img-fluid rounded-3 shadow" src={coins} alt="Foto mostrando unas monedas y un bolígrafo">
        </div>
    </div>

    <hr>

    <div class="row">
        <div class="col-md-7 order-md-2">
            <p class="h2">¡A favor de la educación! <span class="text-muted">Aprenda con nosotros</span></p>
            <p class="lead">UpVent se fundó con la visión de compartir el conocimiento. Tenemos la promesa de educar de forma abierta y gratuita, llevando el conocimiento de nuestro equipo a todos los individuos que podamos.</p>
            <a href="#">Visite nuestro blog <i class="bi bi-arrow-right"></i></a>
        </div>
        <div class="col-md-5 order-md-1">
            <img class="img-fluid rounded-3 shadow" src={education} alt="Imágen mostrando a una persona leyendo un libro, sentada debajo de un árbol">
        </div>
    </div>

    <hr>

    <div class="row">
        <div class="col-md-7">
            <p class="h2">La estabilidad primero. <span class="text-muted">Puede confiar en UpVent</span></p>
            <p class="lead">UpVent mejorará constantemente sus tecnologías actuales, siempre procurando la estabilidad por encima de cualquier otra cualidad que podamos ofrecer, esto se hace con el objetivo de fortalecer su relación con los individuos que lo conforman.
            </p>
            <a href="#">Conozca nuestro ciclo de desarrollo <i class="bi bi-arrow-right"></i></a>
        </div>
        <div class="col-md-5">
            <img class="img-fluid rounded-3 shadow" src={brain} alt="Imágen mostrando un cerebro y una bombilla de fondo">
        </div>
    </div>
</section>
