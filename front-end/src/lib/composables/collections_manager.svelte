<script lang="ts">
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    import { onMount } from "svelte";
    
    import { toast } from "svelte-sonner";

    import {Plus, ArrowLeft } from "@lucide/svelte"
    
    import { Input } from "$lib/components/ui/input/index.js";
    import { Checkbox } from "$lib/components/ui/checkbox/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Textarea } from "$lib/components/ui/textarea/index.js";
    
    import API from "$lib/api";
    import { user_token } from "$lib/state.svelte";
    
    let isOpen = $state(false);
    let isLoading = $state(true)
    let isCreatingCollection = $state(true)
    
    let item_id_g = $state(0)
    let collection_type_g = $state("Model")
    
    let collections: any[] = $state([])
    let in_collections: any[] = $state([])
    
    onMount(() => window.addEventListener("openCollectionManager", async (e) => {
      //@ts-ignore
      const {item_id, collection_type}: {item_id: number, collection_type: string} = e.detail
      
      item_id_g = item_id
      collection_type_g = collection_type
      
      isOpen = false
      isOpen = true
      
      isLoading = true
      
      collections = []
      in_collections = []
      
      collection_name = ""
      collection_desc = ""
      
      await update_collections(item_id, collection_type_g)
      
      isLoading = false
    }))
    
    async function update_collections(item_id: number, collection_type: string) {
      const [res1, res2] = await Promise.all([
          fetch(API.collections_with_media(user_token.token, item_id)),
          fetch(API.get_collections_by_media_type(user_token.token, collection_type)),
      ]);
      
      if (res1.status !== 200 || res2.status !== 200) {
        toast.error("An error has ocurred. Try again.")
        return
      }
      
      collections = (await res2.json() as any[]).filter((coll) => coll.name !== "comfyui_civit_favorites")
      
      in_collections = await res1.json() as any[]
    }
    
    async function handle_submit() {
      isLoading = true
      
      const collectionItems = Array.from(document.querySelectorAll<HTMLInputElement>("[data-collection-item]")).map((item) => {
        const collection_id = parseInt(item.getAttribute("data-collection-item")!)
        const checked = item.getAttribute("data-state") === "checked" ? true : false

        return {collection_id, checked}
      })
      
      await fetch(API.update_media_collections(user_token.token, item_id_g, 
        collectionItems.filter((item) => item.checked).map((item) => item.collection_id),
        collectionItems.filter((item) => !item.checked).map((item) => item.collection_id)
      ))  
      
      await update_collections(item_id_g, collection_type_g)
      
      isLoading = false
    }
    
    let collection_name = $state("")
    let collection_desc = $state("")
    
    async function createCollection() {
      if (collection_name.trim().length < 1) return
      
      isLoading = true
      
      await fetch(API.create_collection(user_token.token, collection_name, collection_type_g, true, collection_desc))
      
      isCreatingCollection = false
      
      await update_collections(item_id_g, collection_type_g)
      
      collection_name = ""
      collection_desc = ""
      
      isLoading = false
    }
</script>

<Dialog.Root open={isOpen}>
    <Dialog.Content style="z-index: 1000">
        <Dialog.Header>
            <Dialog.Title>Manage Collections</Dialog.Title>
            <Dialog.Description>
                {#if !isCreatingCollection}
                    <div class="flex items-center justify-between">
                        <div>
                            Select which collections to add or remove
                        </div>
                        <div>
                            <Button variant="ghost" aria-label="Submit"
                                onclick={() => isCreatingCollection = true}
                            >
                                <Plus />
                                New collection
                            </Button>
                        </div>
                    </div>
                    {:else}
                    <div class="flex items-center justify-between">
                        <div>
                            Create a new collection
                        </div>
                        <div>
                            <Button variant="ghost" aria-label="Submit"
                                onclick={() => isCreatingCollection = false}
                            >
                                <ArrowLeft />
                                Back to selection
                            </Button>
                        </div>
                    </div>
                {/if}
                
            </Dialog.Description>
        </Dialog.Header>
        <div class="grid gap-4">
            {#if isLoading && !isCreatingCollection}
                <div class="flex flex-col items-center py-6">
                    <Spinner />
                    <h2 class="pt-2">Loading</h2>
                </div>
                {:else if !isCreatingCollection}
                <div class="flex flex-col gap-2 py-0.5" style="max-height: 50vh; overflow-y: auto;">
                    {#each collections as collection}
                        <div class="flex items-center gap-3">
                        <Checkbox data-collection-item={collection.id} checked={in_collections.find((coll) => coll.collectionId === collection.id) ? true : false} value={collection.id} id="coll-item-{collection.id}" />
                        <Label for="coll-item-{collection.id}" class="font-normal">{collection.name}</Label>
                        </div>
                    {/each}
                </div>
            {/if}
            
            {#if isCreatingCollection}
                <form class="flex flex-col gap-2">
                    <Input bind:value={collection_name} required placeholder="Name" name="name" class="w-full" />
                    <Textarea bind:value={collection_desc} placeholder="Description" />
                </form>
            {/if}
        </div>
        <Dialog.Footer>
            <Dialog.Close class={buttonVariants({ variant: "outline" })}>Cancel</Dialog.Close>
            
            {#if !isCreatingCollection}
                <Button disabled={isLoading} onclick={handle_submit}>
                    {#if isLoading}
                        <Spinner />
                        Loading
                        {:else}
                        Save changes
                    {/if}
                </Button>
                {:else}
                <Button disabled={isLoading}
                    onclick={createCollection}
                >
                    {#if isLoading}
                        <Spinner />
                        Loading
                        {:else}
                        Create
                    {/if}
                </Button>
            {/if}
            
            
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
