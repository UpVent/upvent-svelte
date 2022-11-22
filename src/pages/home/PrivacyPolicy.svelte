<script lang="ts">
    // Svelte imports
    import { onMount, onDestroy } from 'svelte';
    import { fapi_url, api_user, api_user_pass } from '../../config';
    import { api_result } from '../../stores/store';

    /** Database Imports */
    import PocketBase from 'pocketbase';

    // Database usage
    const pb: PocketBase = new PocketBase(fapi_url);

    onMount(async () => {
        pb.collection('users').authWithPassword(api_user, api_user_pass);
        $api_result = await pb.collection('politica_privacidad').getFullList(1);
        pb.authStore.clear();
    });

    onDestroy(() => { $api_result.length = 0; });

    function format_date(date: string): string { return new Date(date).toLocaleDateString('es-MX'); }
</script>

<section class="container">
    {#each $api_result as record}
        <h1>{record.titulo}</h1>
        <span class="badge rounded-pill text-bg-primary">Última revisión: { format_date(record.ultima_revision) }</span>
        <hr>
        <section class="container"> {@html record.contenido } </section>
    {/each}
</section>
