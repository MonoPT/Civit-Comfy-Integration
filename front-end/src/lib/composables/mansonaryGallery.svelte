<script lang="ts">
    import { onMount } from "svelte";
    import {ImageGallery, type Image} from "$lib/api/imageGallery";
    import api from "$lib/api"
    import Spinner from "$lib/components/spinner.svelte";
    import {user_token, userState} from "$lib/state.svelte"
    import Header from "$lib/components/header.svelte";
    
    import {page} from "$app/state"
    
    let {media, page_title} = $props()
    
    let column_width = $state(370);
    let columns = $state(4);
    
    let collection_list: {[key: number]: number[]} = [] // list of collection to wich media belongs
    
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
    
    const load_collection_data_by_media_id = async (id: number) => {
      const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
      if(!target) return
      
      let res = await fetch(api.collections_with_media(user_token.token, id))
      if (res.status !== 200) alert("Error getting collections of " + id) 
      
      let data = await res.json() as any
      
      let ids = data.map((el: any) => el.collectionId);
                  
      collection_list[id] = ids;
      //ids.forEach((id: number) => collection_list[id].push(id))
      
      const loader = target.querySelector(".loader")
      if (loader) loader.remove()
      
      add_fast_items_by_element_id(id)
    }
    
    const favoriteMedia = async (id: number) => {
      const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
      if(!target) return
      
      let loader = target.querySelector(".loader")
      if (loader) loader.remove()
      
      const fastactions = target.querySelector(".fastActions")
      if (fastactions) fastactions.remove()
      
      let spinner = create_loader(20, 3)
      
      target.appendChild(spinner)
      
      const res = await fetch(api.favorite_media(user_token.token, id))
      console.log(await res.text())
      
      load_collection_data_by_media_id(id)
    }
    
    const add_fast_items_by_element_id = (id: number) => {
      const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
      if(!target) return
      
      const existing_controls = target.querySelector(".fastActions")
      
      if(existing_controls) {
        existing_controls.remove()
      }
      
      const favorite_collection_id = userState.collections.find((collection: any) => collection.name === "comfyui_civit_favorites").id || -1
      const in_collections = collection_list[id]
          
      // Remove existing if its the case
      const existing = target.querySelector(".fastActions")
      if (existing) existing.remove()
      
      const container = document.createElement("div")
      container.classList.add("fastActions")
      
      let favoriteIcon = document.createElement("span")
      favoriteIcon.classList.add("favoriteIcon")
      
      if (in_collections.includes(favorite_collection_id)) {
        favoriteIcon.innerHTML = `<svg fill="#ec2727" height="200px" width="200px" version="1.1" id="Filled_Icons" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 24 24" enable-background="new 0 0 24 24" xml:space="preserve"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g id="Favorite-Filled"> <path d="M12,22C9.63,20.17,1,13.12,1,7.31C1,4.38,3.47,2,6.5,2c1.9,0,3.64,0.93,4.65,2.48L12,5.78l0.85-1.3 C13.86,2.93,15.6,2,17.5,2C20.53,2,23,4.38,23,7.31C23,13.15,14.38,20.18,12,22z"></path> </g> </g></svg>`
      } else {
        favoriteIcon.innerHTML = `<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path fill-rule="evenodd" clip-rule="evenodd" d="M11.993 5.09691C11.0387 4.25883 9.78328 3.75 8.40796 3.75C5.42122 3.75 3 6.1497 3 9.10988C3 10.473 3.50639 11.7242 4.35199 12.67L12 20.25L19.4216 12.8944L19.641 12.6631C20.4866 11.7172 21 10.473 21 9.10988C21 6.1497 18.5788 3.75 15.592 3.75C14.2167 3.75 12.9613 4.25883 12.007 5.09692L12 5.08998L11.993 5.09691ZM12 7.09938L12.0549 7.14755L12.9079 6.30208L12.9968 6.22399C13.6868 5.61806 14.5932 5.25 15.592 5.25C17.763 5.25 19.5 6.99073 19.5 9.10988C19.5 10.0813 19.1385 10.9674 18.5363 11.6481L18.3492 11.8453L12 18.1381L5.44274 11.6391C4.85393 10.9658 4.5 10.0809 4.5 9.10988C4.5 6.99073 6.23699 5.25 8.40796 5.25C9.40675 5.25 10.3132 5.61806 11.0032 6.22398L11.0921 6.30203L11.9452 7.14752L12 7.09938Z"></path> </g></svg>`;
      }
      
      // todo: store collections in state and get id of favorite collections
      
      favoriteIcon.addEventListener("click", () => favoriteMedia(id))
      
      container.appendChild(favoriteIcon)
      
      target.appendChild(container)
    }
    
    const create_loader = (size: number, tickness: number) => {
      let spinner = document.createElement("span")
      spinner.classList.add("loader")
      spinner.style.setProperty("width", `${size}px`)
      spinner.style.setProperty("height", `${size}px`)
      spinner.style.setProperty("--tickness", `${tickness}px`)
      
      return spinner
    }
    
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
          
          const id = parseInt(target.getAttribute("data-id")!)
          
          const src_civit = target.getAttribute("data-url")!
          const mediaType = target.getAttribute("data-media")! as "image" | "video"
          const media_data_index = Number(target.getAttribute("data-mediaIndex")!)
          const media_data = ImageGallery.images.find((img) => img.index === media_data_index)!
          
          const uuid = target.getAttribute("data-uuid")!
          const src_proxy = `${api.endpoint}/media_proxy?id=${uuid}`
          
          // redirects to reverse proxy
          let src = `${src_proxy}&media_type=${mediaType}`; 
                   
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
                            
          let spinner = create_loader(20, 3)
            
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
          
          if (!collection_list[id]) {
             target.appendChild(spinner)
          } else {
            add_fast_items_by_element_id(id)
          }
         
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
                if (!(node as HTMLElement).classList.contains("image-wrapper") || (node as HTMLElement).classList.contains("registered")) return
                (node as HTMLElement).classList.add("registered")
                
                //@ts-ignore
                const id = node.getAttribute("data-id")!
                //@ts-ignore
                const creator_id = node.getAttribute("data-creator-id")!
                //@ts-ignore
                const image_uuid = node.getAttribute("data-uuid")!   
                
                node.addEventListener("click", (e) => { 
                  const target = e.target as HTMLElement;
                  if(target.closest(".fastActions")) return
                  
                  window.dispatchEvent(new CustomEvent("loadMediaVisualizer", {detail: {id, creator_id, image_uuid}}))
                })
                
                node.addEventListener("pointerenter", async (_) => {
                  load_collection_data_by_media_id(id)
                }, {once: true})
                
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
                <div class="image-wrapper skeleton-loading" data-creator-id={image.creatorId} data-id={image.id} data-uuid={image.uuid} data-mediaIndex={image.index} data-media={image.media_type} data-url={image.url} style="--aspectRation: {image.width / image.height}"></div>
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
   
    .image-wrapper {
        :global(.loader), :global(.fastActions) {
            transition: .12s;
            opacity: 0;
            position: absolute;
            left: .5rem;
            top: .5rem;
        }
        
        &:hover :global(.loader), &:hover :global(.fastActions) {
            opacity: 1;
        }
        
        :global(.fastActions) {
            :global(span) {
                cursor: pointer;
            }
                        
            :global(svg) {
                width: 1.5rem;
                height: max-content;
                
                
                :global(path) {
                    fill: #ec2727;
                }
            }
        }
    }
   
    
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
                   opacity: 0;
                   transform: translateY(100px);
                   transition: .12s;
                   
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
              
               &:hover :global(.reactionsContainer) {
                   opacity: 1;
                   transform: translateY(0);
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