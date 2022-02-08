<script>

    // Get API blog posts
    const api_url = 'http://127.0.0.1:8000/api/posts';
    // Get api info
    const fetchPosts = (async () => {
        const response = await fetch(api_url)
        return await response.json()
    })()
</script>

<style>
    
</style>

<section class="container">
    <h1 class="text-center">Blog</h1>
    <p class="text-center text-muted">Visite el blog de UpVent y aprenda algo nuevo junto a nosotros.</p>
    <hr>

</section>

