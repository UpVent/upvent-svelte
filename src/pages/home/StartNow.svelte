<script lang="ts">
    import WrenchAdjustableCircle from 'svelte-bootstrap-icons/lib/WrenchAdjustableCircle.svelte';
    import logo from '../../assets/images/upvent-main.webp';
</script>

<section class="px-4 py-5 my-5 text-center">
    <img class="d-block mx-auto mb-4 rounded-circle shadow-sm" src="{logo}" alt="Logo de UpVent circular" width="100" height="100" loading="lazy"/>
    <p class="h1 display-5 fw-bold">Comenzar a trabajar con <span class="h1 display-5 fw-bold text-primary">UpVent.</span></p>
    <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">Potencie su negocio ahora mismo, mire nuestro portafolio de trabajo o visite nuestra tienda de servicios para comenzar hoy mismo.</p>
        <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <a href="/pwa" class="btn btn-primary btn-lg px-4 me-sm-3">Comprar Ahora</a>
            <a href="/about" class="btn btn-outline-primary fw-bold btn-lg px-4">Ver Portafolio</a>
        </div>
    </div>
</section>

<section class="text-center">
    <WrenchAdjustableCircle height={48} width={48} class="text-primary"/>
    <p class="h2 fw-bold m-2">Tecnología que nos inspira</p>
    <div class="container mt-3 mb-2">
        <div class="container" id="startnowtechs">
            <div class="row row-cols-1 row-cols-sm-2 row-cols-md-4">
                <div class="col">
                    <a rel="noopener noreferrer" href="https://svelte.dev">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 32 32"><path fill="#ff3e00" d="M26.5 5.7a9 9 0 0 0-11.8-2.5L8 7.4a7.5 7.5 0 0 0-3.5 5 7.7 7.7 0 0 0 .8 5 7.4 7.4 0 0 0-1.3 3 7.8 7.8 0 0 0 1.4 6 9 9 0 0 0 11.8 2.5l6.7-4.2a7.5 7.5 0 0 0 3.5-5 7.7 7.7 0 0 0-.8-5 7.3 7.3 0 0 0 1.2-2.9 7.8 7.8 0 0 0-1.4-6"/><path fill="#fff" d="M14 26.6a5.4 5.4 0 0 1-5.7-2 4.7 4.7 0 0 1-.8-3.6 4.3 4.3 0 0 1 .1-.7l.1-.3.4.2a8.8 8.8 0 0 0 2.6 1.3h.3v.3a1.4 1.4 0 0 0 .2 1 1.6 1.6 0 0 0 1.7.6 1.5 1.5 0 0 0 .5-.2l6.7-4.1a1.4 1.4 0 0 0 .6-1 1.4 1.4 0 0 0-.3-1 1.6 1.6 0 0 0-1.7-.7 1.5 1.5 0 0 0-.4.2l-2.6 1.6a5 5 0 0 1-1.3.6 5.4 5.4 0 0 1-5.8-2 4.7 4.7 0 0 1-.8-3.7 4.5 4.5 0 0 1 2.1-3l6.7-4.2a5 5 0 0 1 1.4-.5 5.4 5.4 0 0 1 5.7 2 4.7 4.7 0 0 1 .8 3.6 4.3 4.3 0 0 1-.1.6l-.1.4-.4-.2a8.7 8.7 0 0 0-2.6-1.3H21v-.3a1.4 1.4 0 0 0-.2-1 1.6 1.6 0 0 0-1.7-.6 1.5 1.5 0 0 0-.4.2l-6.8 4.1a1.4 1.4 0 0 0-.6 1 1.4 1.4 0 0 0 .3 1 1.6 1.6 0 0 0 1.7.6 1.5 1.5 0 0 0 .4-.1l2.6-1.6a4.9 4.9 0 0 1 1.3-.6 5.4 5.4 0 0 1 5.7 2 4.7 4.7 0 0 1 .9 3.7 4.5 4.5 0 0 1-2.1 3l-6.7 4.2a4.9 4.9 0 0 1-1.4.5"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a rel="noopener noreferrer" href="https://rust-lang.org">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 32 32"><path fill="#050505" d="M15.1 5.3a.8.8 0 1 1 .9.8.8.8 0 0 1-.9-.8m-9.9 7.5a.8.8 0 1 1 .8.9.8.8 0 0 1-.8-.9m19.9 0a.8.8 0 1 1 .8.9.8.8 0 0 1-.8-.8M7.6 14a.8.8 0 0 0 .4-1l-.4-.8h1.5v6.5H6a10.2 10.2 0 0 1-.3-3.9Zm6 .2v-2h3.5c.2 0 1.3.3 1.3 1s-.8 1-1.5 1ZM9 24.6a.8.8 0 1 1 .8.8.8.8 0 0 1-.8-.8m12.3 0a.8.8 0 1 1 .8.8.8.8 0 0 1-.8-.8m.2-1.9a.8.8 0 0 0-.9.6l-.4 2a10.2 10.2 0 0 1-8.5 0l-.4-2a.8.8 0 0 0-1-.6l-1.7.3a10.2 10.2 0 0 1-.8-1H16l.2-.1v-3h-2.6v-2h2.6a1.7 1.7 0 0 1 1.7 1.5c0 .4.3 1.7.5 2.1.1.5.8 1.5 1.4 1.5h4.4a10.2 10.2 0 0 1-1 1Zm4.7-7.8a10.3 10.3 0 0 1 0 1.8h-1l-.2.1v.5c0 1.2-.6 1.4-1.2 1.5-.5 0-1.1-.3-1.2-.6a3.6 3.6 0 0 0-1.7-2.8c1-.6 2.1-1.6 2.1-2.9a3.3 3.3 0 0 0-1.6-2.7 4.6 4.6 0 0 0-2.2-.7H8.4A10.2 10.2 0 0 1 14 5.8l1.3 1.4a.8.8 0 0 0 1 0L18 5.8a10.2 10.2 0 0 1 7 5l-1 2.3a.8.8 0 0 0 .4 1Zm2.4 0v-.3l1-1a.4.4 0 0 0 0-.6.4.4 0 0 0-.1 0l-1.3-.5-.1-.4.8-1a.4.4 0 0 0-.1-.7.4.4 0 0 0-.2 0l-1.3-.2-.2-.3.6-1.3a.4.4 0 0 0-.2-.5.4.4 0 0 0-.2 0H26l-.2-.3.3-1.3a.4.4 0 0 0-.3-.5.4.4 0 0 0-.2 0l-1.3.3-.3-.2V4.7a.4.4 0 0 0-.4-.4.4.4 0 0 0-.1 0L22 5l-.3-.2-.2-1.3a.4.4 0 0 0-.5-.4.4.4 0 0 0-.1.1l-1.1.8h-.4L19 2.4a.4.4 0 0 0-.5-.2.4.4 0 0 0-.1 0l-1 1h-.3l-.8-1.1a.4.4 0 0 0-.6 0l-.8 1.2h-.3l-1-1a.4.4 0 0 0-.5 0 .4.4 0 0 0-.1.1l-.5 1.3-.3.1L11 3a.4.4 0 0 0-.6.1.4.4 0 0 0 0 .2l-.2 1.3-.3.2-1.3-.6a.4.4 0 0 0-.5.3.4.4 0 0 0 0 .1V6l-.3.2L6.5 6a.4.4 0 0 0-.5.3.4.4 0 0 0 0 .2l.3 1.3-.2.3H4.7a.4.4 0 0 0-.4.5L5 10l-.2.3-1.3.2a.4.4 0 0 0-.4.5.4.4 0 0 0 .1.1l.8 1.1v.4l-1.4.5a.4.4 0 0 0-.2.5.4.4 0 0 0 0 .1l1 1v.3l-1.1.8a.4.4 0 0 0 0 .6l1.2.8v.3l-1 1a.4.4 0 0 0 0 .6.4.4 0 0 0 .1 0l1.3.5.1.3-.8 1.2a.4.4 0 0 0 .1.6.4.4 0 0 0 .2 0l1.3.2.2.3-.6 1.3a.4.4 0 0 0 .3.5.4.4 0 0 0 .1 0h1.4l.2.3-.3 1.3a.4.4 0 0 0 .3.5.4.4 0 0 0 .2 0l1.3-.3.3.2v1.4a.4.4 0 0 0 .4.4.4.4 0 0 0 .1 0L10 27l.3.2.2 1.3a.4.4 0 0 0 .5.4.4.4 0 0 0 .1-.1l1.1-.8h.4l.5 1.4a.4.4 0 0 0 .5.2.4.4 0 0 0 .1 0l1-1h.3l.8 1.1a.4.4 0 0 0 .5.1.4.4 0 0 0 .1 0l.8-1.3h.3l1 1a.4.4 0 0 0 .5 0 .4.4 0 0 0 .1-.1l.5-1.3.3-.1 1.2.8a.4.4 0 0 0 .5-.1.4.4 0 0 0 .1-.2l.2-1.3.3-.2 1.3.6a.4.4 0 0 0 .5-.3.4.4 0 0 0 0-.1v-1.4l.3-.2 1.3.3a.4.4 0 0 0 .5-.3.4.4 0 0 0 0-.2l-.3-1.3.2-.3h1.4a.4.4 0 0 0 .4-.4.4.4 0 0 0 0-.1L27 22l.2-.3 1.3-.2a.4.4 0 0 0 .4-.5.4.4 0 0 0-.1-.1l-.8-1.1v-.4l1.4-.5a.4.4 0 0 0 .2-.5.4.4 0 0 0 0-.1l-1-1V17l1.1-.8a.4.4 0 0 0 0-.6Z"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a rel="noopener noreferrer" href="https://rockylinux.org">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 256 256"><path fill="#10B981" d="M249 170a128 128 0 1 0-212 48L167 88l32 32 50 50Zm-24 41-58-59-93 92a128 128 0 0 0 151-33Z"/></svg>
                    </a>
                </div>
                <div class="col">
                    <a rel="noopener noreferrer" href="https://getbootstrap.com/">
                        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="39.9" viewBox="0 0 256 204"><path fill="#7E13F8" d="M53 0C39 0 28 13 28 27c1 13 0 30-4 44-5 14-12 23-24 24v13c12 2 19 11 24 25 4 14 5 31 4 44 0 14 11 27 25 27h150c14 0 25-13 25-27-1-13 0-30 4-44 5-14 12-23 24-25V95c-12-1-19-10-24-24-4-14-5-31-4-44 0-14-11-27-25-27H53Zm121 126c0 19-15 30-38 30H95a4 4 0 0 1-4-4V52a4 4 0 0 1 4-4h40c20 0 33 10 33 27 0 11-9 22-20 23v1c15 2 26 12 26 27Zm-43-65h-23v33h20c15 0 23-6 23-17 0-10-7-16-20-16Zm-23 46v35h24c16 0 24-6 24-18 0-11-9-17-25-17h-23Z"/></svg>
                    </a>
                </div>
            </div>
        </div>
    </div>
</section>
