<script lang="ts"> 
  import { onDestroy, onMount } from "svelte";
  import API from "$lib/api";
  import { user_token } from "$lib/state.svelte";
    
  let interval: number = 0;
  
  import {type DownloadProgress} from "$lib/composables/model_download_dialogue/downloadManager"
  
  let downloads = $state<DownloadProgress[]>([])
  
  onMount(() => {
     interval = setInterval(async () => {
       const resp = await fetch(API.list_downloads(user_token.token))
       
       if (resp.status !== 200) return
       
       downloads = await resp.json() as DownloadProgress[]
    }, 500)
  })
  
  onDestroy(() => {
    clearInterval(interval)
  })
  
  import TableDownloads from "./table.svelte"
  
</script>
 
<!--<DataTable data={data} {columns} />-->

<TableDownloads downloads={downloads} />