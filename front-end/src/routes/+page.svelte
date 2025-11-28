<script lang="ts">
    import { onMount } from "svelte";
    import {ImageGallery, type Image, MediaType} from "$lib/imageGallery";
    import Spinner from "$lib/components/spinner.svelte";
    import FilterManager from "$lib/filterManager"
    
    let column_width = $state(270);
    let columns = $state(4);
        
    let images: {[key: number]: Image[]} = $state({})
    let imagesLoading = $state(false)
    let screenSize = $state(1500)
    let imageGallerySentinel: HTMLElement;
    
    const handleIntersection = (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {
      entries.forEach(async (entry) => {
        if (entry.isIntersecting) {
          if (imagesLoading) return;
          imagesLoading = true;
          await ImageGallery.load_next_images()
        }
      });
    };
    
    const handle_filter_updates = () => {
      ImageGallery.cursor = null
      ImageGallery.load_next_images()
    }
    
    onMount(async () => {    
      const onResize = () => {
        screenSize = window.innerWidth
        columns = Math.floor(screenSize / column_width)
        window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
      }
      onResize()
      
      window.addEventListener('resize', onResize);
      
      
      window.addEventListener("StartedLoadingImages", () => imagesLoading = true)
      window.addEventListener("UpdatedImagesList", () => {
        imagesLoading = false
        
        let image_list = ImageGallery.images;
              
        images = {}
        for (let i = 0; i < columns; i++) {
          images[i] = []
        }
                      
        let key = 0
        image_list.forEach((item: Image) => {         
          images[key].push(item)
          
          key += 1
          if (key >= columns) {
            key = 0
          }
        })          
      })
      window.addEventListener("UpdatedFilters", handle_filter_updates)
      
      const options = {
        root: null, // Use the viewport as the root
        rootMargin: '0px',
        threshold: 1 // Trigger when the sentinel is fully visible
      };
      
      const observer = new IntersectionObserver(handleIntersection, options);
      
      observer.observe(imageGallerySentinel);
            
    })
</script>

<main style="--screenWidth: {screenSize}; --columns: {columns}">
    {#each Array.from({ length: columns }, (_, index) => index) as number}
        <div class="column">            
            {#each images[number] as image}
                {#if image.media_type === MediaType.Image}
                    <img src="{image.url}" alt="" loading="lazy">
                {:else}
                <video loop autoplay muted>
                  <source src="{image.url}" type="video/mp4">
                  <track src="{image.url}" kind="captions" srclang="en" label="English">
                </video>
                {/if}
                
            {/each}
        </div>
    {/each}
    
</main>

<div bind:this={imageGallerySentinel}></div>

{#if imagesLoading}
    <div class="Images-Loading-Wrapper">
        <Spinner size={35} tickness={3} />
        <p>Loading images</p>
    </div>
{/if}

<style>    
    main {
        --gap: 1rem;
        
        display: grid;
        grid-template-columns: repeat(var(--columns), 1fr);
        
        
        gap: var(--gap);
        
        .column {
            display: flex;
            flex-direction: column;
        }
                
        img, video {
            display: block;
            position: relative;
            max-width: 100%;
            margin-bottom: var(--gap);
            min-height: 50px;
            background: rgba(0,0,0, .07);
        }
    }
    
    .Images-Loading-Wrapper {
        display: flex;
        flex-direction: column;
        margin-top: 2rem;
        justify-content: center;
        align-items: center;
        
        p {
            margin-top: .7rem;
        }
    }
</style>