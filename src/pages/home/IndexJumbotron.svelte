<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { slide } from 'svelte/transition';
    import { sineIn } from 'svelte/easing';

    // Third party imports
    import BagFill from 'svelte-bootstrap-icons/lib/BagFill.svelte';
    import ArrowRight from 'svelte-bootstrap-icons/lib/ArrowRight.svelte';

    // UpVent Logo
    import logo from '../../assets/images/upvent-main.webp';

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
        height: 500px;
    }

    .text-glow-white { text-shadow: 1px 1px 1px #010101; }

    .upvent-description {
        background: rgba( 255, 255, 255, 0.35 );
        box-shadow: 0 8px 32px 0 rgba( 31, 38, 135, 0.37 );
        -webkit-backdrop-filter: blur( 5.5px );
        backdrop-filter: blur( 5.5px );
        border-radius: 10px;
        border: 1px solid rgba( 255, 255, 255, 0.18 );
    }
</style>

<section class="pt-4 mb-4 pt-md-11 waves">
    <div class="container">
        <div class="row">
            <div class="col-12 col-md-5 col-lg-6 order-md-2 d-none d-sm-block mt-xs-0">
                {#if logo_visible}
                    <img transition:slide="{{delay: 250, duration: 200, easing: sineIn }}" src="{logo}" height="270" width="270" class="shadow rounded-circle mb-6 mb-md-0 " alt="logo" loading="lazy">
                {/if}
            </div>
            <div class="col-12 col-md-7 col-lg-6 upvent-description order-md-1">
                <h1 class="display-4 fw-bold text-center text-md-start">
                    Nube <span class="text-light text-glow-white">inteligente.</span>
                    <br>
                    Para negocios inteligentes.
                </h1>

                <p class="text-dark text-center text-md-start mb-6 mb-lg-8">
                    Impulse la innovación en su negocio con soluciones sencillas, accesibles y éticas de software libre. Conozca por que las pequeñas y medianas empresas Mexicanas confían en UpVent.
                </p>
                
            <div class="text-center text-md-start">
              <a href="/pwa" class="btn btn-primary mx-auto my-2 btn-lg me-1">Comprar ahora <BagFill/></a>
              <a href="/about" class="btn btn-outline-primary mx-auto my-2 btn-lg me-1">Portafolio de Trabajo <ArrowRight/></a>
            </div>
          </div>
        </div>
    </div>
</section>
