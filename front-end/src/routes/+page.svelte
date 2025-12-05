<script lang="ts">
    import { onMount } from "svelte";
    import {ImageGallery, type Image} from "$lib/api/imageGallery";
    import Spinner from "$lib/components/spinner.svelte";
    import FilterManager from "$lib/api/filterManager"
    import {user_token} from "$lib/state.svelte"
    
    let column_width = $state(370);
    let columns = $state(4);
    
    let last_n_columns = 4;
        
    let images: {[key: number]: Image[]} = $state({})
    let imagesLoading = $state(false)
    let screenSize = $state(1500)
    let imageGallerySentinel: HTMLElement;
        
    const handleIntersection = (entries: IntersectionObserverEntry[], observer: IntersectionObserver) => {
      entries.forEach(async (entry) => {
        if (entry.isIntersecting) {
          if (imagesLoading) return;
          imagesLoading = true;
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
      
      const options = {
        root: null, // Use the viewport as the root
        rootMargin: '0px',
        threshold: 0 // Trigger when the sentinel is fully visible
      };
      
      const observer = new IntersectionObserver(handleIntersection, options);
      
      observer.observe(imageGallerySentinel);
            
      // Check for images to load and add tags
      document.addEventListener(
        "load",
        (event) => {
          const target = event.target as HTMLImageElement
          if (target.tagName === "IMG" && target.classList.contains("listenLoadings")) {
            (target.parentNode! as HTMLElement).classList.remove("skeleton-loading")
          }
        },
        true // <-- muito importante! precisa de capturing
      );
      
      document.addEventListener(
        "error",
        (event) => {
          const target = event.target as HTMLImageElement
          if (target.tagName === "IMG") {
            (target.parentNode! as HTMLElement).classList.add("failedToLoad")
          }
        },
        true // necessário para capturar eventos não-bubbling (como load e error)
      );
    })
    
    let MediaContainer: HTMLElement
    
    onMount(() => { // Lazy load videos
      const options = {
        root: null,
        rootMargin: "0px",
        scrollMargin: "0px",
        threshold: 0.01,
      };
      
      const observer = new IntersectionObserver((entries, observer) => {
        entries.forEach((entry) => {
          if(!entry.isIntersecting) return
          
          const target = entry.target as HTMLDivElement 
         
          const src = target.querySelector("track")!.src
          
          let source = document.createElement("source")
          source.src = src
          source.type = "video/mp4"
            
          target.appendChild(source)
          target.closest(".image-wrapper")?.classList.remove("skeleton-loading")     
          observer.unobserve(target)
        })
      }, options);
      
      //
      
      const mutationObserver = new MutationObserver((mutations) => {
          let videos: HTMLVideoElement[] = []
          mutations.forEach(mutation => {
            mutation.addedNodes.forEach((node) => {
              if (node.nodeName !== "#text") {
                //@ts-ignore
                node.querySelectorAll("video").forEach((v) => videos.push(v))
              }
            })
          });
          
          Array.from(new Set(videos)).forEach((v) => {
            observer.observe(v)
            window.addEventListener("UpdatedImagesList", () => {
              observer.unobserve(v)
            })
          })
      });
      
      mutationObserver.observe(MediaContainer, {
          childList: true,  // Observe direct children (like added <video> tags)
          subtree: true,    // Observe the entire subtree (including children of children)
      });
    })
</script>

<main style="--screenWidth: {screenSize}; --columns: {columns}" bind:this={MediaContainer}>
    {#each Array.from({ length: columns }, (_, index) => index) as number}
        <div class="column">       
            {#each images[number] as image}
                <div class="image-wrapper skeleton-loading" style="--aspectRation: {image.width / image.height}">
                    {#if image.media_type === "image"}
                        <img class="listenLoadings" src="{image.url}" alt="" loading="lazy">
                    {:else}
                        <video loop autoplay muted preload="none">
                            <track src="{image.url}" kind="captions" srclang="en" label="English">
                        </video>
                    {/if}
                    <div class="errorMsg">
                        <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M4.02693 18.329C4.18385 19.277 5.0075 20 6 20H18C19.1046 20 20 19.1046 20 18V14.1901M4.02693 18.329C4.00922 18.222 4 18.1121 4 18V6C4 4.89543 4.89543 4 6 4H18C19.1046 4 20 4.89543 20 6V14.1901M4.02693 18.329L7.84762 14.5083C8.52765 13.9133 9.52219 13.8482 10.274 14.3494L10.7832 14.6888C11.5078 15.1719 12.4619 15.1305 13.142 14.5865L15.7901 12.4679C16.4651 11.9279 17.4053 11.8856 18.1228 12.3484C18.2023 12.3997 18.2731 12.4632 18.34 12.5302L20 14.1901M11 9C11 10.1046 10.1046 11 9 11C7.89543 11 7 10.1046 7 9C7 7.89543 7.89543 7 9 7C10.1046 7 11 7.89543 11 9Z" stroke="#ffffff" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
                        <h2>Failed to load image</h2>
                    </div>
                </div>
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
            
            .image-wrapper {
                --color_1: rgba(255,255,255, .03);
				--color_2: rgba(255,255,255, .07);
					
				&.skeleton-loading {
				    img, video {
						opacity: 0 !important;
					}
				}
								
				.errorMsg {
				    display: none;
					opacity: .45;
								
					svg {
	                    display: block;
						width: 30%;
						margin-inline: auto;
					}
					
					h2 {
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
                    
				    img {
						display: none;
					}
					
					.errorMsg {
	                    display: block;
					}
				}
				
				border-radius: .5rem;
                min-height: 50px;
                width: 100%;
                margin-bottom: var(--gap);
                position: relative;
                aspect-ratio: var(--aspectRation);
                
                cursor: progress; 
            }
        }
                
        img, video {
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