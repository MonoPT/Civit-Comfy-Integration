<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import * as Accordion from "$lib/components/ui/accordion/index.js";
    
    let data = $state<null | any>(null)
    
    let gen_data = $state<null | any>(null)
    
    let isOpen = $state(false)
    let favoriteIsLoading = $state(true)
    
    let isFavorite = $state(false)
    
    import {Workflow, Cog} from "@lucide/svelte"
    
    onMount(() => {
      window.addEventListener("ViewMedia", async (e) => {
        isOpen = false
        isFavorite = false
        isOpen = true
        
        gen_data = null
        
        //@ts-ignore
        data = e.detail
        
        //console.log(data)
        
        //@ts-ignore
        const media_id = e.detail.id
        
        update_collections_l()
        
        let resp = await fetch(API.get_gen_data(user_token.token, media_id))
        
        if (resp.status !== 200) return
        
        const d = await resp.json()
                
        gen_data = d
      })
    })
    
    async function update_collections_l() {
      favoriteIsLoading = true;;
      const media_id = data.id;
      
      let resp = await update_collections(media_id)
          
      favoriteIsLoading = false
      
      isFavorite = resp.isFavorite
    }
    
    import * as Item from "$lib/components/ui/item/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    import { Heart, Bookmark, ArrowDownToLine, X } from "@lucide/svelte";
    import { onMount } from "svelte";
    
    import {update_collections, favorite_media} from "$lib/AssetsUtils"
    import API from "$lib/api";
    import { user_token } from "$lib/state.svelte";
    
    async function favorite_media_l() {
      favoriteIsLoading = true
      const media_id = data.id;
      
      const resp = await favorite_media(media_id, isFavorite)
      
      favoriteIsLoading = false
      
      isFavorite = resp.isFavorite
    }
</script>

