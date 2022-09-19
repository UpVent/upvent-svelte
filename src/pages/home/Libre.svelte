<script>
    import Lazy from 'svelte-lazy';
    import { ArrowRight } from 'svelte-bootstrap-icons';
    // Import gnu logo
    import gnu from '../../assets/images/gnu.webp';
</script>

<section class="container mt-5 mb-5">
    <div class="container text-center">
        <Lazy height={150}>
            <img class="d-block mx-auto mb-4" src="{gnu}" alt="Logo de UpVent circular" width="150" height="150"/>
        </Lazy>
        <p class="h2 fw-bold">Todo lo que hacemos es de</p> <span class="h2 fw-bold text-glow text-primary">código libre</span>
        <p class="text-center text-wrap lead">
                Nuestro modelo de desarrollo consiste totalmente de componentes de código
                libre y se basa en múltiples principios éticos y prácticos para su conveniencia como empresa.<br />
            Además, colaboramos de manera activa con el ecosistema de software libre en México para mejorar dichas tecnologías para todos.
        </p>
        <a href="#startnowtechs">Explore las tecnologías usadas en UpVent <ArrowRight/></a>
    </div>
</section>
