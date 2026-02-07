<script lang="ts">
    // Explore Models
    import ModelsMansonary from "$lib/composables/models_mansonary.svelte"
    import { onMount } from "svelte";
    
    import { portal } from "svelte-portal";
    
    import ModelFilters from "$lib/filters/model_filters.svelte"
    
    import { type FilterOption } from "$lib/filter";
    import {model_filters} from "$lib/filters/model"
    
    let filters_state = $state<FilterOption>({...model_filters})
    
    let ready = $state(false);
    
    onMount(() => {
      ready = true;
    });
    
</script>


{#if ready}    
    <div use:portal={".header-content-nav"}>
        <ModelFilters bind:filters_state />
    </div>
{/if}

<ModelsMansonary filters={filters_state} />