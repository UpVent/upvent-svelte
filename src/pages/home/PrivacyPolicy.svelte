<svelte:options immutable={true}/>

<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';

    // Get API pages
    const api_url: string = "https://wpapi.upvent.codes/wp-json/wp/v2/pages/38";

    // Page properties
    let page: any;
    let page_title: string;
    let page_content: string;

    // Fetch current page info
    onMount(async () => {
        // Page request
        const page_res: Response = await fetch(api_url);
        const page_json: Response = await page_res.json(); 
        page = page_json;
        page_title = page.title.rendered;
        page_content = page.content.rendered;
    });
</script>

<style>
    * {
        font-family: 'Poppins', sans-serif;
    }
</style>

<section class="container">
    {#await onMount}
        <p class="lead">Cargando política de privacidad...</p>
    {:then data} 
        <h1>{page_title}</h1>
        <hr>
        <section class="container">
            {@html page_content}
        </section>
    {/await}
</section>