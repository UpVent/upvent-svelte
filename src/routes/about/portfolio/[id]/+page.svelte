<script lang="ts">
 /* Svelte / Internal imports */
 import { page } from "$app/stores";
 import { fapi_url } from "$lib/common/settings";
 import { getTables } from "$lib/common/pbwrap";
 import SEO from "$lib/SEO.svelte";
</script>

<h1>La concha de su madre, estamos en el proyecto de id: {$page.params.id} </h1>
