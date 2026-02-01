<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import imageLoader from "$lib/apis/images.svelte"
    import { RegularMasonryGrid  as MasonryGrid, Frame } from '@masonry-grid/svelte'
    
    let assets_list: any[] = $state([])
    
    let is_loadings_assets = false;
    
    const {media_type = ""}: {media_type?: "" | "Image" | "Video"} = $props()
    
    onMount(() => {     
      imageLoader.reset()
      imageLoader.set_media(media_type)
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
            <div class="asset-container">
                {#if asset.type === "Image"}
                    <img loading="lazy" src='{asset.optimized_asset_url}' alt='Civit' />
                {:else}
                    <video loop autoplay muted preload="auto" poster={asset.optimized_poster_img_url} disablepictureinpicture>
                        <source src="{asset.optimized_asset_url}.webm" type="video/webm">
                        <source src="{asset.optimized_asset_url}.mp4" type="video/mp4">
                    </video>
                {/if}
            </div>
        </Frame>
    {/each}
</MasonryGrid>

<style>
    .asset-container {
        border-radius: 5px;
        overflow: hidden;
        position: relative;
    }
    
    img, video {
        display: block;
        width: 100%;
        height: 100%;
    }
</style>