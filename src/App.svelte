<script lang="ts">
    // Import router
    import { Route, router } from 'tinro';

    // Import lib components
    import Transition from './lib/Transition.svelte';
    import NotFound from './lib/NotFound.svelte';

    // Navbar logo
    import logo from './assets/images/logo-grey.webp';

    // Import page components
    import Home from './pages/home/Home.svelte';
    import Blog from './pages/blog/Blog.svelte';
    import Post from './pages/blog/Post.svelte';
    import About from './pages/about/About.svelte';
    import Services from './pages/services/Services.svelte';
    import Contact from './pages/contact/Contact.svelte';

    // Footer components
    import PrivacyPolicy from './pages/home/PrivacyPolicy.svelte';

    // Import navbar icons
    import { House,
    Laptop,
    Pen,
    People,
    Phone,
    Bag
    } from 'svelte-bootstrap-icons';

    // Import Page layouts + components
    import Footer from './pages/Footer.svelte';
    

    // Scroll to top after navigation
    router.subscribe(_ => window.scrollTo(0, 0));
</script>

<style>
    @supports (backdrop-filter: none) {
        .navbar-upvent {
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
        }
    }

    @supports not (backdrop-filter: none) {
        .navbar-upvent {
            background-color: #FAFAFA;
        }
    }
</style>

<!-- Navbar component -->
<header class="d-flex flex-wrap align-items-center justify-content-center justify-content-md-between py-2 mb-0 sticky-md-top navbar-upvent">
    <a href="/" class="d-flex align-items-center col-md-3 mb-2 mb-md-0 text-dark text-decoration-none navbar-brand">
        <img class="mx-auto img-fluid" width="148" height="51" src="{logo}" alt="UpVent Logo">
    </a>

    <ul class="nav nav-pills col-12 mx-auto col-md-auto mb-2 justify-content-center mb-md-0">
        <li><a class="fs-5 nav-link px-2 text-muted" href="/">Inicio <House/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/blog">Blog <Pen/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/about">Nosotros <People/></a></li>
        <li><a class="fs-5 nav-link px-2 text-muted" href="/services">Servicios <Laptop/></a></li>
    </ul>

    <div class="text-end me-auto ms-auto mx-auto">
        <a class="btn btn-primary btn-lg" href="/contact" >Contacto <Phone/></a>
    </div>
</header>

<!-- Main route handling -->
<Transition>
    <Route path="/">
        <Home/>
    </Route>

    <!-- Blog Routes -->
    <Route path="/blog/*">
        <Route path="/">
            <Blog/>
        </Route>

        <Route path="/post/:slug" let:meta>
            <Post/>
        </Route>

    </Route>

    <Route path="/about">
        <About/>
    </Route>

    <Route path="/services">
        <Services/>
    </Route>

    <!-- Footer links -->
    <Route path="/privacy-policy">
        <PrivacyPolicy/>
    </Route>

    <Route path="/contact">
        <Contact/>
    </Route>

    <!-- Custom 404 -->
    <Route fallback><NotFound/></Route>
</Transition>

<!-- Always show the Footer component -->
<Footer/>