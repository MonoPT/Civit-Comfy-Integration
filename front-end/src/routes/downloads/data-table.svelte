<script lang="ts" generics="TData, TValue">
    import {
        createSvelteTable,
        FlexRender,
    } from "$lib/components/ui/data-table/index.js";
    
    //@ts-ignore
    import { toast } from "svelte-sonner";
    
    import * as Table from "$lib/components/ui/table/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Spinner } from "$lib/components/ui/spinner/index.js";
    
    import { Plus } from "@lucide/svelte";
    
    import {get_model_data} from "$lib/apis/model_data"
    
    import {
        type ColumnDef,
        type SortingState,
        type ColumnFiltersState,
        type RowSelectionState,
        getCoreRowModel,
        getPaginationRowModel,
        getSortedRowModel,
        getFilteredRowModel,
    } from "@tanstack/table-core";

    type DataTableProps<TData, TValue> = {
        columns: ColumnDef<TData, TValue>[];
        data: TData[];
    };
    
    let { data, columns }: DataTableProps<TData, TValue> = $props();
    
    let sorting = $state<SortingState>([]);
    let columnFilters = $state<ColumnFiltersState>([]);
    let rowSelection = $state<RowSelectionState>({});
      
    const table = createSvelteTable({
        get data() {
            return data;
        },
        columns,
        getCoreRowModel: getCoreRowModel(),
        getPaginationRowModel: getPaginationRowModel(),
        getSortedRowModel: getSortedRowModel(),
        getFilteredRowModel: getFilteredRowModel(),
        onSortingChange: (updater) => {
            if (typeof updater === "function") {
                sorting = updater(sorting);
            } else {
                sorting = updater;
            }
        },
        onColumnFiltersChange: (updater) => {
            if (typeof updater === "function") {
                columnFilters = updater(columnFilters);
            } else {
                columnFilters = updater;
            }
        },
        onRowSelectionChange: (updater) => {
          if (typeof updater === "function") {
            rowSelection = updater(rowSelection);
          } else {
            rowSelection = updater;
          }
        },
        state: {
            get sorting() {
                return sorting;
            },
            get columnFilters() {
                return columnFilters;
            },
            get rowSelection() {
              return rowSelection;
            },
        },
    });
    
    let add_download_dialog = $state(false)
    let is_fetching_model_data = $state(false)
    
    async function handleModelDownload(e: Event) {
      is_fetching_model_data = true
      const model_param = document.querySelector<HTMLInputElement>("#modelParam")!.value.trim()
      
      let resp = await get_model_data(model_param)
            
      is_fetching_model_data = false
      
      if (resp.status !== 200) {
        toast.error("The provided url/id is not valid")
        return
      }
      
      add_download_dialog = false
      
      window.dispatchEvent(new CustomEvent("DownloadManagerShowModelVersions", {
        detail: {
          ModelData: resp.data
        }
      }))
    }
</script>

<div>
    <div class="flex items-center py-4">
        <Input
            placeholder="Filter emails..."
            value={(table.getColumn("version")?.getFilterValue() as string) ?? ""}
            onchange={(e) => {
                table.getColumn("version")?.setFilterValue(e.currentTarget.value);
            }}
            oninput={(e) => {
                table.getColumn("version")?.setFilterValue(e.currentTarget.value);
                }}
            class="max-w-sm"
        />
        
        <div class="actions flex items-center ml-auto">
            <Button onclick={() => {add_download_dialog = true}} variant="outline"><Plus /> Download model</Button>
            {@render add_model()}
        </div>
        
    </div>
    <div class="rounded-md border">
        <Table.Root>
            <Table.Header>
                {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
                    <Table.Row>
                        {#each headerGroup.headers as header (header.id)}
                            <Table.Head colspan={header.colSpan}>
                                {#if !header.isPlaceholder}
                                    <FlexRender
                                        content={header.column.columnDef.header}
                                        context={header.getContext()}
                                    />
                                {/if}
                            </Table.Head>
                        {/each}
                    </Table.Row>
                {/each}
            </Table.Header>
            <Table.Body>
                {#each table.getRowModel().rows as row (row.id)}
                    <Table.Row data-state={row.getIsSelected() && "selected"}>
                        {#each row.getVisibleCells() as cell (cell.id)}
                            <Table.Cell>
                                <FlexRender
                                    content={cell.column.columnDef.cell}
                                    context={cell.getContext()}
                                />
                            </Table.Cell>
                        {/each}
                    </Table.Row>
                {:else}
                    <Table.Row>
                        <Table.Cell
                            colspan={columns.length}
                            class="h-24 text-center"
                        >
                            Click on the add button to start downloading models.
                        </Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>
</div>

<div class="text-muted-foreground flex-1 text-sm">
  {table.getFilteredSelectedRowModel().rows.length} of{" "}
  {table.getFilteredRowModel().rows.length} row(s) selected.
</div>

<style>
    :global(th:has(.thumbnailHeader)) {
        width: 190px;
    }
</style>

{#snippet add_model()}
    <Dialog.Root open={add_download_dialog}>
      <form>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>Download a new model</Dialog.Title>
          </Dialog.Header>
          <div class="grid gap-4">
              <Input min="1" id="modelParam" type="text" required placeholder="Paste the model id or page url" class="w-full" />
          </div>
          <Dialog.Footer>
            <Dialog.Close class={buttonVariants({ variant: "outline" })}
              >Cancel</Dialog.Close>
            <Button type="submit" disabled={is_fetching_model_data} onclick={handleModelDownload}>
                {#if is_fetching_model_data}
                    <Spinner />
                {/if}
                Download
            </Button>
          </Dialog.Footer>
        </Dialog.Content>
      </form>
    </Dialog.Root>
{/snippet}