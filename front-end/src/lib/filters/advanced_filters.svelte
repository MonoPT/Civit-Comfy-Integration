<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  
  import { type FilterOption } from "$lib/filter";
  import { ListFilter } from "@lucide/svelte";
  
  import {user_token} from "$lib/state.svelte"
  import API from "$lib/api"
  
  const {filters_state = $bindable<FilterOption>()} = $props()
  let showStatusBar = $state(true);
 
  import { onMount } from "svelte";
  
  let base_model = $state<string[]>([])
  
  onMount(async () => {
    
    const response = await fetch(API.get_base_models(user_token.token))
    
    if (response.status !== 200) return
        
    base_model = await response.json()
  })
  
  let selected_base_models = $state("")
  
  const handle_select_change = (values: string[]) => {
    filters_state.baseModel.selected = {name: "", value: values.join(",")}
    
    selected_base_models = values.join(",")
  }
  
</script>
 
<DropdownMenu.Root>
 <DropdownMenu.Trigger>
  {#snippet child({ props })}
   <Button {...props} variant="ghost" class="font-normal"><ListFilter/> Filters</Button>
  {/snippet}
 </DropdownMenu.Trigger>
 <DropdownMenu.Content preventScroll={false} class="w-56">
  <DropdownMenu.Group>
    <DropdownMenu.Label>Filters</DropdownMenu.Label>
    <DropdownMenu.Separator />
    <DropdownMenu.CheckboxItem closeOnSelect={false} bind:checked={showStatusBar}>Status Bar</DropdownMenu.CheckboxItem>
    {@render selectModel()}
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>


{#snippet selectModel()}
    <Select.Root type="multiple" onValueChange={handle_select_change}>
      <Select.Trigger class="w-full">Base Model ({
          selected_base_models.split(",").filter((str) => str.trim().length > 0).length > 0 ? selected_base_models.split(",").filter((str) => str.trim().length > 0).length : "all"
        })</Select.Trigger>
      <Select.Content preventScroll={false}>
        <Select.Group>
          {#each base_model as model}
               <Select.Item value={model}> {model}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}