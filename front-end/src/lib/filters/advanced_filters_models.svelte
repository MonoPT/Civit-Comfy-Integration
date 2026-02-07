<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  
  import { Checkbox } from "$lib/components/ui/checkbox/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  
  import { type FilterOption } from "$lib/filter";
  import { ListFilter, Box, Brush, PencilRuler  } from "@lucide/svelte";
  
  import {user_token} from "$lib/state.svelte"
  import API from "$lib/api"
  import {model_filters} from "./model"
  
  const {filters_state = $bindable<FilterOption>()} = $props()
 
  import { onMount } from "svelte";
  
  let base_model = $state<string[]>([])
  let types = $state<{name: string, value: string}[]>([])
  let checkpointTypes = $state<{name: string, value: string}[]>([])
  let fileFormats = $state<{name: string, value: string}[]>([])
  
  let searchModels = $state("")
  let searchTypes = $state("")
  let searchCheckpointType = $state("")
  let searchFileFormat = $state("")
  
  let selectedModels = $state([])
  let selected_types = $state([])
  let selected_file_format = $state([])
  
  let selected_filesFormat = $state("")
  let selected_base_models = $state("")
  let selected_types_ = $state("")
  let selected_checkpoint_type = $state("All")
  
  onMount(async () => {
    fetch(API.get_base_models(user_token.token)).then(async (response) => {
      if (response.status !== 200) return
      base_model = await response.json()
    })
        
    types = model_filters.types.options.map((o) => {
      return {name: o.name, value: o.value}
    })
    
    checkpointTypes = model_filters.checkpointType.options.map((o) => {
      return {name: o.name, value: o.value}
    })
    
    fileFormats = model_filters.fileFormat.options.map((o) => {
      return {name: o.name, value: o.value}
    })
  })
  
  const modifiers_state = $state<{[key: string]: boolean}>({
    earlyAccess: false,
    supportsGeneration: false,
    fromPlatform: false,
    isFeatured: false,
  })
  
  const handle_select_change_base_model = (values: string[]) => {
    filters_state.baseModel.selected = {name: "", value: values.join(",")}
    
    selected_base_models = values.join(",")
  }
  
  const handle_select_types = (values: string[]) => {
    filters_state.types.selected = {name: "", value: values.join(",")}
    
    selected_types_ = values.join(",")
  }
  
  const handle_file_format = (values: string[]) => {
    filters_state.fileFormat.selected = {name: "", value: values.join(",")}
    
    selected_filesFormat = values.join(",")
  }
  
  const handle_checkpoint_type = (values: string) => {
    filters_state.checkpointType.selected = {name: "", value: values}
    
    selected_checkpoint_type = values
  }
   
  const update_modifiers = async (modifier: string) => {
    await new Promise(resolve => setTimeout(resolve, 5));
    
    const current_val = document.querySelector(`[data-filter-modifier="${modifier}"]`)!.getAttribute("data-state") === "checked" ? true : false
            
    filters_state[modifier].selected = {name: "", value: current_val}
    modifiers_state[modifier] = current_val
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
    <div class="wrapper flex flex-col gap-2 py-2">
        {@render selectModel()}
        {@render selectTypes()}
        {@render file_format()}
        {@render checkpoint_type()}
        {@render modifiers()}
    </div>
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>

{#snippet modifiers()}
    <div class="flex flex-col gap-1.5">
        <Label class="font-semibold pt-2">Modifiers</Label>
        
        {@render create_modifier("earlyAccess", "Early Access")}
        {@render create_modifier("supportsGeneration", "On-site Generation")}
        {@render create_modifier("fromPlatform", "Made on Site")}
        {@render create_modifier("isFeatured", "Featured")}
    </div>
{/snippet}

{#snippet create_modifier(modifier: string, name: string)}
    <div class="flex items-center gap-3">
        <Checkbox checked={modifiers_state[modifier]} data-filter-modifier={modifier} onclick={() => update_modifiers(modifier)} id="filter-advanced-{modifier}" />
        <Label class="font-normal" for="filter-advanced-{modifier}">{name}</Label>
    </div>
{/snippet}

{#snippet selectModel()}
    <Select.Root bind:value={selectedModels} type="multiple" onValueChange={handle_select_change_base_model}>
      <Select.Trigger class="w-full"><Box /> Base Model ({
          selected_base_models.split(",").filter((str) => str.trim().length > 0).length > 0 ? selected_base_models.split(",").filter((str) => str.trim().length > 0).length : "all"
        })</Select.Trigger>
      <Select.Content preventScroll={false}>
          <div class="wrapper sticky block left-0 z-1 py-2" style="background: var(--popover); top: -5px;">
              <Input bind:value={searchModels} type="text" placeholder="Search..." class="max-w-xs" />
          </div>
        <Select.Group>
          {#each base_model.filter((model) => searchModels.trim().length > 0 ? model.toLowerCase().includes(searchModels.toLowerCase()) : true) as model}
               <Select.Item value={model}> {model}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}

{#snippet selectTypes()}
    <Select.Root bind:value={selected_types} type="multiple" onValueChange={handle_select_types}>
      <Select.Trigger class="w-full"><Box /> Types ({
          selected_types_.split(",").filter((str) => str.trim().length > 0).length > 0 ? selected_types_.split(",").filter((str) => str.trim().length > 0).length : "all"
        })</Select.Trigger>
      <Select.Content preventScroll={false}>
          <div class="wrapper sticky block left-0 z-1 py-2" style="background: var(--popover); top: -5px;">
              <Input bind:value={searchTypes} type="text" placeholder="Search..." class="max-w-xs" />
          </div>
        <Select.Group>
          {#each types.filter((type) => searchTypes.trim().length > 0 ? type.name.toLowerCase().includes(searchTypes.toLowerCase()) : true) as type}
               <Select.Item value={type.value}> {type.name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}

{#snippet checkpoint_type()}
    <Select.Root bind:value={selected_checkpoint_type} type="single" onValueChange={handle_checkpoint_type}>
      <Select.Trigger class="w-full"><Box /> Checkpoint Type</Select.Trigger>
      <Select.Content preventScroll={false}>
          <div class="wrapper sticky block left-0 z-1 py-2" style="background: var(--popover); top: -5px;">
              <Input bind:value={searchCheckpointType} type="text" placeholder="Search..." class="max-w-xs" />
          </div>
        <Select.Group>
          {#each checkpointTypes.filter((type) => searchCheckpointType.trim().length > 0 ? type.name.toLowerCase().includes(searchCheckpointType.toLowerCase()) : true) as type}
               <Select.Item value={type.value}> {type.name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}

{#snippet file_format()}
    <Select.Root bind:value={selected_file_format} type="multiple" onValueChange={handle_file_format}>
      <Select.Trigger class="w-full"><Box /> File Format</Select.Trigger>
      <Select.Content preventScroll={false}>
          <div class="wrapper sticky block left-0 z-1 py-2" style="background: var(--popover); top: -5px;">
              <Input bind:value={searchFileFormat} type="text" placeholder="Search..." class="max-w-xs" />
          </div>
        <Select.Group>
          {#each fileFormats.filter((format) => searchFileFormat.trim().length > 0 ? format.name.toLowerCase().includes(searchFileFormat.toLowerCase()) : true) as format}
               <Select.Item value={format.value}> {format.name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}