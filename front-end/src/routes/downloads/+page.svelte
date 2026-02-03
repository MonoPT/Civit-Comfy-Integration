<script lang="ts">
  import DataTable from "./data-table.svelte";
  import { columns } from "./columns.js";
 
  import {data as demoData} from "./data.ts"
  
  let data = $state(demoData)
  
</script>
 
<DataTable data={data} {columns} />