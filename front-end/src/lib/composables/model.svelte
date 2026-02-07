<script lang="ts">
    const {asset} = $props()
    
    import { Skeleton } from "$lib/components/ui/skeleton/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    
    let elRef: HTMLElement;
    
    import { Bookmark, ThumbsUp, EllipsisVertical,Library, ThumbsDown, Zap, Heart, MessageSquare, ArrowDownToLine } from "@lucide/svelte";
    
    import {update_collections, favorite_media} from "$lib/AssetsUtils"
    import { onMount } from "svelte";
    
    let favoriteIsLoading = $state(true)
    let isFavorite = $state(false)
    let inCollections: any[] = $state([])
    
    async function favorite_media_l() {
      favoriteIsLoading = true
      
      const resp = await favorite_media(asset.id, isFavorite, "Model")
      
      favoriteIsLoading = false
      isFavorite = resp.isFavorite
      inCollections = resp.media_in_collections
    }
    
    onMount(() => elRef.addEventListener("pointerover", async (e) => {
      const data = await update_collections(asset.id, "Model")
      
      favoriteIsLoading = false
                  
      isFavorite = data.isFavorite
      inCollections = data.media_in_collections
      
    }, {once: true}))
    
    const formatter = new Intl.NumberFormat('en-UK', {
      useGrouping: true,
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div role="button" tabindex="0" data-asset-container  bind:this={elRef} style="outline: none !important;"
    onclick={(e) => {
      if ((e.target! as HTMLElement).closest("[data-stats]")) return
      
      window.dispatchEvent(new CustomEvent("ViewModel", {
        detail: asset
      }))
    }}
    class="asset-container" data-cover={asset.optimized_poster_img_url} 
    data-media={asset.optimized_asset_url}
    data-media-w={asset.ratio.w}
    data-media-h={asset.ratio.h}
>
    <Skeleton class="skeleteonLoader h-full w-full absolute top-0 left-0" />
    <!--<video loop autoplay={false} muted preload="auto" poster={asset.optimized_poster_img_url} disablepictureinpicture>
        <source src="{asset.optimized_asset_url}.webm" type="video/webm">
        <source src="{asset.optimized_asset_url}.mp4" type="video/mp4">
    </video>-->
    <div class="overlay ">
        <div data-stats class="stats absolute top-1.5 flex items-center gap-2 w-full px-2">
            
            <Button variant="outline" size="sm" class="text-sm">
              <ThumbsUp /> {formatter.format(asset.rank.thumbsUpCount).replaceAll(",", ' ')}
            </Button>
            
            <Button variant="outline" size="sm" class="text-sm">
              <ArrowDownToLine /> {formatter.format(asset.rank.downloadCount).replaceAll(",", ' ')}
            </Button>
            
            
            {@render reactionsDropdown(
              asset.rank.thumbsDownCount,
              asset.rank.commentCount,
              asset.rank.collectedCount,
              asset.rank.tippedAmountCount,
            )}
        </div>
        
        <div class="absolute bottom-5 flex flex-col gap-0 w-full px-2 z-12">
            <div class="name font-semibold clampText">{asset.name}</div>
            <div class="name font-normal flex items-center w-full">
                <div><Badge variant="outline">{asset.type}</Badge></div>
                <div class="ml-auto" data-actions>
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
                            collection_type: "Model"
                          }
                        }))}
                    >
                        <Bookmark />
                    </Button>
                </div>
            </div>
        </div>
    </div>
</div>

{#snippet reactionsDropdown(downvote: number, comment: number, collected: number, tipped: number)}
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
                <ThumbsDown  /> {formatter.format(downvote).replaceAll(",", ' ')}
            </Button>
        </DropdownMenu.CheckboxItem>
        
        <DropdownMenu.CheckboxItem class="p-0 w-full" style="pointer-events: none;">
            <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
                <MessageSquare  /> {formatter.format(comment).replaceAll(",", ' ')}
            </Button>
        </DropdownMenu.CheckboxItem>
        
        <DropdownMenu.CheckboxItem class="p-0 w-full" style="pointer-events: none;">
            <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
                <Library  /> {formatter.format(collected).replaceAll(",", ' ')}
            </Button>
        </DropdownMenu.CheckboxItem>
        
        <DropdownMenu.CheckboxItem class="p-0 w-full" style="pointer-events: none;">
            <Button class="w-1/1 bg-transparent" variant="ghost" size="sm">
                <Zap  /> {formatter.format(tipped).replaceAll(",", ' ')}
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
        --bgGT: linear-gradient(to top, color-mix(in lab, var(--background) 50%, transparent 50%) 15% ,color-mix(in lab, var(--background) 50%, transparent 70%) 30%, transparent 100%);
        
        position: absolute;
        z-index: 2;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        pointer-events: none;
        background: var(--bgGT);
        
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
            background: linear-gradient(to bottom, color-mix(in lab, var(--background) 80%, transparent 20%) 15% ,color-mix(in lab, var(--background) 50%, transparent 70%) 30%, transparent 100%),
            var(--bgGT);
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
    
    [data-actions] {
        transition: .12s;
    }
    
    .asset-container:not(:hover) [data-actions] {
        opacity: 0 !important;
    }
    
    .clampText {
       overflow: hidden;
       display: -webkit-box;
       -webkit-line-clamp: 2; /* number of lines to show */
               line-clamp: 2; 
       -webkit-box-orient: vertical;
    }
</style>