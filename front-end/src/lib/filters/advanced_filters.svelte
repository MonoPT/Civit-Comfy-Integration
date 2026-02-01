<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  
  import { type FilterOption } from "$lib/filter";
  import { ListFilter } from "@lucide/svelte";
  
  import {user_token} from "$lib/state.svelte"
  import API from "$lib/api"
  
  const {filters_state = $bindable<FilterOption>()} = $props()
 
  import { onMount } from "svelte";
  
  let base_model = $state<string[]>([])
  let tools = $state<{id: number, name: string}[]>([])
  let techniques = $state<{id: number, name: string, type: string}[]>([])
  
  onMount(async () => {
    fetch(API.get_base_models(user_token.token)).then(async (response) => {
      if (response.status !== 200) return
      base_model = await response.json()
    })
    
    fetch(API.get_tools(user_token.token)).then(async (response) => {
      if (response.status !== 200) return
      let data = await response.json() as object[]
      
      tools = data.map((tool: any) => {
        return {id: tool.id, name: tool.name, }
      })
    })
    
    fetch(API.get_techniques(user_token.token)).then(async (response) => {
      if (response.status !== 200) return
      let data = await response.json() as object[]
      
      techniques = data.map((technique: any) => {
        return {id: technique.id, name: technique.name, type: technique.type }
      })
    })
  })
  
  let selected_base_models = $state("")
  let selected_tools = $state("")
  let selected_techniques = $state("")
  
  const handle_select_change_base_model = (values: string[]) => {
    filters_state.baseModel.selected = {name: "", value: values.join(",")}
    
    selected_base_models = values.join(",")
  }
  
  const handle_select_change_tool = (values: string[]) => {
    filters_state.tools.selected = {name: "", value: values.join(",")}
    
    selected_tools = values.join(",")
  }
  
  const handle_select_change_technique = (values: string[]) => {
    filters_state.techniques.selected = {name: "", value: values.join(",")}
    
    selected_techniques = values.join(",")
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
    {@render selectModel()}
    {@render selectTools()}
    {@render selectTechniques()}
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>


{#snippet selectModel()}
    <Select.Root type="multiple" onValueChange={handle_select_change_base_model}>
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

{#snippet selectTools()}
    <Select.Root type="multiple" onValueChange={handle_select_change_tool}>
      <Select.Trigger class="w-full">Tools ({
          selected_tools.split(",").filter((str) => str.trim().length > 0).length > 0 ? selected_tools.split(",").filter((str) => str.trim().length > 0).length : "all"
        })</Select.Trigger>
      <Select.Content preventScroll={false}>
        <Select.Group>
          {#each tools as tool}
               <Select.Item value={`${tool.id}`}> {tool.name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}

{#snippet selectTechniques()}
    <Select.Root type="multiple" onValueChange={handle_select_change_technique}>
      <Select.Trigger class="w-full">Techniques ({
          selected_tools.split(",").filter((str) => str.trim().length > 0).length > 0 ? selected_tools.split(",").filter((str) => str.trim().length > 0).length : "all"
        })</Select.Trigger>
      <Select.Content preventScroll={false}>
        <Select.Group>
          {#each techniques as technique}
               <Select.Item value={`${technique.id}`}> {technique.name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}