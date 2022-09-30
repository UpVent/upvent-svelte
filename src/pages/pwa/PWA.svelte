<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte'; 
    import { fapi_url, api_user, api_user_pass } from '../../config';
    
    // Lazy Load Import
    import Lazy from 'svelte-lazy';

    // Database imports
    import PocketBase from 'pocketbase';
    import { Record } from 'pocketbase';

    // External imports
    import BagFill from 'svelte-bootstrap-icons/lib/BagFill.svelte';
    import StarFill from 'svelte-bootstrap-icons/lib/StarFill.svelte';
    import CartFill from 'svelte-bootstrap-icons/lib/CartFill.svelte';
    import QrCode from 'svelte-bootstrap-icons/lib/QrCode.svelte';

    // Database usage
    const client: PocketBase = new PocketBase(fapi_url);

    // Grid logos for technology showcasing
    let records: Record[] = [];

    let visible: boolean = false;

    function toggleVisible() {
        visible = !visible;
    }

    onMount(async () => {
        client.users.authViaEmail(api_user, api_user_pass);
        records = await client.records.getFullList('productos', 10, {
            sort: '+created',
        });
        console.log(records)
        client.authStore.clear();
    });
</script>

<style>
    .btn-stripe {
        background-color: #5433ff;
        border-color: #5433ff;
    }
</style>

<section class="container mb-5">
    <div class="bg-light d-lg-flex justify-content-between align-items-center py-6 py-lg-3 px-8 rounded-3 text-center text-lg-start">
        <div class="d-lg-flex align-items-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" preserveAspectRatio="xMidYMid meet" viewBox="0 0 32 32"><g fill="none"><path fill="#9B9B9B" d="M7.17 21.757L22.94 23l1.042-4h.536c1.372 0 2.494 1.34 2.494 2.885V22c0 1.608-1.203 3-2.634 3H3c-.547 0-1 .382-1 .956S2.447 27 2.994 27h21.425C26.943 27 29 24.756 29 22v-.115C29 19.192 26.993 17 24.518 17h-.015l.004-.012a.495.495 0 0 0 .393-.469v-.038a.47.47 0 0 0-.168-.359L27.37 6H5.2C3.44 6 2 7.432 2 9.253l.52 8.495c.15 2.517 2.52 3.84 4.65 4.009ZM21.616 16H20v-4h2.605l-.989 4Zm1.236-5H20V8.007l1.95.002c.84 0 1.45.776 1.25 1.582L22.852 11ZM19 8.006V11h-4.38V8l4.38.006Zm-5.38-.007V11h-4.6V7.994l4.6.005Zm-5.6-.006V11H4.111L4 9.193c0-.666.54-1.204 1.21-1.204l2.81.004ZM4.173 12H8.02v4h-3.6l-.247-4Zm.308 5H8.02v2.83l-.68-.053c-1.34-.11-2.74-.875-2.82-2.149L4.481 17Zm4.539 2.91V17h4.6v3.272l-4.6-.363Zm5.6.44V17H19v3.62c0 .026.002.051.006.076l-4.386-.346Zm5.38.25V17h1.369l-.679 2.747a1.276 1.276 0 0 1-.69.853ZM14.62 12H19v4h-4.38v-4Zm-5.6 0h4.6v4h-4.6v-4Z"/><path fill="#BEBEBE" d="M22.803 23.97c-.561-.131-.91-.688-.773-1.225l4.884-18.977c.138-.537.721-.871 1.282-.74c.562.132.912.69.774 1.227L24.086 23.23c-.138.547-.71.871-1.282.74Z"/><path fill="#0084CE" d="M27 5h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1ZM5.5 30a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3Zm18 0a1.5 1.5 0 1 0 0-3a1.5 1.5 0 0 0 0 3Z"/></g></svg>
                <!-- text -->
            <div class="ms-lg-4">
                <h1 class="fs-2 mb-1">Bienvenido a la tienda de UpVent</h1>
                <span>Encuentre la solución que busca con nosotros a un solo click.</span>

            </div>
        </div>
        <div class="mt-3 mt-lg-0">
            <a href="#estelare" class="btn btn-primary">Solicitar PWA <BagFill/></a>
        </div>
    </div>
</section>

<section class="container-fluid bg-primary text-light p-2 mt-2">
    <div class="container">
        <p class="h1 fw-bold">Productos Estelares <StarFill height={24} width={24} class="text-warning"/></p>
        <p class="text-light">Construya el sitio de su negocio a su gusto. Aumente sus ventas, mejore la retención de clientes y mucho más...!</p>
    </div>
</section>

<section id="estelare" class="container mt-3">
    <div class="row row-cols-sm-1 row-cols-md-3">
        {#await onMount}
           <p>Cargando productos...</p> 
        {:then}
        {#each records as record}
            <div class="col">
                <div class="card border-0 shadow-sm">
                    <Lazy>
                        <img class="img-fluid rounded-3" src="{ client.records.getFileUrl(record, record.imagen_destacada) }" alt="">
                    </Lazy>
                    <div class="card-header text-break border-0">
                        {record.nombre}
                    </div>
                    <div class="card-body">
                        <p class="rounded-pill m-2 bg-light fw-bold shadow-sm text-center  p-1">{(record.precio).toLocaleString('es-MX', { style: 'currency', currency: 'MXN' })}</p>
                        <br>
                        <div class="d-grid gap-2">
                            <a href="{record.stripe_payment_link} MXN" class="btn btn-primary m-0">Comprar <CartFill/> </a>
                            <button on:click={toggleVisible} class="btn btn-stripe btn-secondary">Pagar con QR <QrCode/></button>
                        </div>
                        {#if visible}
                            <img height="200" width="200" class="m-2 shadow-sm rounded-3 img-fluid" src={client.records.getFileUrl(record, record.qr_de_producto)} alt={record.nombre}>
                        {/if}
                    </div>
                </div>
            </div>
        {/each}
        {/await}
    </div> 
</section>