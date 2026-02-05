<script lang="ts">
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    import { onMount } from "svelte";
    
    import { toast } from "svelte-sonner";

    import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    
    import API from "$lib/api";
    import { user_token } from "$lib/state.svelte";
    
    let isOpen = $state(false);
    let isLoading = $state(true)
    
    let item_id_g = $state(0)
    let collection_type_g = $state("Model")
    
    let collections: any[] = $state([])
    let in_collections: any[] = $state([])
    
    onMount(() => window.addEventListener("openCollectionManager", async (e) => {
      //@ts-ignore
      const {item_id, collection_type}: {item_id: number, collection_type: string} = e.detail
      isOpen = false
      isOpen = true
      
      isLoading = true
      item_id_g = item_id
      collection_type_g = collection_type
      collections = []
      in_collections = []
      
      const [res1, res2] = await Promise.all([
          fetch(API.collections_with_media(user_token.token, item_id)),
          fetch(API.get_collections_by_media_type(user_token.token, collection_type)),
      ]);
      
      isLoading = false
      
      if (res1.status !== 200 || res2.status !== 200) {
        toast.error("An error has ocurred. Try again.")
        return
      }
      
      collections = (await res2.json() as any[]).filter((coll) => coll.name !== "comfyui_civit_favorites")
      
      in_collections = await res1.json() as any[]
    }))
</script>

<Dialog.Root open={isOpen}>
    <Dialog.Content style="z-index: 1000">
        <Dialog.Header>
            <Dialog.Title>Manage Collections</Dialog.Title>
            <Dialog.Description>
                Select which collections to add or remove
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4">
            {#if isLoading}
                <div class="flex flex-col items-center py-12">
                    <Spinner />
                    <h2 class="pt-2">Loading</h2>
                </div>
                {:else}
                <div class="flex flex-col gap-2 py-0.5" style="max-height: 50vh; overflow-y: auto;">
                    {#each collections as collection}
                        <div class="flex items-center gap-3">
                        <Checkbox checked={in_collections.find((coll) => coll.collectionId === collection.id)} value={collection.id} id="coll-item-{collection.id}" />
                        <Label for="coll-item-{collection.id}" class="font-normal">{collection.name}</Label>
                        </div>
                    {/each}
                </div>
            {/if}
        </div>
        <Dialog.Footer>
            <Dialog.Close class={buttonVariants({ variant: "outline" })}>Cancel</Dialog.Close>
            <Button disabled={isLoading}>
                {#if isLoading}
                    <Spinner />
                    Loading
                    {:else}
                    Save changes
                {/if}
                
            </Button>
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
