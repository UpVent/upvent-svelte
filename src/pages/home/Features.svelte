<svelte:options immutable={true}/>

<script>
    import Lazy from 'svelte-lazy';
    // Import images
    import universal from '../../assets/images/universal.svg';
    import secure from '../../assets/images/secure.svg';
    import escalable from '../../assets/images/escalable.svg';
    import rapido from '../../assets/images/rapido.svg';
</script>

<style>
    * {
        font-family: 'Poppins', sans-serif;
    }
</style>

<section class="pt-8 pt-md-10">
    <div class="container mt-5 mb-5">
        <div class="row justify-content-center">
            <div class="col-12 col-md-10 col-lg-8 text-center">
                <h2 class="fw-bold">
                    Soluciones en la nube <span class="text-primary text-glow">para todo tipo de negocios</span>.
                </h2>

                <p class="fs-5 text-muted mb-9">
                    UpVent es una empresa de soluciones creadas con software libre para
                    su proyecto, empresa o equipo de trabajo. Soluciones, Infraestructura. ¡Lo tenemos cubierto!
                </p>
            </div>
        </div>
        <div class="row">
            <div class="col-12 col-lg-6">
                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <Lazy height={189}>
                            <img width="216" height="189" src="{universal}" alt="Gráfico mostrando a una persona junto a un teléfono celular enorme" class="mt-1 img-fluid">
                        </Lazy>
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">
                            Universales
                        </h3>
                        <p class="text-muted mb-0">
                            Haga uso de su solución personalizada en computadoras, tabletas y teléfonos.
                        </p>
                    </div>
                </div>

                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5">
                        <Lazy height={154}>
                            <img width="216" height="154" src="{secure}" alt="..." class="mt-1 img-fluid">
                        </Lazy>
                    </div>
                    <div class="col-8 col-lg-7 mt-2 mb-2">
                        <h3 class="fw-bold mb-2">
                            Seguros
                        </h3>
                        <p class="text-muted mb-0">
                            Su solución empresarial estará programada para ser segura desde el inicio.
                        </p>
                    </div>
                </div>
            </div>

            <div class="col-12 col-lg-6">
                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <Lazy height={137}>
                            <img width="216" height="137" src="{escalable}" alt="..." class="mb-1 img-fluid">
                        </Lazy>
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">
                            Escalables
                        </h3>
                        <p class="text-muted mb-0">
                            Manténgase siempre a la vanguardia con software escalable desde el primer día.
                        </p>
                    </div>
                </div>

                <div class="row align-items-center mb-8">
                    <div class="col-4 col-lg-5 mt-2 mb-2">
                        <Lazy height={169}>
                            <img width="216" height="169" src="{rapido}" alt="..." class="mt-1 img-fluid">
                        </Lazy>
                    </div>
                    <div class="col-8 col-lg-7">
                        <h3 class="fw-bold mb-2">
                            Rápidos
                        </h3>
                        <p class="text-muted mb-0">
                            Programación cautelosa de los productos desde su raíz. ¡No más software lento!
                        </p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</section>
