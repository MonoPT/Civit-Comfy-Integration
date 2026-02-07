<script lang="ts">
    let data = $state<{modelVersions: any[]}>({modelVersions: []})
    
    let dialogueState = $state(false)
    
    import {start_download} from "./downloadManager"
    
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    
    import Table from "./table.svelte"
    import { onMount } from "svelte";
    
    import {type ModelDownload} from "./downloadManager"
    
    let needle = $state("")
    
    let table_state = $state({
      total: 0,
      selected: 0
    })
    
    onMount(() => {      
      window.addEventListener("DownloadManagerShowModelVersions", (e) => {
        data = {modelVersions: []}
        dialogueState = false
        
        //@ts-ignore
        const {ModelData} = e.detail
                
        data = ModelData
        dialogueState = true
        needle = ""
        
        handleTableStateChange({target: null} as Event);
      })      
    })
    
    async function handleTableStateChange(e: Event) {
      await new Promise(res => setTimeout(res, 10));
      
      const total_entries = Array.from(document.querySelectorAll<HTMLElement>("#ModelVersionsTable [data-version-id]"))
        .filter((item) => item.getAttribute("data-version-id") !== "headerCheckbox")
      
      const checked_entries = total_entries.filter((item) => {
        return item.getAttribute("data-state") === "checked"
      })
              
      const target = (e.target as HTMLElement)
      
      const dataVId = target?.getAttribute("data-version-id")
      
      table_state.total = total_entries.length
      table_state.selected = checked_entries.length
      
      if (dataVId === "headerCheckbox") {
        if (!dataVId) return
        
        if (table_state.total !== table_state.selected) {
          total_entries.filter((item) => {
            return item.getAttribute("data-state") !== "checked"
          }).forEach((btn) => {
            btn.click()
          })
        } else {
          total_entries.forEach((item) => item.click())
        }
      }      
    }
    
    function start_downloads() {
      const total_entries = Array.from(document.querySelectorAll<HTMLElement>("#ModelVersionsTable [data-version-id]"))
        .filter((item) => item.getAttribute("data-version-id") !== "headerCheckbox")
      
      const checked_entries = total_entries.filter((item) => {
        return item.getAttribute("data-state") === "checked"
      })
      
      if (checked_entries.length < 1) return
      
      const versions = checked_entries.map((entry) => {
        return entry.getAttribute("data-version-id") || ""
      }).filter((entry) => entry.trim().length > 1)
      
      if (versions.length < 1) return
      
      let modelsVersions = data.modelVersions.filter((item) => {
        return versions.includes(`${item.id}`)
      })
      
      
      
      let files: ModelDownload[] = []
      
      modelsVersions.forEach((modelV) => {
        //@ts-ignore
        modelV.files.forEach((file) => {  

          files.push({
            id: parseInt(file.downloadUrl.split("download/models/")[1]),
            //@ts-ignore
            type: data.type,
            model_name: modelV.name,
            //@ts-ignore
            base_model: data.name,
            file_name: file.name,
            based_on_model: modelV.baseModel,
            cover: modelV.images[0].url || "",
            //@ts-ignore
            author_name: data.creator.username,
            published_at: modelV.publishedAt || "",
            //@ts-ignore
            stats: JSON.stringify(data.stats)
          })
        })
      })
                  
      dialogueState = false
      start_download(files)
    }
    
</script>

<Dialog.Root open={dialogueState}>
    <form>
        <Dialog.Content style="max-width: 80vw;">
            <Dialog.Header class="flex flex-row items-center">
                <div class="w-max">
                    <Dialog.Title>Download Model</Dialog.Title>
                    <Dialog.Description class="pt-2">
                        Select which files you which to download
                    </Dialog.Description>
                </div>
                <div class="ml-auto mr-12 w-full">
                    <Input bind:value={needle} type="text" placeholder="Search by model name" class="ml-auto" style="max-width: 400px;"  />
                </div>
            </Dialog.Header>
            <div class="grid gap-4" style="max-height: 60vh; overflow-y: auto;">
                <Table data={data} needle={needle} callback={handleTableStateChange} />
            </div>
            <Dialog.Footer class="w-full flex flex-row justify-between! items-center">
                <div class="text-muted-foreground flex-1 text-sm">
                    {table_state.selected} of {table_state.total} Version(s) selected
                </div>
                <div class="div">
                    <Dialog.Close class={buttonVariants({ variant: "outline" })}>Cancel</Dialog.Close>
                    <Button type="submit" onclick={start_downloads} disabled={table_state.selected < 1}>Download Models</Button>
                </div>
            </Dialog.Footer>
        </Dialog.Content>
    </form>
</Dialog.Root>
