<script>
    // External imports
    import Carousel from 'svelte-carousel';
    import { Card, CardBody, CardText } from 'sveltestrap';

    // Testimonials API URL
    const api_url = 'http://127.0.0.1:8000/api/testimonials';

    // Get api info
    const fetchTestimonials = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()

</script>

<section class="container mt-5 mb-5">
    <div class="container mb-5">
        <h2 class="text-center">Testimonios</h2>
        <p class="text-muted text-center">
            Nuestros clientes hablan por nosotros, conozca la opinión de los profesionales acerca del trabajo de UpVent.
        </p>
    </div>

    <div id="testimonials" class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
        {#await fetchTestimonials}
            <p class="text-muted small">Loading testimonials...</p>
        {:then data}
            <Carousel
                particlesToShow={3}
                particlesToScroll={2}
                autoplay
                autoplayDuration={2000}
            >
                {#each data.result as testimonial}
                    <div class="container-fluid">
                        <Card class="border border-0 shadow-sm rounded">
                            <img height="200" width="200" class="img-fluid rounded-circle p-2" src='images/avatar.png' alt="Imágen mostrando el retrato de una persona"/>
                            <CardBody>
                                <p class="h5 mt-2 fw-bold">{testimonial.name}</p>
                                <hr>
                                <CardText class="text-muted small text-wrap text-break">{testimonial.testimonial}</CardText>
                                <div class="d-flex justify-content-center align-items-center">
                                    <p><a class="btn btn-primary btn-sm bg-upvent text-uppercase text-decoration-none" href="{testimonial.website}">{testimonial.workplace}</a></p>
                                </div>
                            </CardBody>
                        </Card>
                    </div>
                {/each}
            </Carousel>
        {:catch error}
            <p class="text-danger">Error getting testimonials</p>
        {/await}
    </div>
</section>
