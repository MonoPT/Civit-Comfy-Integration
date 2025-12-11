<script lang="ts">
    import { onMount } from "svelte";
    import {ImageGallery, type Image} from "$lib/api/imageGallery";
    import api from "$lib/api"
    import Spinner from "$lib/components/spinner.svelte";
    import {user_token} from "$lib/state.svelte"
    import Header from "$lib/components/header.svelte";
    
    import {page} from "$app/state"
    
    let {media, page_title} = $props()
    
    let column_width = $state(370);
    let columns = $state(4);
    
    let currentRoute = $state( page.url.pathname);
    
    let last_n_columns = 4;
        
    let images: {[key: number]: Image[]} = $state({})
    let imagesLoading = $state(false)
    let screenSize = $state(1500)
    let imageGallerySentinel: HTMLElement;
    
    let is_first_intersection = true
        
    const handleIntersection = (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {
      entries.forEach(async (entry) => {
        if (entry.isIntersecting) {
          if (imagesLoading) return;
          imagesLoading = true;
                    
          if (is_first_intersection) { // Prevents dual loading on page load
            is_first_intersection = false
            imagesLoading = false;
            return
          }
          
          await ImageGallery.load_next_images(user_token.token)
        }
      });
    };
    
    const handle_filter_updates = () => {
      ImageGallery.cursor = null
      ImageGallery.images = []
      images = []
      
      ImageGallery.load_next_images(user_token.token)
    }
    
    onMount(async () => {  
      //updateMedia
      handle_filter_updates()
      
      const onResize = () => {
        screenSize = window.innerWidth
        last_n_columns = columns
        columns = Math.floor(screenSize / column_width)
        
        if (columns === last_n_columns) return
        
        window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
      }
      onResize()
      
      window.addEventListener('resize', onResize);
      window.addEventListener("UpdatedFilters", handle_filter_updates)
      
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
      
      
      const observer = new IntersectionObserver(handleIntersection, {root: null, rootMargin: '0px', threshold: 0});
      
      observer.observe(imageGallerySentinel);
    })
    
    let MediaContainer: HTMLElement
    
    onMount(() => { // Infinite scroller loader and observers
      
      const errorEl = document.createElement("div")
      errorEl.classList.add("errorMsg")
      
      errorEl.innerHTML = `
        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M4.02693 18.329C4.18385 19.277 5.0075 20 6 20H18C19.1046 20 20 19.1046 20 18V14.1901M4.02693 18.329C4.00922 18.222 4 18.1121 4 18V6C4 4.89543 4.89543 4 6 4H18C19.1046 4 20 4.89543 20 6V14.1901M4.02693 18.329L7.84762 14.5083C8.52765 13.9133 9.52219 13.8482 10.274 14.3494L10.7832 14.6888C11.5078 15.1719 12.4619 15.1305 13.142 14.5865L15.7901 12.4679C16.4651 11.9279 17.4053 11.8856 18.1228 12.3484C18.2023 12.3997 18.2731 12.4632 18.34 12.5302L20 14.1901M11 9C11 10.1046 10.1046 11 9 11C7.89543 11 7 10.1046 7 9C7 7.89543 7.89543 7 9 7C10.1046 7 11 7.89543 11 9Z" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
        <h2>Failed to load image</h2>
      `

      const observerPerfOt = new IntersectionObserver((entries, observer) => {
        entries.forEach((entry) => {
          const target = entry.target
          
          if(!entry.isIntersecting) {
            target.innerHTML = ""
            return
          }
          
          const errorHandler = (type: "video" | "image") => {
            target.classList.remove("skeleton-loading")
            target.classList.add("failedToLoad")
            target.innerHTML = ""
            
            let el = errorEl.cloneNode(true) as HTMLElement
            el.querySelector("h2")!.textContent = `Failed to load ${type}`
            
            target.appendChild(el)
          }
          
          const src_civit = target.getAttribute("data-url")!
          const mediaType = target.getAttribute("data-media")! as "image" | "video"
          const media_data_index = Number(target.getAttribute("data-mediaIndex")!)
          const media_data = ImageGallery.images.find((img) => img.index === media_data_index)!
          
          const uuid = target.getAttribute("data-uuid")!
          const src_proxy = `${api.endpoint}/media_proxy?id=${uuid}`
          
          // redirects to reverse proxy
          let src = `${src_proxy}&media_type=${mediaType}`; 
          
          /*if ( == "video") {
            src = `${src_proxy}&media_type=video`; 
            }*/
          
          let media: HTMLImageElement | HTMLVideoElement
          
          if (mediaType == "image") {
            media = document.createElement("img")
            media.src = src
            media.loading = "lazy"

          } else {
            media = document.createElement("video")
            media.loop = true
            media.muted = true
            media.autoplay = true
            
            let source = document.createElement("source")
            source.src = src
            source.type = "video/webm"
            
            media.appendChild(source) 
          }
          
          media.addEventListener("loadeddata", () => target.classList.remove("skeleton-loading"))
          media.addEventListener("load", () => target.classList.remove("skeleton-loading"))
          media.addEventListener("error", () => errorHandler(mediaType))
          media.querySelector("source")?.addEventListener("error", () => errorHandler(mediaType))
          
          let reactions = document.createElement("div")
          reactions.classList.add("reactionsContainer")
          
          const createSpanWithText = (val: number, icon: string) => {
            let el = document.createElement("span")
            el.innerHTML = `${icon} ${val}`
            return el
          } 
          
          reactions.appendChild(createSpanWithText(media_data.stats.likeCountAllTime, "👍"))
          reactions.appendChild(createSpanWithText(media_data.stats.heartCountAllTime, "❤️"))
          reactions.appendChild(createSpanWithText(media_data.stats.laughCountAllTime, "😂"))
          reactions.appendChild(createSpanWithText(media_data.stats.cryCountAllTime, "😢"))
          
          target.appendChild(media)
          target.appendChild(reactions)
        })
      },  {
        root: null,
        rootMargin: "0px",
        threshold: 0.0,
      });
      
      
      const mutationObserver = new MutationObserver((mutations) => {
          mutations.forEach(mutation => {
            mutation.addedNodes.forEach((node) => {
              if (node.nodeName !== "#text") {
                if (!(node as HTMLElement).classList.contains("image-wrapper")) return
                
                observerPerfOt.observe(node as HTMLElement)
              }
            })
          });
      });
      
      mutationObserver.observe(MediaContainer, {
          childList: true, 
          subtree: true,   
      });
    })
</script>

<Header current_route={currentRoute} media={media} page_title={page_title}/>

<main style="--screenWidth: {screenSize}; --columns: {columns}" bind:this={MediaContainer}>
    {#each Array.from({ length: columns }, (_, index) => index) as number}
        <div class="column">       
            {#each images[number] as image}
                <div class="image-wrapper skeleton-loading" data-uuid={image.uuid} data-mediaIndex={image.index} data-media={image.media_type} data-url={image.url} style="--aspectRation: {image.width / image.height}"></div>
            {/each}
        </div>
    {/each}
    
</main>

<div bind:this={imageGallerySentinel}></div>

{#if imagesLoading}
    <div class="Images-Loading-Wrapper">
        <Spinner size={35} tickness={3} />
        <p>Loading data</p>
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
        
            .image-wrapper {
                --color_1: rgba(255,255,255, .03);
				--color_2: rgba(255,255,255, .07);
				contain: content;
				will-change: transform, opacity, display;
				transform: translate3d(0,0,0);
				position: relative;
								
				&.skeleton-loading > :global(*) {
                    opacity: 0;
				}
				
				:global(.errorMsg) {
				    display: block;
					opacity: .45;
								
					:global(svg) {
	                    display: block;
						width: 30%;
						margin-inline: auto;
					}
					
					:global(h2) {
	                    font-size: 1.2rem;
						font-weight: 350;
						text-align: center;
						margin-top: 1rem;
						
					}
				}
				
				&:global(.failedToLoad) {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    background: var(--color_1);
				}
				
				border-radius: .5rem;
                min-height: 50px;
                width: 100%;
                margin-bottom: var(--gap);
                position: relative;
                aspect-ratio: var(--aspectRation);
                
                cursor: progress;
               
               :global(.reactionsContainer) {
                   position: absolute;
                   display: flex;
                   align-items: center;
                   z-index: 2;
                   bottom: 0rem;
                   padding: 1rem;
                   gap: calc(var(--spacing) * 1);
                   width: 100%;
                   flex-wrap: wrap;
                   
                   :global(span) {
                       display: flex;
                       align-items: center;
                       gap: calc(var(--spacing) * 2);
                       background: rgba(0,0,0,.6);
                       padding: calc(var(--spacing) * 1);
                       padding-inline: calc(var(--spacing) * 2);
                       border-radius: calc(var(--spacing) * 1);
                       width: max-content;
                       font-size: .8em;
                       
                       :global(svg) {
                           position: relative;
                           display: block;
                           max-width: 1rem;
                           height: max-content;
                       }
                   }
                   
               } 
            }
        }
                
        :global(img, video) {
            display: block;
            position: relative;
            max-width: 100%;
            height: 100%;
            background: rgba(0,0,0, .07);
            border-radius: inherit;
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