<script lang="ts">
  import DataTable from "./data-table.svelte";
  import { columns } from "./columns.js";
 
  import {data as demoData} from "./data.ts"
    import { onDestroy, onMount } from "svelte";
    import API from "$lib/api";
    import { user_token } from "$lib/state.svelte";
  
  let data = $state(demoData)
  
  let interval: number = 0;
  
  onMount(() => {
     interval = setInterval(async () => {
       const resp = await fetch(API.list_downloads(user_token.token))
       
       if (resp.status !== 200) return
       
       const downloads = await resp.json()
       
       console.log(downloads)
       
      // add download update logic here
    }, 500)
  })
  
  onDestroy(() => {
    clearInterval(interval)
  })
  
</script>
 
<DataTable data={data} {columns} />