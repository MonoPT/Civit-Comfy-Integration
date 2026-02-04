<script lang="ts">
    import * as Table from "$lib/components/ui/table/index.js";
    import * as Accordion from "$lib/components/ui/accordion/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
        
    import DOMPurify from "dompurify";
    
    //@ts-ignore
    import byteSize from "byte-size";
    
    import {
        ChevronDown,
        ArrowDownToLine,
        ThumbsDown,
        ThumbsUp,
    } from "@lucide/svelte";

    import { type ModelData } from "$lib/apis/model_data";
    import { Checkbox } from "$lib/components/ui/checkbox/index.js";

    const { data, callback, needle }: { data: ModelData, callback: any, needle: string } = $props();
    
    const options: Intl.DateTimeFormatOptions = {
        year: "numeric",
        month: "long",
        day: "numeric",
    };
    
    
</script>

<Table.Root id="ModelVersionsTable">
    <Table.Header>
        <Table.Row >
            <Table.Head><Checkbox data-version-id="headerCheckbox" onclick={callback} /></Table.Head>
            <Table.Head class="w-[100px]">Name</Table.Head>
            <Table.Head>Base Model</Table.Head>
            <Table.Head class="text-center">Files</Table.Head>
            <Table.Head>Stats</Table.Head>
            <Table.Head class="text-center">Type</Table.Head>
            <Table.Head class="text-center">Total Size</Table.Head>
            <Table.Head class="text-end">Published at</Table.Head>
        </Table.Row>
    </Table.Header>
    <Table.Body>
        {#each data.modelVersions as ModelV}
            <Table.Row style={ !ModelV.name.toLowerCase().includes(needle.toLowerCase()) ? "position: absolute; scale: 0" : ""}>
                <Table.Cell><Checkbox data-version-id={ModelV.id} onclick={callback} /></Table.Cell
                >
                <Table.Cell>{ModelV.name}</Table.Cell>
                <Table.Cell>{ModelV.baseModel}</Table.Cell>
                <Table.Cell class="text-center">
                    {@render filesView(ModelV.files)}
                </Table.Cell>
                <Table.Cell>
                    <div class="flex items-center gap-4 text-muted-foreground">
                        <span class="flex items-center gap-1.5">
                            <ArrowDownToLine size={16} strokeWidth={2} />
                            {ModelV.stats.downloadCount}
                        </span>
                        <span class="flex items-center gap-1.5">
                            <ThumbsUp size={16} strokeWidth={2} />
                            {ModelV.stats.thumbsUpCount}
                        </span>
                        <span class="flex items-center gap-1.5">
                            <ThumbsDown size={16} strokeWidth={2} />
                            {ModelV.stats.thumbsDownCount}
                        </span>
                    </div>
                </Table.Cell>
                <Table.Cell class="text-center">{data.type}</Table.Cell>
                <Table.Cell class="text-center">{byteSize(Math.round(ModelV.files.reduce((sum: number, file: any) => sum + file.sizeKB, 0)) * 1024).toString()}</Table.Cell>
                <Table.Cell class="text-end"
                    >{new Date(ModelV.publishedAt).toLocaleDateString(
                        "en-UK",
                        options,
                    )}</Table.Cell
                >
                <Table.Cell class="flex">
                    <div class="ml-auto relative block">
                        <Button
                            variant="ghost"
                            size="icon"
                            aria-label="Submit"
                            onclick={(e) => {
                                (e.target as HTMLElement)!
                                    .closest<HTMLElement>("tr")!
                                    .nextElementSibling!.querySelector<HTMLElement>(
                                        ".trigger",
                                    )!
                                    .click();
                            }}
                        >
                            <ChevronDown />
                        </Button>
                    </div>
                </Table.Cell>
            </Table.Row>
            <Table.Row class="modelTableRow">
                <Table.Cell class="modelTableCell" colspan={8}>
                    <Accordion.Root type="single">
                        <Accordion.Item value="item-1" data-state="open">
                            <div
                                class="absolute opacity-0 scale-0 pointer-none"
                            >
                                <Accordion.Trigger class="trigger"
                                ></Accordion.Trigger>
                            </div>
                            <Accordion.Content class="py-2 text-wrap">
                                {@html DOMPurify.sanitize(ModelV.description)}
                            </Accordion.Content>
                        </Accordion.Item>
                    </Accordion.Root>
                </Table.Cell>
            </Table.Row>
        {/each}
    </Table.Body>
</Table.Root>

{#snippet filesView(files: any[])}
    <Popover.Root>
        <Popover.Trigger class="hover:underline">{files.length}</Popover.Trigger
        >
        <Popover.Content class="text-wrap " style="max-width: 50vw; width: max-content;">
            <Table.Root>
                <Table.Header>
                    <Table.Row>
                        <Table.Head>Name</Table.Head>
                        <Table.Head>Format</Table.Head>
                        <Table.Head>FP</Table.Head>
                        <Table.Head>Size</Table.Head>
                        <Table.Head>Size</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each files as file}
                        <Table.Row>
                            <Table.Cell>{file.name}</Table.Cell>
                            <Table.Cell>{file.metadata.format}</Table.Cell>
                            <Table.Cell>{file.metadata.fp}</Table.Cell>
                            <Table.Cell>{file.metadata.size}</Table.Cell>
                            <Table.Cell>{byteSize(Math.round(file.sizeKB) * 1024).toString()}</Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </Popover.Content>
    </Popover.Root>
{/snippet}

<style>
    :global(.modelTableRow:not(:has([data-state="open"]))) {
        border: none;
    }

    :global(.modelTableCell:not(:has([data-state="open"]))) {
        padding-block: 0;
    }
    
     
    :global(#ModelVersionsTable [data-slot="checkbox-indicator"]) {
        pointer-events: none;
    }
    
</style>

