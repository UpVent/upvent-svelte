<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // slug to get post
    import { meta } from 'tinro';
    const route = meta();
    const slug = route.params.slug;

    let data = [];

    // Get API blog post
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?slug=" + slug;

    // Get post data
    onMount(async () => {
        const response = await fetch(api_url);
        const data_array = await response.json();
        console.log(data_array);
        data = data_array[0];
    });
</script>

<h1>Test</h1>
