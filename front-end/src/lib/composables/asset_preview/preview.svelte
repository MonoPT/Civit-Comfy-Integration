<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    
    let data = $state<null | any>(
      /*{
          "img_url": "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/4278af43-a356-4bd1-81aa-dea1c75ae335/transcode=true,original=true,quality=100/3cde953f-b339-426a-97fa-9b47071c1df6",
          "id": 117581125,
          "reactionCount": 3127,
          "commentCount": 9,
          "collectedCount": 39,
          "index": 15,
          "postId": 25901513,
          "url": "4278af43-a356-4bd1-81aa-dea1c75ae335",
          "aiNsfwLevel": 0,
          "hash": "UDCPO:}=bw%MVrxvsnWB9Zs;n$NH-;s8X9NH",
          "width": 1088,
          "height": 1408,
          "nsfwLevel": 1,
          "createdAt": "2026-01-15T16:46:23.008Z",
          "stats": {
              "likeCountAllTime": 1601,
              "laughCountAllTime": 666,
              "heartCountAllTime": 682,
              "cryCountAllTime": 178,
              "commentCountAllTime": 9,
              "collectedCountAllTime": 39,
              "tippedAmountCountAllTime": 679,
              "dislikeCountAllTime": 0,
              "viewCountAllTime": 0
          },
          "meta": null,
          "baseModel": null,
          "modelVersionIds": [],
          "modelVersionId": 2600396,
          "sortAt": "2026-01-15T16:46:23.008Z",
          "type": "image",
          "userId": 81430,
          "hasMeta": true,
          "onSite": false,
          "postedToId": 2600396,
          "combinedNsfwLevel": 1,
          "toolIds": [],
          "techniqueIds": [],
          "existedAtUnix": 1768496341579,
          "sortAtUnix": 1768495583008,
          "tagIds": [
              3739,
              4262,
              5132,
              7954,
              111768,
              115513,
              116352,
              120012,
              122902,
              129427,
              161848,
              162242,
              162710,
              162946,
              162947,
              162968,
              162969,
              163939,
              164145,
              164146,
              164288,
              167411,
              234268
          ],
          "tags": [],
          "modelVersionIdsManual": [
              2600396
          ],
          "minor": false,
          "remixOfId": null,
          "hasPositivePrompt": true,
          "availability": "Public",
          "poi": false,
          "acceptableMinor": false,
          "publishedAt": "2026-01-15T16:46:23.008Z",
          "user": {
              "id": 81430,
              "username": "Sophorium",
              "image": "11d7576e-488d-4ae1-9ccb-f342e713bc09",
              "deletedAt": null,
              "profilePicture": null
          },
          "optimized_asset_url": "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/4278af43-a356-4bd1-81aa-dea1c75ae335/transcode=true,width=400,original=false,optimized=true/3cde953f-b339-426a-97fa-9b47071c1df6",
          "optimized_poster_img_url": "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/4278af43-a356-4bd1-81aa-dea1c75ae335/anim=false,transcode=true,width=400,original=false,optimized=true/3cde953f-b339-426a-97fa-9b47071c1df6",
          "ratio": {
              "w": 17,
              "h": 22
          }
      }*/
      null
    )
    
    let isOpen = $state(false)
    let favoriteIsLoading = $state(true)
    
    let collections = $state<any[]>([])
    let media_in_collections = $state<any[]>([])
    let isFavorite = $state(false)
    
    onMount(() => {
      window.addEventListener("ViewMedia", async (e) => {
        isOpen = false
        isFavorite = false
        isOpen = true
        media_in_collections = []
        
        //@ts-ignore
        data = e.detail
        
        update_collections_l()
      })
    })
    
    async function update_collections_l() {
      favoriteIsLoading = true;;
      const media_id = data.id;
      
      let resp = await update_collections(media_id)
          
      favoriteIsLoading = false
      
      collections = resp.collections
      isFavorite = resp.isFavorite
      media_in_collections = resp.media_in_collections
    }
    
    import * as Item from "$lib/components/ui/item/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    import { Heart, Bookmark, ArrowDownToLine } from "@lucide/svelte";
    import { onMount } from "svelte";
    
    import {update_collections, favorite_media} from "$lib/AssetsUtils"
    
    async function favorite_media_l() {
      favoriteIsLoading = true
      const media_id = data.id;
      
      const resp = await favorite_media(media_id, isFavorite)
      
      favoriteIsLoading = false
      
      collections = resp.collections
      isFavorite = resp.isFavorite
      media_in_collections = resp.media_in_collections
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
                    <div class= "flex ">
                        {@render quickActions()}
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
</style>

{#snippet quickActions()}
    <Item.Root style="background: var(--background); width: 100%; height: max-content;">
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
         </div>
     </Item.Content>
     <Item.Actions />
    </Item.Root>
{/snippet}