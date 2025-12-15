<script lang="ts">
    import { onMount } from "svelte";
    import api from "$lib/api"
    import {user_token} from "$lib/state.svelte"
    import Select from "$lib/components/form/select.svelte";
    import Spinner from "$lib/components/spinner.svelte";
    
    let isOpen = $state(false)
    let isLoading = $state(false)
   
    let collectionContainer: HTMLElement
    
    let closeButton: HTMLElement
    
    let collections: { name: string; value: string; }[] = $state([])
    
    let selected: string[] = $state([])
    
    let current_state = $state({id: 0, media_type: "Image"})
    
    onMount(() => {
      window.addEventListener("openCollectionSelector", async (e) => {
        //@ts-ignore
        const {id, media_type} = e.detail
        current_state.id = id
        current_state.media_type = media_type
        
        isOpen = true
        isLoading = true
        collections = []
        selected = []
        
        const urls = [
          api.collections_with_media(user_token.token, id),
          api.get_collections_by_media_type(user_token.token, media_type)
        ];
        
        const responses = await Promise.all(
          urls.map(url => fetch(url))
        );
        
        responses.forEach(res => {
          if (!res.ok) {
            throw new Error(`Erro em ${res.url}`);
          }
        });
        
        const data = await Promise.all(
          responses.map(res => res.json())
        );
        
        const [inCollections, collections_list] = data;
        
        collections = (collections_list as {id: number, name: string}[]).map((c) => {
          return {name: c.name, value: `${c.id}`}
        }).filter((c) => c.name !== "comfyui_civit_favorites")
                
        selected = (inCollections as {collectionId: number}[]).map((inc) => {
          let f = collections_list.find((el: any) => {
            return el.id === inc.collectionId
          })
          if (f) return f.name;
          return ""
        }).filter((cn) => cn.length > 0)
        
        isLoading = false
      })
    })
    
    onMount(() => closeButton.addEventListener("click", async () => isOpen = false))
    
    const saveToCollection = async () => {
      isLoading = true
      
      const inputs = Array.from(collectionContainer.querySelectorAll(`input[name="SelectedCollections"]`)) as HTMLInputElement[]
      
      let selected: number[] = []
      let not_selected: number[] = []
      
      inputs.forEach((input) => {
        const value = parseInt(input.value)
        
        if (input.checked) {
          selected.push(value)
        } else {
          not_selected.push(value)
        }
      })
      
      let res = await fetch(api.update_media_collections(user_token.token, current_state.id, selected, not_selected))
      
      window.dispatchEvent(new CustomEvent("openCollectionSelector", {detail: {id: current_state.id, media_type: current_state.media_type}}))
    }
</script>

<div class="container" class:open={isOpen} id="selectCollections" bind:this={collectionContainer}>
    <div class="card wrapper">
        <div class="wrapperMenu">
            <h2>Add to collection</h2>
            
            <span bind:this={closeButton}>
                <svg viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg" ><path d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z" fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path></svg>
            </span>
        </div>
        
        <div class="group">
            {#if isOpen && !isLoading}
                <Select onlySelect={true} select_list_cards empty_name=" " icon={2} name="SelectedCollections" selection_name="Collections" 
                    data_options={collections} selected={selected}/>
            {/if}
            
            {#if isLoading}
                <div class="Images-Loading-Wrapper">
                    <Spinner size={35} tickness={3} />
                    <p>Loading collections</p>
                </div>
            {/if}
            
        </div>
        
        {#if !isLoading}
            <button class="saveWrapper" onclick={() => saveToCollection()}>
                <span>Save</span>
            </button>
        {/if}
    </div>
</div>


<style>    
    .saveWrapper {
        all: inherit;
        border: none;
        margin-top: calc(var(--spacing) * 4);
        margin-left: auto;
        border-radius: calc(var(--spacing));
        background: var(--bgLighter);
        width: max-content;
        padding: calc(var(--spacing) * 2);
        padding-inline: calc(var(--spacing) * 10 );
        transition: .12s;
        cursor: pointer;
        
        &:hover {
            background-color: #1971C2;
        }
    }
    
    .Images-Loading-Wrapper {
        display: flex;
        flex-direction: column;
        margin-top: 2rem;
        justify-content: center;
        align-items: center;
        padding-bottom: calc(var(--spacing) * 4);
        
        p {
            margin-top: .7rem;
        }
    }
    
    :global(html:has(#selectCollections.open)) {
        overflow: hidden;
    }
        
    .wrapperMenu {
        display: flex;
        align-items: center;
        padding-bottom: calc(var(--spacing) * 6);
        
        h2 {
            margin: 0;
            padding: 0 !important;
        }
        
        svg {
            width: 1.5rem;
            height: auto;
            opacity: .7;
        }
        
        span {
            display: flex;
            align-items: center;
            justify-content: center;
            margin-left: auto;
            cursor: pointer;
            padding: .3rem;
            border-radius: calc(var(--spacing) * 1);
            
            transition: .12s;
            
            &:hover {
                background: rgba(255,255,255,.05);
            }
        }
    }
    
    .card {
        background: var(--navBG);
        border: 1px solid rgba(255,255,255, .1);
        
        position: relative;
        padding: 1rem;
        width: 100%;
        min-height: 100px;
        border-radius: .4rem;
        min-height: max-content;
        width: 100%;
        max-width: min(600px, 80vw);
                
        h2 {
            padding-top: 0;
            padding-bottom: calc(var(--spacing) * 4);
            display: flex;
            align-items: center;
            gap: .4rem;
            
            svg {
                width: 2rem;
                aspect-ratio: 1 / 1;
                display: inline-block;
            }
        }
        
        h3 {
            padding-top: calc(var(--spacing) * 6);
            padding-bottom: calc(var(--spacing) * 2);
        }
        
        h2 + h3, h2 + .wrapperContainer h3 {
            padding-top: 0;
        }
    }
    
    .container {
        display: flex;
        
        gap: calc(var(--spacing) * 16);
        padding-inline: calc(var(--spacing) * 4);
        align-items: center;
        justify-content: center;
        position: fixed;
        top: 0;
        left: 0;
        background: rgba(0,0,0,.65);
        
        z-index: 999999999999999999;
        width: 100vw;
        height: 100vh;
        transform-origin: center;
        transition: .12s;
        backdrop-filter: blur(12px);
        
        &:not(.open) {
            pointer-events: none;
            opacity: 0;
            scale: .4;
        }
        
    }
</style>