<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { CalendarArrowUp, ArrowDownWideNarrow  } from "@lucide/svelte";
    
    import { type FilterOption } from "$lib/filter";
    
    const {filters_state = $bindable<FilterOption>()} = $props()
</script>

<DropdownMenu.Root  >
 <DropdownMenu.Trigger><Button variant="ghost" class="font-normal"><CalendarArrowUp /> {filters_state.period.selected.name}</Button></DropdownMenu.Trigger>
 <DropdownMenu.Content>
  <DropdownMenu.Group>
    {#each filters_state.period.options as period}
        <DropdownMenu.Item onclick={() => {filters_state.period.selected = period}}>{period.name}</DropdownMenu.Item>
    {/each}
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>


<DropdownMenu.Root  >
 <DropdownMenu.Trigger><Button variant="ghost" class="font-normal"><ArrowDownWideNarrow  /> {filters_state.sort.selected.name}</Button></DropdownMenu.Trigger>
 <DropdownMenu.Content>
  <DropdownMenu.Group>
    {#each filters_state.sort.options as period}
        <DropdownMenu.Item onclick={() => {filters_state.sort.selected = period}}>{period.name}</DropdownMenu.Item>
    {/each}
  </DropdownMenu.Group>
 </DropdownMenu.Content>
</DropdownMenu.Root>