<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import modelLoader from "$lib/apis/models.svelte"
    import { RegularMasonryGrid  as MasonryGrid, Frame } from '@masonry-grid/svelte'
    
    import { type FilterOption, type Filter } from "$lib/filter";
    
    import Model from "$lib/composables/model.svelte"
    
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
      modelLoader.reset()
      load_assets_batch()
	});
    
    onMount(async () => {   
      assets_list = []      
      modelLoader.reset()
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
      
      let resp = await modelLoader.fetch_assets(applied_filters);
       
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
    frameWidth={300}
    gap={10}
>
    {#each assets_list as model}
        <Frame width={model.ratio.w} height={model.ratio.h}>
            <Model asset={model} />
        </Frame>
    {/each}
</MasonryGrid>


{#if show_loading_indicator}
    <div class="flex flex-col items-center gap-2">
        <Spinner class="size-8" />
    </div>
{/if}

<style>   
    :global(#mansonary video) {
        display: block;
        width: 100%;
        height: 100%;
        z-index: 1;
        position: relative;
    }
    
    :global([data-asset-container]:has(video) [data-slot="skeleton"]) {
        display: none;
    }
    
    :global([data-asset-container]:has(video)) {
        border: 1px solid transparent;
        border-style: var(--tw-border-style);
        border-color: var(--input);
    }
</style>