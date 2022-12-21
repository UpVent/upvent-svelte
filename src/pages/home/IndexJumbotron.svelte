<script lang="ts">
    // Third party imports
    import ArrowRight from 'svelte-bootstrap-icons/lib/ArrowRight.svelte';

    // UpVent Logo
    import logo from '../../assets/images/upvent-main.webp';
</script>

<style>
    .waves {
        background: url('../../assets/images/waves.webp');
        background-size: contain;
        background-repeat: repeat-x;
        height: 450px;
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

<section class="pt-4 mb-2 pt-md-11 waves">
    <div class="container">
        <div class="row">
            <div class="col-12 col-md-5 col-lg-6 order-md-2 d-none d-sm-block mt-xs-0">
                <img src="{logo}" height="270" width="270" class="shadow-sm rounded-circle mb-6 mb-md-0 " alt="logo" loading="lazy">
            </div>
            <div class="col-12 col-md-7 col-lg-6 upvent-description order-md-1">
                <h1 class="display-4 fw-bold text-center text-md-start">
                    Nube <span class="text-light text-glow-white">inteligente.</span>
                    <br>
                    Para negocios inteligentes.
                </h1>
                <p class="link-dark text-center text-md-start mb-6 mb-lg-8">
                    Impulse la innovación en su negocio con soluciones sencillas y éticas de software libre. Conozca por que las pequeñas y medianas empresas Mexicanas confían en UpVent.
                </p>
            <div class="text-center text-md-start">
              <a href="/about" class="btn btn-primary mx-auto my-2 btn-lg me-1">Portafolio de Trabajo <ArrowRight/></a>
            </div>
          </div>
        </div>
    </div>
</section>
