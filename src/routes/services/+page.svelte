<script lang="ts">
 /* Svelte / Internal imports */
 import { fapi_url } from '$lib/common/settings';
 import { getTables } from '$lib/common/pbwrap';
 import SEO from '$lib/SEO.svelte';

 /* Asset imports */
 import responsive from '$lib/assets/responsive.svg';
 import ecommerce from '$lib/assets/ecommerce.svg';
 import oneplace from '$lib/assets/oneplace.webp';

 /* External imports */
 import LightningCharge from 'svelte-bootstrap-icons/lib/LightningCharge.svelte';
 import Brush from 'svelte-bootstrap-icons/lib/Brush.svelte';
 import PersonHearts from 'svelte-bootstrap-icons/lib/PersonHearts.svelte';
 import Laptop from 'svelte-bootstrap-icons/lib/Laptop.svelte';
 import CodeSlash from 'svelte-bootstrap-icons/lib/CodeSlash.svelte';
 import Award from 'svelte-bootstrap-icons/lib/Award.svelte';
 import Download from 'svelte-bootstrap-icons/lib/Download.svelte';

 import PocketBase from "pocketbase";
 import type { Record } from "pocketbase";
 /* Logic */

 const pb: PocketBase = new PocketBase(fapi_url);
 const records: Promise<Record[]> = getTables(pb, 'proyectos_libres');
</script>

