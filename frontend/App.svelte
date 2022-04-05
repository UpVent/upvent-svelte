<script>
    // External imports
    import {
        Route,
        router
    } from 'tinro';

    // Page Imports
    import Home from './home/Home.svelte';
    import Blog from './blog/Blog.svelte';
    import Post from './blog/Post.svelte';
    import About from './about/About.svelte';
    import Services from './services/Services.svelte';
    import Contact from './contact/Contact.svelte';
    // Additional Pages
    import PrivacyPolicy from './home/PrivacyPolicy.svelte';
    import NotFound from './NotFound.svelte';

    // Component imports
    import Footer from './Footer.svelte';
    import Transition from './Transition.svelte';

    // Icon imports
    import House from 'svelte-bootstrap-icons/lib/House';
    import Laptop from 'svelte-bootstrap-icons/lib/Laptop';
    import Pen from 'svelte-bootstrap-icons/lib/Pen';
    import People from 'svelte-bootstrap-icons/lib/People';
    import Phone from 'svelte-bootstrap-icons/lib/Phone';

    /** Modal variable */
    let open = false;
    const toggle = () => (open = !open);

    /** Scroll to top after navigation */
    router.subscribe(_ => window.scrollTo(0, 0));

</script>

<!--
@component
     The App component is the main component for UpVent Svelte frontend.
     It's the root of the entire web application as it handles routes, manages
     404's and styles the main navigation bar.
-->

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
        <img class="mx-auto img-fluid" width="148" height="51" src="/images/logo-grey.webp" alt="UpVent Logo">
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

<!-- Home Page -->
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

    <Route fallback><NotFound/></Route>
</Transition>

<!-- Footer component -->
<Footer/>