<Dialog.Root open={isOpen}>
    <form>
        <Dialog.Content class="backdrop-blur-sm grid justify-center" style="max-width: 100vw; height: 100vh; border-radius: 0; border: none; background: transparent; ">
            <div class="flex items-center justify-center" style="width: 100vw; height: 100vh;">
                <div id="media-preview" style="flex-shrink: 0 ;width: 100%; min-height: 30px ;max-width: 85vw; max-height: 85vh; overflow-y: auto;">
                    <div class="media-container" style="max-height: 85vh;">
                        <video style="aspect-ratio: {data.ratio.w} / {data.ratio.h};" loop autoplay muted preload="auto" poster={data.img_url} disablepictureinpicture>
                            <source src="{data.img_url}.webm" type="video/webm">
                            <source src="{data.img_url}.mp4" type="video/mp4">
                        </video>
                    </div>
                    <div class= "flex flex-col gap-2" style="overflow-y: auto; max-height: 85vh;">
                        {@render quickActions()}
                        {#if gen_data}
                            {@render proccess()}
                        {/if}
                        
                        {#if gen_data}
                            {@render generation_data()}
                        {/if}
                    </div>
                </div>
            </div>
        </Dialog.Content>
    </form>
</Dialog.Root>

<style>
    video {
        position: relative;
        display: block;
        height: 100%;
        width: 100%;
        max-height: 100%;
    }
    
    #media-preview {
        display: grid;
        grid-template-columns: 2fr 2fr;
        gap: 2rem;
    }
    
    .clampText {
       overflow: hidden;
       display: -webkit-box;
       -webkit-line-clamp: 1; /* number of lines to show */
               line-clamp: 1; 
       -webkit-box-orient: vertical;
    }
    
    .clampText-4 {
       overflow: hidden;
       display: -webkit-box;
       -webkit-line-clamp: 4; /* number of lines to show */
               line-clamp: 4; 
       -webkit-box-orient: vertical;
    }
</style>

{#snippet quickActions()}
    <Item.Root class="block" style="background: var(--background); width: 100%; height: max-content;">
     <Item.Content>
         <div class="flex items-center gap-1.5">
             <Button disabled={favoriteIsLoading} onclick={favorite_media_l} variant="ghost" size="icon" aria-label="Favorite">
                {#if favoriteIsLoading}
                    <Spinner />
                    {:else}
                    <Heart fill={isFavorite ? "#fff" : ""} />
                {/if}
             </Button>
             
             <Button variant="ghost" size="icon" aria-label="Add/Remove collection"
                 onclick={() => window.dispatchEvent(new CustomEvent("openCollectionManager", {
                   detail: {
                     item_id: data.id,
                     collection_type: "Image"
                   }
                 }))}
             >
                 <Bookmark/>
             </Button>
             
             <Button variant="ghost" size="icon" aria-label="Download">
                 <ArrowDownToLine />
             </Button>
             
             <Button variant="ghost" size="icon" aria-label="Close" class="ml-auto" onclick={() => isOpen = false}>
                 <X />
             </Button>
         </div>
     </Item.Content>
     <Item.Actions />
    </Item.Root>
{/snippet}

{#snippet proccess()}
    {#if gen_data.tools.length > 0 || gen_data.techniques.length > 0}
        <Item.Root class="block" style="background: var(--background); width: 100%; height: max-content;">
        <Item.Content>
            <div class="flex flex-col gap-1.5">
                <div class="flex gap-2 items-center">
                    <Workflow />
                    <h2 class="text-2xl font-semibold">Proccess</h2>
                </div>
                
                {#if gen_data.tools.length > 0}
                    <div class="pt-2">
                        <h3 class="font-semibold">Tools</h3>
                        <div class="flex flex-wrap gap-1.5 pt-1.5">
                            {#each gen_data.tools as tools}
                                <Badge variant="outline">{tools.name}</Badge>
                            {/each}
                        </div>
                    </div>
                {/if}
                
                
                {#if gen_data.techniques.length > 0}
                    <div class="pt-2">
                        <h3 class="font-semibold">Techniques</h3>
                        <div class="flex flex-wrap gap-1.5 pt-1.5">
                            {#each gen_data.techniques as techni}
                                <Badge variant="outline">{techni.name}</Badge>
                            {/each}
                        </div>
                    </div>
                {/if}
            </div>
        </Item.Content>
        <Item.Actions />
        </Item.Root>
    {/if}
{/snippet}

{#snippet generation_data()}
    <Item.Root class="block" style="background: var(--background); width: 100%; height: max-content;">
     <Item.Content>
         <div class="flex flex-col gap-1.5">
            <div class="flex gap-2 items-center">
                <Cog />
                <h2 class="text-2xl font-semibold">Generation data</h2>
            </div>
             
            {#if gen_data.resources.length > 0}
                <div class="pt-2">
                    <h3 class="font-semibold">Resources used</h3>
                    <div class="flex flex-wrap gap-1.5 pt-1.5">
                        {#each gen_data.resources as resource}
                            <a href="#?{resource.modelId}" class="pt-1.5 w-full" style="display: grid;gap: 1rem ;grid-template-columns: 1fr auto;">
                                <div class="clampText underline">{resource.modelName}</div> 
                                <Badge variant="outline" class="bg-sky-400 text-white dark:bg-sky-600">{resource.modelType}</Badge>
                            </a>
                        {/each}
                    </div>
                </div>
            {/if}
            
            {#if gen_data.meta?.prompt || gen_data.meta?.negativePrompt}
                <div class="pt-2">
                    <Accordion.Root type="multiple" class="w-full">
                        {#if gen_data.meta.prompt}
                            <Accordion.Item>
                                <Accordion.Trigger>Prompt</Accordion.Trigger>
                                <Accordion.Content class="flex flex-col gap-4 text-balance">
                                    <p class="text-muted-foreground clampText-4">{gen_data.meta.prompt}</p>
                                </Accordion.Content>
                            </Accordion.Item>
                        {/if}
                        
                        {#if gen_data.meta.negativePrompt}
                            <Accordion.Item>
                                <Accordion.Trigger>Negative prompt</Accordion.Trigger>
                                <Accordion.Content class="flex flex-col gap-4 text-balance">
                                    <p class="text-muted-foreground clampText-4">{gen_data.meta.negativePrompt}</p>
                                </Accordion.Content>
                            </Accordion.Item>
                        {/if}
                    </Accordion.Root>
                </div>
            {/if}
         </div>
     </Item.Content>
     <Item.Actions />
    </Item.Root>
{/snippet}
