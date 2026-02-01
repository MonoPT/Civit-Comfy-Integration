<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import imageLoader from "$lib/apis/images.svelte"
    import { RegularMasonryGrid  as MasonryGrid, Frame } from '@masonry-grid/svelte'
    
    import { type FilterOption, type Filter } from "$lib/filter";
    
    import { Skeleton } from "$lib/components/ui/skeleton/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    let assets_list: any[] = $state([])
    let applied_filters: Filter[] = []
    
    let is_loadings_assets = false;
    let show_loading_indicator = $state(false)

    const {filters}: {filters: FilterOption} = $props()
    
    $effect(() => {
      const filters_selected: Filter[] = []
      
      for (const key in filters) {
        const value = filters[key];
        
        const selected_name = value.selected.name
        const selected_value = value.selected.value
        
        filters_selected.push({name: key, value: selected_value})
      }

      
      applied_filters = filters_selected
      assets_list = [] // Resets asset list
      imageLoader.reset()
      load_assets_batch()
	});
    
    onMount(async () => {   
      assets_list = []      
      imageLoader.reset()
      await load_assets_batch();
      
      window.addEventListener("scroll", handle_scroll)
    })
    
    onDestroy(() => window.removeEventListener("scroll", handle_scroll))
    
    async function handle_scroll(event: Event) {
      
      if (window.scrollY > (document.body.scrollHeight * .8 || document.body.scrollHeight - 3000)) {
        await load_assets_batch();
      }
      
    }
    
    async function load_assets_batch() {
      if (is_loadings_assets) return
      
      show_loading_indicator = true
      is_loadings_assets = true
      
      let resp = await imageLoader.fetch_assets(applied_filters);
       
      is_loadings_assets = false
      show_loading_indicator = false
      
      if (resp.status === 200) {
        assets_list = [...assets_list, ...resp.assets]
      }
    }
</script>

<MasonryGrid
  frameWidth={400}
  gap={10}
>
    {#each assets_list as asset}
        <Frame width={asset.ratio.w} height={asset.ratio.h}>
            <div class="asset-container">
                <Skeleton class="skeleteonLoader h-full w-full absolute top-0 left-0" />
                <video loop autoplay muted preload="auto" poster={asset.optimized_poster_img_url} disablepictureinpicture>
                    <source src="{asset.optimized_asset_url}.webm" type="video/webm">
                    <source src="{asset.optimized_asset_url}.mp4" type="video/mp4">
                </video>
            </div>
        </Frame>
    {/each}
</MasonryGrid>

{#if show_loading_indicator}
    <div class="flex flex-col items-center gap-2">
        <Spinner class="size-8" />
    </div>
{/if}

<style>
    .asset-container {
        border-radius: 5px;
        overflow: hidden;
        position: relative;
        width: 100%;
        height: 100%;
    }
        
    img, video {
        display: block;
        width: 100%;
        height: 100%;
        z-index: 1;
        position: relative;
    }
</style>