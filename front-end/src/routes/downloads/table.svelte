<script lang="ts">
    import * as Table from "$lib/components/ui/table/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    import byteSize from "byte-size";
    
    import { Plus } from "@lucide/svelte";

    import {get_model_data} from "$lib/apis/model_data"
    
    import {type DownloadProgress} from "$lib/composables/model_download_dialogue/downloadManager"
    
    //@ts-ignore
    import { toast } from "svelte-sonner";
    
    let add_download_dialog = $state(false)
    let is_fetching_model_data = $state(false)
    
    async function handleModelDownload(e: Event) {
      is_fetching_model_data = true
      const model_param = document.querySelector<HTMLInputElement>("#modelParam")!.value.trim()
      
      let resp = await get_model_data(model_param)
            
      is_fetching_model_data = false
      
      if (resp.status !== 200) {
        toast.error("The provided url/id is not valid")
        return
      }
      
      add_download_dialog = false
      
      window.dispatchEvent(new CustomEvent("DownloadManagerShowModelVersions", {
        detail: {
          ModelData: resp.data
        }
      }))
    }
    
    const options: Intl.DateTimeFormatOptions = {
        year: "numeric",
        month: "long",
        day: "numeric",
    };
    
    
    function titleCaseWord(word: string) {
      if (!word) return word;
      return word[0].toUpperCase() + word.substr(1).toLowerCase();
    }
    
    const  {downloads} : {downloads: DownloadProgress[]} = $props()
</script>


{@render add_model()}

{#if downloads.length < 1} 
    {@render firstModelMessage()}
    {:else}
    <div class="flex items-center py-4">
        <div class="actions flex items-center ml-auto">
            <Button onclick={() => {add_download_dialog = false; add_download_dialog = true}} variant="outline"><Plus /> Download model</Button>
        </div>
    </div>
{/if}

<Table.Root>
    <!--<Table.Header>
        <Table.Row>
            <Table.Head class=""></Table.Head>
            <Table.Head></Table.Head>
            <Table.Head></Table.Head>
            <Table.Head class="text-end"></Table.Head>
        </Table.Row>
        </Table.Header>-->
    <Table.Body>
        {#each downloads as download (download.id)}
            <Table.Row>
                <Table.Cell class="font-medium" style="width: 160px;"><img style="width: 100%;" class="block rounded-xs" src="{download.cover}" alt=""></Table.Cell>
                <Table.Cell>
                    <div class="w-full h-full">
                      <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight">{download.base_model}</h2>
                      <div class="model-details flex align-middle pt-2 gap-4">
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-user-icon lucide-user"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                              {download.author_name.replace("\"", "")}
                          </span>
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-layers-icon lucide-layers"><path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12"/><path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17"/></svg>
                              {download.based_on_model}
                          </span>
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-calendar-icon lucide-calendar"><path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/></svg>
                              {new Date(download.published_at).toLocaleDateString(
                                  "en-UK",
                                  options,
                              )}
                          </span>
                      </div>
                      <div class="model-details flex align-middle pt-4 gap-4">
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-down-to-line-icon lucide-arrow-down-to-line"><path d="M12 17V3"/><path d="m6 11 6 6 6-6"/><path d="M19 21H5"/></svg>
                              {JSON.parse(download.stats.slice(0, -1)).downloadCount}
                          </span>
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-thumbs-up-icon lucide-thumbs-up"><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"/><path d="M7 10v12"/></svg>
                              {JSON.parse(download.stats.slice(0, -1)).thumbsUpCount}
                          </span>
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg style="rotate: 180deg;" xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-thumbs-up-icon lucide-thumbs-up"><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"/><path d="M7 10v12"/></svg>
                              {JSON.parse(download.stats.slice(0, -1)).thumbsDownCount}
                          </span>
                          <span class="flex items-center gap-2 text-muted-foreground">
                              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-zap-icon lucide-zap"><path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"/></svg>
                              {JSON.parse(download.stats.slice(0, -1)).tippedAmountCount}
                          </span>
                      </div>
                      <div class="model-details flex align-middle pt-8 gap-2 w-full flex-col text-muted-foreground" style="max-width: 400px">
                          <div class="flex items-center">
                              <span>{titleCaseWord(download.status)}</span>
                              <span class="block ml-auto" style="font-size: .85em">{byteSize(Math.round(download.download_speed)).toString()}/s ({byteSize(Math.round(download.downloaded)).toString()} / {byteSize(Math.round(Math.max(download.total_size, 1))).toString()})</span>
                          </div>
                          <div class="block w-full">
                              <div data-slot="progress" class="bg-primary/20 relative h-2 overflow-hidden rounded-full w-full" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="66" data-value="66" data-state="loading" data-max="100" data-min="0" data-progress-root=""><div data-slot="progress-indicator" class="bg-primary h-full w-full flex-1 transition-all" 
                              style="transform: translateX(-{(100 - (download.downloaded * 100) / download.total_size)}%);"></div></div>
                          </div>
                      </div>
                    </div>
                </Table.Cell>
                <Table.Cell>
                    {download.model_name}
                </Table.Cell>
                <Table.Cell>
                     {byteSize(Math.round(download.total_size)).toString()}
                </Table.Cell>
                
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>


{#snippet add_model()}
    <Dialog.Root open={add_download_dialog}>
      <form>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>Download a new model</Dialog.Title>
          </Dialog.Header>
          <div class="grid gap-4">
              <Input min="1" id="modelParam" type="text" required placeholder="Paste the model id or page url" class="w-full" />
          </div>
          <Dialog.Footer>
            <Dialog.Close class={buttonVariants({ variant: "outline" })}
              >Cancel</Dialog.Close>
            <Button type="submit" disabled={is_fetching_model_data} onclick={handleModelDownload}>
                {#if is_fetching_model_data}
                    <Spinner />
                {/if}
                Download
            </Button>
          </Dialog.Footer>
        </Dialog.Content>
      </form>
    </Dialog.Root>
{/snippet}

{#snippet firstModelMessage()}
    <div class="flex flex-col items-center py-4 gap-6">
        <h2 class="text-center">You haven't downloaded any models yet.</h2>
        <div class="actions flex items-center mx-auto">
            <Button onclick={() => {add_download_dialog = false; add_download_dialog = true}} variant="outline"><Plus /> Download model</Button>
        </div>
    </div>
{/snippet}