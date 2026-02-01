<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  
  import { ListFilter } from "@lucide/svelte";
  
  let showStatusBar = $state(true);
 
  import BaseModels from "$lib/data/base_model_descriptions"
  
  let base_model = $state({
    ...Object.fromEntries(
      Object.entries(BaseModels).map(([key, value]) => [
        key,
        { ...value, selected: false },
      ])
    ),
  } as {
    [K in keyof typeof BaseModels]: typeof BaseModels[K] & {
      selected: boolean;
    };
  })
  
  const handle_select_change = (values: string[]) => {
    console.log(values)
  }
  
</script>
 
<DropdownMenu.Root>
 <DropdownMenu.Trigger>
  {#snippet child({ props })}
   <Button {...props} variant="ghost" class="font-normal"><ListFilter/> Filters</Button>
  {/snippet}
 </DropdownMenu.Trigger>
 <DropdownMenu.Content class="w-56">
  <DropdownMenu.Group>
    <DropdownMenu.Label>Filters</DropdownMenu.Label>
    <DropdownMenu.Separator />
    <DropdownMenu.CheckboxItem closeOnSelect={false} bind:checked={showStatusBar}>Status Bar</DropdownMenu.CheckboxItem>
    {@render dropdown()}
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>


{#snippet dropdown()}
    <Select.Root type="multiple" onValueChange={handle_select_change}>
      <Select.Trigger class="w-full">Base Model (all)</Select.Trigger>
      <Select.Content>
        <Select.Group>
          {#each Object.entries(base_model) as model}
               <Select.Item value={model[1].name}>{model[1].name}</Select.Item>
          {/each}
        </Select.Group>
      </Select.Content>
    </Select.Root>
{/snippet}