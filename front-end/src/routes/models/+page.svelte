<script lang="ts">
    // Explore Models
    import ModelsMansonary from "$lib/composables/models_mansonary.svelte"
    import { onMount } from "svelte";
    
    let ready = $state(false);
    
    onMount(() => {
      ready = true;
    });
    
</script>


{#if ready}    
    <!--Add filters teleport later-->
{/if}

<ModelsMansonary filters={{}} />