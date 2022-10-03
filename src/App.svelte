<script lang="ts">
    // Import router
    import { Route, router, active } from 'tinro';

    // Import lib components
    import Transition from './lib/Transition.svelte';
    import LL from './lib/LL.svelte';

    // Navbar logo
    import logo from './assets/images/logo-grey.webp';

    // Import page components
    import Home from './pages/home/Home.svelte';
    import Blog from './pages/blog/Blog.svelte';
    import Post from './pages/blog/Post.svelte';

    import Footer from './pages/Footer.svelte';
    
    // Scroll to top after navigation
    router.subscribe(_ => window.scrollTo(0, 0));
</script>

<style>
    :global(*) { font-family: 'Poppins'; }

    @supports (backdrop-filter: none) {
        .navbar-upvent {
            background-color:rgba(254, 245, 254, 0.6);
            -webkit-backdrop-filter: blur(20px);
            backdrop-filter: blur(20px);
        }
    }

    @supports not (backdrop-filter: none) {
        .navbar-upvent { background-color: #FAFAFA; }
    }

    #nav-upvent a:global(.active-nav) {
        color: #FAFAFA !important; 
        background-color: #007BFC;
    }
</style>

<!-- Navbar component -->
<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-2 mb-0 sticky-md-top navbar-upvent">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
        <img class="mx-auto" width="148" height="51" src="{logo}" alt="UpVent Logo">
    </a>

    <ul id="nav-upvent" class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="fs-5 nav-link px-2 link-dark" href="/" use:active active-class="active-nav" exact>Inicio</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/blog" use:active active-class="active-nav">Blog</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/about" use:active active-class="active-nav">Nosotros</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/services" use:active active-class="active-nav">Servicios</a></li>
        <li><a class="fs-5 nav-link px-2 link-dark" href="/pwa" use:active active-class="active-nav">Store</a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <a class="btn btn-primary btn-lg" href="/contact" >Contacto</a>
    </div>
</header>

<!-- Main route handling -->
<Transition>
    <Route path="/"><Home/></Route>

    <!-- Blog Routes -->
    <Route path="/blog/*">
        <Route path="/"><Blog/></Route>
        <Route path="/post/:id" let:meta><Post/></Route>
    </Route>

    <Route path="/about">
        <LL component={() => import('./pages/about/About.svelte')} />
    </Route>

    <Route path="/services">
        <LL component={() => import('./pages/services/Services.svelte')} />
    </Route>

    <Route path="/pwa">
        <LL component={() => import('./pages/pwa/PWA.svelte')} />
    </Route>

    <!-- Footer links -->
    <Route path="/privacy-policy">
        <LL component={() => import('./pages/home/PrivacyPolicy.svelte')} />
    </Route>
    <Route path="/contact">
        <LL component={() => import('./pages/contact/Contact.svelte')} />
    </Route>

    <!-- Custom 404 -->
    <Route fallback>
        <LL component={() => import('./lib/NotFound.svelte')} />
    </Route>
</Transition>

<!-- Always show the Footer component -->
<Footer/>
