<script lang="ts">
    const {asset} = $props()
    
    import { Skeleton } from "$lib/components/ui/skeleton/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    
    let elRef: HTMLElement;
    
    import { Bookmark, ThumbsUp, EllipsisVertical, Laugh, Heart, Frown, MessageSquare, ArrowDownToLine } from "@lucide/svelte";
    
    import {update_collections, favorite_media} from "$lib/AssetsUtils"
    import { onMount } from "svelte";
    
    let favoriteIsLoading = $state(true)
    let isFavorite = $state(false)
    let inCollections: any[] = $state([])
    
    async function favorite_media_l() {
      favoriteIsLoading = true
      
      const resp = await favorite_media(asset.id, isFavorite, "Image")
      
      favoriteIsLoading = false
      isFavorite = resp.isFavorite
      inCollections = resp.media_in_collections
    }
    
    onMount(() => elRef.addEventListener("pointerover", async (e) => {
      const data = await update_collections(asset.id, "Image")
      
      favoriteIsLoading = false
                  
      isFavorite = data.isFavorite
      inCollections = data.media_in_collections
      
    }, {once: true}))
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div role="button" tabindex="0" data-asset-container  bind:this={elRef} style="outline: none !important;"
    onclick={(e) => {
      if ((e.target! as HTMLElement).closest("[data-stats]")) return
      
      window.dispatchEvent(new CustomEvent("ViewMedia", {
        detail: asset
      }))
    }}
    class="asset-container" data-cover={asset.optimized_poster_img_url} data-media={asset.optimized_asset_url}
>
    <Skeleton class="skeleteonLoader h-full w-full absolute top-0 left-0" />
    <!--<video loop autoplay={false} muted preload="auto" poster={asset.optimized_poster_img_url} disablepictureinpicture>
        <source src="{asset.optimized_asset_url}.webm" type="video/webm">
        <source src="{asset.optimized_asset_url}.mp4" type="video/mp4">
    </video>-->
    <div class="overlay">
        
        <div data-stats class="stats absolute bottom-1.5 left-1.5 flex items-center gap-2 w-full px-2">
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
            
            <div class="ml-auto">
                <Button type="button" disabled={favoriteIsLoading} onclick={favorite_media_l} variant="ghost" size="icon" aria-label="Favorite">
                   {#if favoriteIsLoading}
                       <Spinner />
                       {:else}
                       <Heart fill={isFavorite ? "#fff" : "transparent"} />
                   {/if}
                </Button>
                <Button type="button" variant="ghost" size="icon" aria-label="Collections"
                    onclick={() => window.dispatchEvent(new CustomEvent("openCollectionManager", {
                      detail: {
                        item_id: asset.id,
                        collection_type: "Image"
                      }
                    }))}
                >
                    <Bookmark />
                </Button>
            </div>
        </div>
    </div>
</div>

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

<style>
    .asset-container {
        border-radius: 5px;
        overflow: hidden;
        position: relative;
        width: 100%;
        height: 100%;
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