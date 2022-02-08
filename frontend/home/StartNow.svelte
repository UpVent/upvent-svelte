<script>

    import Laptop from 'svelte-bootstrap-icons/lib/Laptop';
    import Headset from 'svelte-bootstrap-icons/lib/Headset';

    let logo = 'images/upvent-logo-new.png';
</script>

<style>
    .text-glow {
        text-shadow: 1px 1px 1px #007BFC;
    }
</style>

<section class="px-4 py-5 my-5 text-center">
    <img class="d-block mx-auto mb-4 rounded-circle shadow" src={logo} alt="Logo de UpVent circular" width="100" height="100"/>
    <p class="h1 display-5 fw-bold">Comenzar a trabajar con <span class="h1 display-5 fw-bold text-primary text-glow">UpVent.</span></p>
    <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">Visite nuestra página de servicios para explorar nuestras ofertas prediseñadas para su empresa o contacte directo con ventas.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="#" class="btn btn-primary btn-lg px-4 me-sm-3">Ir a Servicios <Laptop/></a>
            <a href="#" class="btn btn-outline-primary btn-lg px-4">Contactar con ventas <Headset/></a>
        </div>
    </div>
</section>
