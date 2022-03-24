<script>
    // Svelte imports
    import { onMount } from 'svelte';

    // ID to get post
    export let id;
    let data = [];

    // Get API blog post
    const api_url = "https://wpapi.upvent.codes/wp-json/wp/v2/posts?slug=" + id;

    // Get post data
    onMount(async () => {
        const response = await fetch(api_url);
        const data_array = await response.json();
        data = data_array[0];
    });
</script>


