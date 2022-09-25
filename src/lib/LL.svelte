<script lang="ts">
    export let component: any;
</script>

{#await component.then ? component : component()}
    <span class="placeholder col-7"></span><span class="placeholder col-7"></span><span class="placeholder col-4"></span><span class="placeholder col-8"></span>
{:then Cmp}
   <svelte:component this={Cmp.default} />
{/await}