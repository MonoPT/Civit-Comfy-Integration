<script lang="ts">
  import DataTable from "./data-table.svelte";
  import { columns } from "./columns.js";
 
  import {data} from "./data.ts"
</script>
 
<DataTable data={data} {columns} />