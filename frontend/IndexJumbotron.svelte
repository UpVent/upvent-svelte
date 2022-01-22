<script>
    // Sveltestrap imports
    import {
        Button
    } from 'sveltestrap';

    // Svelte imports
    import { onMount } from 'svelte';
    import { slide } from 'svelte/transition';
    import { sineIn } from 'svelte/easing';

    // Third party imports
    import ArrowRight from 'svelte-bootstrap-icons/lib/ArrowRight';

    // Component variables
    let logo_section = 'images/upvent-logo-new.png';
    let logo_visible = false;

    // Component functions
    onMount(() => logo_visible = true);
</script>

<style>
    .bg-light-grey {
        background-color: rgb(249, 250, 251);
    }
</style>

<section class="pt-4 mt-4 mb-4 pt-md-11">
    <div class="container">
        <div class="row">
            <div class="col-12 col-md-5 col-lg-6 order-md-2">
                {#if logo_visible}
                    <img transition:slide="{{delay: 250, duration: 200, easing: sineIn }}" src={logo_section} height="250" width="250" class="img-fluid shadow rounded-circle mb-6 mb-md-0" alt="logo">
                {/if}
            </div>
            <div class="col-12 col-md-7 col-lg-6 order-md-1">
                <h1 class="display-4 fw-bold text-center text-md-start">
                    Nube inteligente.
                    <br>
                    Para negocios inteligentes.
                </h1>

                <p class="text-muted text-center text-md-start fw-bold mb-6 mb-lg-8">
                    Impulse la innovación en su negocio con soluciones sencillas, accesibles y éticas de software libre. Conozca por que las micro, pequeñas y medianas empresas Mexicanas confían en UpVent.
                </p>
            <div class="text-center text-md-start">
              <a href="#" class="btn btn-primary mx-auto my-2 btn-lg me-1">
                  Conocer más <ArrowRight/>
              </a>
            </div>
          </div>
        </div>
    </div>
</section>
