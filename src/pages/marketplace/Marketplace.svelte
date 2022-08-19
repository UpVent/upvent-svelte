<script lang="ts">
    import { onMount } from 'svelte';

    // Bootstrap icons
    import { ArrowRight } from 'svelte-bootstrap-icons';


    // Shopping Cart array
    let cart: any[] = [];

    // REST API PRODUCTS
    let products: any[] = [];

    const api_url: string = "https://wpapi.upvent.codes/wp-json/wp/v2/producto";

    onMount(async () => {
        // Products request
        const products_res: Response = await fetch(api_url);
        const products_json = await products_res.json();

        products = products_json;
        console.log(products);
    });

</script>

<style>
    .banner-holder {
        background: url(../../assets/images/marketbanner.png) center center / cover no-repeat;
    }
</style>

<section class="container">
    <section class="banner-holder rounded-3">
        <div class="container m-5 p-5">
            <div class="row">
                <div class="col-8 text-light">
                    <div class="container mt-5 mb-5">
                        <span class="badge text-bg-warning fw-bold shadow-sm">20% de descuento en su primera compra</span>
                        <h1 class="fw-bold mt-2 mb-2">Marketplace de UpVent</h1>
                        <p class="h6 mb-5">Introduzca la innovación <br> y el software libre su empresa por un precio increíble.</p>
                    </div>
                    <div class="container mt-5 mb-5">
                        <!-- svelte-ignore a11y-missing-content -->
                        <a href="#populares" class="mt-5 mb-5 btn btn-lg btn-dark shadow-sm">Comprar Ahora <ArrowRight/></a>
                    </div>
                </div>
                <div class="col-4">

                </div>
            </div>
        </div>
    </section>



    <div class="container" id="populares">
        <p class="h2 mt-3 mb-3">Productos populares</p>
        <div class="row row-cols-3">

        </div>
    </div>
</section>