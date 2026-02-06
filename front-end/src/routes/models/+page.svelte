<script lang="ts">
    // Explore Models
    import { onMount } from "svelte";
    
    let ready = $state(false);
    
    onMount(() => {
      ready = true;
    });
    
</script>


{#if ready}    
    ready
{/if}

