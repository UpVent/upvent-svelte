<script>
    import Fa from 'svelte-fa';
    import {
        faHome,
        faLaptop,
        faPen,
        faPhone,
        faShoppingBag,
        faUsers,
    } from '@fortawesome/free-solid-svg-icons';


    let logo = 'images/logo-grey.png';
</script>

<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-3 mb-0">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
    <img class="mx-auto"  width="148" height="48" src="{logo}" alt="UpVent Logo">
    </a>

    <ul class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="nav-link px-2 link-dark" href="/">Inicio <Fa icon={faHome} /></a></li>
        <li><a class="nav-link px-2 link-dark" href="/">Blog <Fa icon={faPen} /></a></li>
        <li><a class="nav-link px-2 link-dark" href="/">Nosotros <Fa icon={faUsers} /></a></li>
        <li><a class="nav-link px-2 link-dark" href="/">Servicios <Fa icon={faLaptop} /></a></li>
        <li><a class="nav-link px-2 link-dark" href="/">Contacto <Fa icon={faPhone} /></a></li>
        <li><a class="nav-link px-2 link-dark" href="/">MarketCloud <Fa icon={faShoppingBag} /></a></li>
    </ul>
    <div class="text-end me-auto ms-auto mx-auto">
        <button type="button" class="btn btn-light border-0" title="Modo daltonismo">👁</button>
    </div>
</header>
