<svelte:options immutable={true}/>

<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { slide } from 'svelte/transition';
    import { sineIn } from 'svelte/easing';

    // Third party imports
    import { ArrowRight, BagFill } from 'svelte-bootstrap-icons';
    import Lazy from 'svelte-lazy';

    // UpVent Logo
    import logo from '../../assets/images/upvent-logo-new.webp';

    // Component variables
    let logo_visible: boolean = false;

    // Component functions
    onMount(() => logo_visible = true);
</script>

<style>
    .waves {
        background: url('../../assets/images/waves.webp');
        background-size: contain;
        background-repeat: repeat-x;
    }

    .text-glow-white {
        text-shadow: 1px 1px 1px #010101;
    }
</style>

<section class="pt-4 mb-4 pt-md-11 waves">
    <div class="container">
        <div class="row">
            <div class="col-12 col-md-5 col-lg-6 order-md-2 d-none d-sm-block mt-xs-0">
                {#if logo_visible}
                    <Lazy height={270}>
                        <img transition:slide="{{delay: 250, duration: 200, easing: sineIn }}" src="{logo}" height="270" width="270" class="img-fluid shadow rounded-circle mb-6 mb-md-0 " alt="logo">
                    </Lazy>
                {/if}
            </div>
            <div class="col-12 col-md-7 col-lg-6 order-md-1">
                <h1 class="display-4 fw-bold text-center text-md-start">
                    Nube <span class="text-light text-glow-white">inteligente.</span>
                    <br>
                    Para negocios inteligentes.
                </h1>

                <p class="text-muted text-center text-md-start fw-bold mb-6 mb-lg-8">
                    Impulse la innovación en su negocio con soluciones sencillas, accesibles y éticas de software libre. Conozca por que las micro, pequeñas y medianas empresas Mexicanas confían en UpVent.
                </p>
            <div class="text-center text-md-start">
              <a href="/about" class="btn btn-primary mx-auto my-2 btn-lg me-1">
                  Conocer más <ArrowRight/>
              </a>

              <a rel="nofollow" href="https://www.paypal.com/webapps/billing/plans/subscribe?plan_id=P-7TT42401B5144205WMKJSD3I" class="btn btn-primary mx-auto my-2 btn-lg me-1">
                  Suscribirse a UpVent <BagFill/>
              </a>
            </div>
          </div>
        </div>
    </div>
</section>
