<script lang="ts">
    import { onMount } from "svelte";
    import Spinner from "$lib/components/spinner.svelte"
    import api from "$lib/api";
    import { user_token, userState } from "$lib/state.svelte";
    
    let {open = false, is_mobile=false, media_id = 0, media_type = "Image"} = $props()
    
    let button: HTMLElement
    let buttonClose: HTMLElement
    let downloadButton: HTMLElement
    let collectionsButton: HTMLElement
    let favoriteButton: HTMLElement
    
    let favoriteLoading = $state(true)
    let isFavorite = $state(false)

    
    onMount(() => {
      buttonClose.addEventListener("click", () => window.dispatchEvent(new CustomEvent("closeVisualizer")))
      downloadButton.addEventListener("click", () => window.dispatchEvent(new CustomEvent("downloadVisualizerMedia")))
      collectionsButton.addEventListener("click", () => window.dispatchEvent(new CustomEvent("openCollectionSelector", {
        detail: {id: media_id, media_type: media_type}
      })))
      
      
      if(!button) return
      button.addEventListener("click", () => window.dispatchEvent(new CustomEvent("visualizerToggleMeta")))
      load_favorite()
      
      favoriteButton.addEventListener("click", async () => {
        if (favoriteLoading) return
        favoriteLoading = true
        
        if (isFavorite) {
          await fetch(api.unfavorite_media(user_token.token, media_id))
        } else {
          await fetch(api.favorite_media(user_token.token, media_id))
        }
        
        load_favorite()
      })
    })
    
    const load_favorite = async () => {
      favoriteLoading = true
      isFavorite = false
      
      let res = await fetch(api.collections_with_media(user_token.token, media_id))
      if (res.status !== 200) {
        favoriteLoading = false;
        console.error("Could not get media with collection for media id " + media_id)
        console.log(await res.text())
        return
      }
      
      let collections = await res.json()
      
      const favorite_collection_id = userState.collections.find((collection: any) => collection.name === "comfyui_civit_favorites").id || -1
      
      let fColl = collections.find((collection: any) => collection.collectionId === favorite_collection_id)
      
      if (fColl) {
        isFavorite = true 
      }
      
      favoriteLoading = false
    }
</script>

<div class="wrapper" class:open class:is_mobile>
    <div class="icon" bind:this={buttonClose}>
        <svg viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
    </div>
    
    <div class="icon" bind:this={downloadButton}>
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-download "><path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2"></path><path d="M7 11l5 5l5 -5"></path><path d="M12 4l0 12"></path></svg>
    </div>
    
    <div class="icon collection" bind:this={collectionsButton}>
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-bookmark "><path d="M18 7v14l-6 -4l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4z"></path></svg>
    </div>
    
    <div class="icon favorite" class:favorited={isFavorite} bind:this={favoriteButton}>
        {#if !favoriteLoading}
            <svg class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium css-1phnduy" focusable="false" aria-hidden="true" viewBox="0 0 24 24"><path d="m12 21.35-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54z"></path></svg>
        {:else}
            <Spinner size={15} tickness={2} />
        {/if}
    </div>
    
    <div class="icon mobileOpenMeta" bind:this={button}>
        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-chevron-up "><path d="M6 15l6 -6l6 6"></path></svg>
    </div>
</div>

<style>
    .wrapper {
        display: flex;
        align-items: center;
        height: 20px;
        gap: .5rem;
        
        &.open .mobileOpenMeta {
            rotate: 180deg;
        }
        
        &:not(.is_mobile) .mobileOpenMeta {
            display: none;
        }
        
        .icon {
            display: flex;
            align-items: center;
            justify-content: center;
            background: rgba(255,255,255, .1);
            padding: .5rem;
            border-radius: 100%;
            transition: .12s;
            transform-origin: center;
            cursor: pointer;
            min-width: 2rem;
            min-height: 2rem;
            
            &:hover {
                background: rgba(255,255,255, .2);
            }
            
            svg {
                width: 1rem;
                display: block;
                height: 50%;
                aspect-ratio: 1 / 1;
            }
            
            &.mobileOpenMeta {
                margin-left: auto;
            }
            
            &.favorite {
                fill: transparent;
                stroke: #fff;
                
                :global(&.favorited) {
                    fill: #fff;
                }
            }
        }
    }
</style>