<style>
    .jumbotron {
        background-image: url('/src/lib/assets/core-services.webp');
        background-size: cover;
        height: 500px;
    }

    .upvent-background { background-color: #007BFC; }
</style>

<SEO
    title="Servicios | UpVent - Soluciones en la nube para tu negocio. Low cost, siempre listas."
    description="Conoce nuestras soluciones de código libre para tu negocio."
    canonical = "https://upvent.codes/services"
/>

<section class="p-5 text-center bg-image jumbotron">
    <div class="d-flex justify-content-center align-items-center m-5 h-100">
        <div class="text-white text-center">
            <h1 class="display-3 fw-bold mb-3">Soluciones únicas, para problemas únicos.</h1>
            <p class="h1 text-wrap fs-5 mb-3">Ofrecemos soluciones para todo tipo de negocios. No importa el tamaño ¡Nosotros nos encargamos!</p>
        </div>
    </div>
</section>

<section class="container mt-5 mb-5">
    <div class="row text-right align-items-center">
        <div class="col-12 col-lg-6 col-xl-5 m-lg-auto text-left">
            <h1 class="text-center fw-bold display-3">PWA's</h1>
            <h6 class="text-center">(Aplicaciones Web Progresivas)</h6>
            <div class="row pt-5">
                <div class="col-3"> <Brush width={90} height={90}/> </div>
                <div class="col-9">
                    <h2>Desarrollo Personalizado</h2>
                    <p class="text-muted text-wrap">
                        Desarrollamos su aplicación web de acuerdo a sus necesidades, sea una pequeña empresa, freelancer o una corporación.<br/>
                        UpVent estará ahí para ofrecerle una solución única.
                    </p>
                </div>
            </div>

            <div class="row pt-5">
                <div class="col-9 text-right">
                    <h2>Velocidad y Seguridad</h2>
                    <p class="text-muted text-wrap">
                        Creamos aplicaciones web veloces y disponibles incluso sin internet, todo esto utilizando una cuidadosa mezcla de tecnologías propias y tecnologías libres.<br />
                        Gracias a la tecnología de UpVent podemos ofrecerle sitios seguros y con un tiempo de carga casi instantáneo.
                    </p>
                </div>
                <div class="col-3"><LightningCharge width={90} height={90}/></div>
            </div>
        </div>
        <div class="col-7 col-sm-4 m-auto pt-5 pt-md-0">
            <img alt="Imágen mostrando una computadora" class="img-fluid" src="{responsive}">
        </div>
    </div>
</section>

<section class="container mt-5 mb-5">
    <div class="row align-items-center">
        <div class="col-12 col-md-6 mb-4 mb-md-0">
            <img alt="Imágen mostrando un e-commerce en funcionamiento" class="img-fluid" src="{ecommerce}">
        </div>
        <div class="col-12 col-md-6 col-lg-5 ml-md-auto text-left">
            <h1 class="fw-bold display-3">Mejora de sitios</h1>
            <p class="lead">Si ya tiene un sitio web y desea mejorarlo tenemos la solución ideal para usted. Con UpVent podrá elegir entre convertirlo a una PWA o aumentar la velocidad de su sitio.</p>
        </div>
    </div>
</section>

<section class="container-fluid upvent-background text-light p-4 mt-2 mb-2">
    <div class="container">
        <div class="row">
            <div class="col text-center">
                <div class="container p-2">
                    <PersonHearts height={48} width={48}/>
                    <p class="h2 fw-bold">+21</p>
                    <hr>
                    Clientes Satisfechos.
                </div>
            </div>

            <div class="col text-center">
                <div class="container p-2">
                    <Laptop height={48} width={48}/>
                    <p class="h2 fw-bold">+11</p>
                    <hr>
                    Páginas Web Realizadas.
                </div>
            </div>

            <div class="col text-center">
                <div class="container p-2">
                    <CodeSlash height={48} width={48}/>
                    <p class="h2 fw-bold">+1,023,215</p>
                    <hr>
                    De líneas de código libre.
                </div>
            </div>

            <div class="col text-center">
                <div class="container p-2">
                    <Award height={48} width={48}/>
                    <p class="h2 fw-bold">+5</p>
                    <hr>
                    Casos de éxito en México.
                </div>
            </div>
        </div>
    </div>
</section>
<section class="container">
    <div class="container text-center mt-5 mb-5">
        <h2 class="fw-bold display-3">Software <span class="text-primary">libre</span> al alcance de su mano.</h2>
        <p class="text-muted"> Conozca los proyectos de Software Libre que UpVent ha preparado para ustedy su empresa.</p>
    </div>
    <div class="container">
        <div class="row row-cols-1 row-cols-sm-2 row-cols-md-3 g-3">
            {#await records}
                <p>Cargando proyectos libres...</p>
            {:then data}
            {#each data as record}
                <div class="col">
                    <figure>
                        <div class="card h-75 position-relative border-0 shadow-sm-sm p-2">
                            <img height="100" width="100" class="img-fluid m-1 shadow-sm-md rounded-circle" src="{pb.getFileUrl(record, record.imagen)}" alt="Producto de software libre">
                            <p class="lead fw-bold">{record.nombre}</p>
                        </div>
                        <blockquote>
                            <div class="container">
                                <p class="card-text text-muted text-wrap">{record.descripcion}</p>
                                <a class="btn btn-primary" href="{record.enlace}">Descargar <Download/></a>
                            </div>
                        </blockquote>
                    </figure>
                </div>
            {/each}
            {/await}
    </div>
</section>

<section class="container mt-5 py-16 py-sm-24 py-lg-32">
    <div class="mx-auto max-w-md px-4 text-center">
        <h2 class="text-primary">SOFTWARE ESTABLE, LISTO PARA USARSE</h2>
        <p class="mt-2 text-3lx text-muted">UpVent construye software eficiente desde el primer día, listo para uso empresarial y proporcionando una estabilidad sólida con actualizaciones periódicas gratuitas.</p>
    </div>
    <div class="container"><img class="img-fluid" src={oneplace} alt=""/></div>
    <p class="text-center display-5 text-primary fw-bold">¡Todo en un solo lugar!</p>
    <p class="text-center lead">Ofrecemos una variedad de servicios y software pre-hecho. ¿Necesita de un CRM, un e-commerce o una solución personalizada? Para UpVent no hay obstáculos, ¡Lo tenemos cubierto en cualquier necesidad tecnológica!</p>
</section>
