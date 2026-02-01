<script lang="ts">
    // Explore all
    import AssetMansonary from "$lib/composables/asset_mansonary.svelte"
    import { onMount } from "svelte";
    import { portal } from "svelte-portal";
        
    import MediaFilters from "$lib/filters/media_filters.svelte"
    
    import { type FilterOption } from "$lib/filter";
    import {media_filters} from "$lib/filters/media"
    
    let ready = $state(false);
    
    onMount(() => {
      ready = true;
    });
    
    let filters_state = $state<FilterOption>({...media_filters, mediaType: {selected: {name: "", value: ""}, options: []}})
</script>

<AssetMansonary filters={filters_state} />

{#if ready}    
    <div use:portal={".header-content-nav"}>
        <MediaFilters bind:filters_state />
    </div>
{/if}

