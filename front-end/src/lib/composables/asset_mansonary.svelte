<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import imageLoader from "$lib/apis/images.svelte"
    import { RegularMasonryGrid  as MasonryGrid, Frame } from '@masonry-grid/svelte'
    
    import { type FilterOption, type Filter } from "$lib/filter";
    
    import { Skeleton } from "$lib/components/ui/skeleton/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    
    import { ThumbsUp, EllipsisVertical, Laugh, Heart, Frown, MessageSquare, ArrowDownToLine } from "@lucide/svelte";
    
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
        
    onMount(() => { // Tracks added items to DOM
      const mansonary = document.getElementById("mansonary")!
      
      const config = { attributes: true, childList: true, subtree: true };
      
      const observer = new MutationObserver(onMansonaryMutation);
      
      observer.observe(mansonary, config);
    })
    
    const intObsv = new IntersectionObserver(
      (entries) => {
        entries.forEach(entry => {
          if (entry.isIntersecting) {
            const target = entry.target as HTMLElement;
            
            target.appendChild(createMedia(target))
          } else {
            entry.target.querySelector("video")?.remove()
          }
        });
      },
      {
        root: null,
        threshold: 0,
        rootMargin: '2500px',
      }
    );
    
    function onMansonaryMutation(mutationList: MutationRecord[], observer: MutationObserver) {
      mutationList.forEach((mut) => {
        if(mut.addedNodes.length < 1) return;
        
        mut.addedNodes.forEach((node) => {
          if (node.nodeName.startsWith("#")) return

          const assetContainer = (node as HTMLElement).querySelector<HTMLElement>("[data-asset-container]")
          
          if (!assetContainer) return
                   
          intObsv.observe(assetContainer);
        })
      })
    }
    
    function createMedia(asset: HTMLElement) {
      const src = asset.getAttribute("data-media")!
      
      const videoElement = document.createElement("video")
      videoElement.loop = true;
      videoElement.autoplay = false;
      videoElement.muted = true;
      videoElement.poster = asset.getAttribute("data-cover")!
      videoElement.disablePictureInPicture = true
      videoElement.disableRemotePlayback = true
      
      const sourceWebm = document.createElement("source")
      sourceWebm.src = `${src}.webm`
      sourceWebm.type = "video/webm"
      
      const sourceMp4 = document.createElement("source")
      sourceMp4.src = `${src}.mp4`
      sourceMp4.type = "video/mp4"
      
      videoElement.appendChild(sourceWebm)
      videoElement.appendChild(sourceMp4)
      
      videoElement.addEventListener("pointerenter", () => {
        videoElement.play()
      }, {once: true})
      
      return videoElement
    }
    
</script>

<MasonryGrid
    id="mansonary"
    frameWidth={400}
    gap={10}
>
    {#each assets_list as asset}
        <Frame width={asset.ratio.w} height={asset.ratio.h}>
            <div data-asset-container class="asset-container" data-cover={asset.optimized_poster_img_url} data-media={asset.optimized_asset_url}>
                <Skeleton class="skeleteonLoader h-full w-full absolute top-0 left-0" />
                <!--<video loop autoplay={false} muted preload="auto" poster={asset.optimized_poster_img_url} disablepictureinpicture>
                    <source src="{asset.optimized_asset_url}.webm" type="video/webm">
                    <source src="{asset.optimized_asset_url}.mp4" type="video/mp4">
                </video>-->
                <div class="overlay">
                    <div class="stats absolute bottom-1.5 left-1.5">
                        <Button variant="outline" size="sm">
                          <ThumbsUp  /> {asset.stats.likeCountAllTime}
                        </Button>
                        {@render reactionsDropdown(
                          asset.stats.likeCountAllTime, 
                          asset.stats.laughCountAllTime,
                          asset.stats.heartCountAllTime,
                          asset.stats.cryCountAllTime,
                          asset.stats.commentCountAllTime,
                          asset.stats.collectedCountAllTime
                        )}
                    </div>
                </div>
            </div>
        </Frame>
    {/each}
</MasonryGrid>

{#snippet reactionsDropdown(like: number, laugh: number, heart: number, cry: number, comment: number, collected: number)}
<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}
      <Button {...props} variant="outline" size="sm"><EllipsisVertical  /></Button>
    {/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content class="w-max min-w-max bg-transparent backdrop-blur-sm">
    <DropdownMenu.Group class="w-max flex flex-col gap-1.5">
      <DropdownMenu.CheckboxItem class="p-0 w-full">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <ThumbsUp  /> {like}
          </Button>
      </DropdownMenu.CheckboxItem>
      
      <DropdownMenu.CheckboxItem class="p-0 w-full">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <Laugh  /> {laugh}
          </Button>
      </DropdownMenu.CheckboxItem>
      
      <DropdownMenu.CheckboxItem class="p-0 w-full">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <Heart   /> {heart} 
          </Button>
      </DropdownMenu.CheckboxItem>
      
      <DropdownMenu.CheckboxItem class="p-0 w-full">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <Frown  /> {cry} 
          </Button>
      </DropdownMenu.CheckboxItem>
      
      <DropdownMenu.CheckboxItem class="p-0 w-full" style="pointer-events: none;">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <MessageSquare  /> {comment} 
          </Button>
      </DropdownMenu.CheckboxItem>
      
      <DropdownMenu.CheckboxItem class="p-0 w-full" style="pointer-events: none;">
          <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
            <ArrowDownToLine /> {collected}
          </Button>
      </DropdownMenu.CheckboxItem>
      
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
{/snippet}



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
        
    :global(#mansonary video) {
        display: block;
        width: 100%;
        height: 100%;
        z-index: 1;
        position: relative;
    }
    
    .overlay {
        position: absolute;
        z-index: 2;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        pointer-events: none;
        
        > * {
            pointer-events: all;
        }
        
        .stats {
            opacity: 0;
            transition: .12s;
            z-index: 1;
        }
        
        &::after {
            content: '';
            position: absolute;
            width: 100%;
            height: 100%;
            background: linear-gradient(0deg, var(--background) 4% ,color-mix(in lab, var(--background) 50%, transparent 70%) 30%, transparent 100%);
            transition: .12s;
            opacity: 0;
            z-index: 0;
        }
    }
    
    .asset-container:hover .overlay {
        &::after {
            opacity: 1;
        }
        
        .stats {
            opacity: 1;
        }
    }
</style>