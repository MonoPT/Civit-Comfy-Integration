<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import imageLoader from "$lib/apis/images.svelte"
    import { RegularMasonryGrid  as MasonryGrid, Frame } from '@masonry-grid/svelte'
    
    let assets_list: any[] = []
    
    let is_loadings_assets = false;
    
    onMount(() => {     
      load_assets_batch();
      
      window.addEventListener("scroll", handle_scroll)
    })
    
    onDestroy(() => window.removeEventListener("scroll", handle_scroll))
    
    function handle_scroll(event: Event) {
      
      if (window.scrollY > (document.body.scrollHeight * .8 || document.body.scrollHeight - 3000)) {
        load_assets_batch();
      }
      
    }
    
    async function load_assets_batch() {
      if (is_loadings_assets) return
      
      is_loadings_assets = true
      
      let resp = await imageLoader.fetch_assets();
      
      if (resp.status === 200) {
        assets_list = [...assets_list, ...resp.assets]
      }
      
      is_loadings_assets = false
    }
</script>

<MasonryGrid
  frameWidth={400}
  gap={10}
>
    {#each assets_list as asset}
        <Frame width={asset.ratio.w} height={asset.ratio.h}>
            <div class="container">
                <img loading="lazy" src='{asset.optimized_img_url}' alt='Photo' />
            </div>
        </Frame>
    {/each}
</MasonryGrid>