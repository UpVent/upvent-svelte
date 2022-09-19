<script lang="ts">
    // Svelte imports
    import { onMount } from 'svelte';
    import { fapi_url, api_user, api_user_pass } from '../../config';

    /** Database Imports */
    import PocketBase from 'pocketbase';
    import type { Record } from 'pocketbase';
 
    // Database usage
    const client: PocketBase = new PocketBase(fapi_url);
    let records: Record[] = [];

    onMount(async () => {
        const user_auth_data = await client.users.authViaEmail(api_user, api_user_pass);
        records = await client.records.getFullList('politica_privacidad', 1);
        client.authStore.clear();
    });

    function format_date(date: string): string { return new Date(date).toLocaleDateString('es-MX'); }
</script>

<section class="container">
    {#await onMount}
        <p class="lead">Cargando política de privacidad...</p>
    {:then} 
    {#each records as record}
        <h1>{record.titulo}</h1>
        <span class="badge rounded-pill text-bg-primary">Última revisión: { format_date(record.ultima_revision) }</span>
        <hr>
        <section class="container"> {@html record.contenido } </section>
    {/each}
    {/await}
</section>
