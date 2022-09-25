<script lang="ts">
    export let component: any;
</script>

{#await component.then ? component : component()}
    Cargando...
{:then Cmp}
   <svelte:component this={Cmp.default} />
{/await}