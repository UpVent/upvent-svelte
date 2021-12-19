<svelte:head>

</svelte:head>

<script>
    import Navbar from './Navbar.svelte';

	export let name;
</script>

<Navbar/>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>
