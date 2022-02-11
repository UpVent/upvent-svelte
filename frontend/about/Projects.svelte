<script>
    // Get API projects
    const api_url = 'http://127.0.0.1:8000/api/projects';
    // Get api info
    const fetchProjects = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()

    const placeholder = 'images/avatar.png';
</script>

<style>

</style>

<section class="container mt-5 mb-5">
    <div class="container text-center">
        <h1>Portafolio de Trabajo</h1>
        <p class="lead text-muted">
            Conozca los trabajos de UpVent y la conformidad de sus clientes con los mismos. Además, también podrá ver nuestros proyectos de código libre en favor de la comunidad.
        </p>
    </div>
</section>

<section class="album py-5 bg-light">
    <div id="projects" class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await fetchProjects}
                <p class="text-muted lead">Cargando los proyectos realizados por UpVent...</p>
            {:then data}
            {#each data.result as project}
                <div class="col">
                    <div class="card shadow-sm rounded-3">
                        <img class="card-img-top" alt="Imágen mostrando un proyecto creado por UpVent" src={placeholder}/>
                        <div class="card-body">
                            <h5 class="fs-2 fw-bold">{ project.title }</h5>
                            <p class="card-text">{ project.description }</p>
                            <div class="d-flex justify-content-between align-items-center">
                                <a class="btn btn-primary" href="{ project.site }">Visitar</a>
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
            {:catch error}
                <p>Error al obtener las últimas publicaciones del blog.</p>
            {/await}
        </div>
    </div>
</section>
