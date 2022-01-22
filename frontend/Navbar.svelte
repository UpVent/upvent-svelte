<script>
    import House from 'svelte-bootstrap-icons/lib/House';

    import { Button } from 'sveltestrap';

    const logo = 'images/logo-grey.png';
</script>

<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-0">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
    <img class="mx-auto"  width="148" height="48" src="{logo}" alt="UpVent Logo">
    </a>

    <ul class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="nav-link px-2 text-muted" href="/">Inicio <House/></a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Blog</a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Nosotros</a></li>
        <li><a class="nav-link px-2 text-muted" href="/">Servicios</a></li>
        <li><a class="nav-link px-2 text-muted" href="/">MarketCloud</a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <Button size="lg" color="primary">Contacto</Button>
    </div>
</header>
