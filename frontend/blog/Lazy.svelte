<script>
    export let component;
</script>

{#await component.then ? component : component()}
    Cargando...
{:then Cmp}
   <svelte:component this={Cmp.default} />
{/await}