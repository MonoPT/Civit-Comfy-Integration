import type { ColumnDef } from "@tanstack/table-core";
import { createRawSnippet } from "svelte";
import { renderSnippet } from "$lib/components/ui/data-table/index.js";
import { renderComponent } from "$lib/components/ui/data-table/index.js";
import DataTableActions from "./data-table-actions.svelte";
import { Checkbox } from "$lib/components/ui/checkbox/index.js";

import { type Table, type TableModelInfo } from "./data"
import { Progress } from "$lib/components/ui/progress/index.js";

export const columns: ColumnDef<Table>[] = [
  {
    id: "select",
    header: ({ table }) =>
      renderComponent(Checkbox, {
        checked: table.getIsAllPageRowsSelected(),
        indeterminate:
          table.getIsSomePageRowsSelected() &&
          !table.getIsAllPageRowsSelected(),
        onCheckedChange: (value) => table.toggleAllPageRowsSelected(!!value),
        "aria-label": "Select all",
      }),
    cell: ({ row }) =>
      renderComponent(Checkbox, {
        checked: row.getIsSelected(),
        onCheckedChange: (value) => row.toggleSelected(!!value),
        "aria-label": "Select row",
      }),
    enableSorting: false,
    enableHiding: false,
  },
  {
    accessorKey: "thumbnail",
    header: () => {
      const amountHeaderSnippet = createRawSnippet(() => ({
        render: () => `<div class="thumbnailHeader"></div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
      const cellSnippet = createRawSnippet<[{ thumbnail: string }]>(
        (getThumbnail) => {
          const { thumbnail } = getThumbnail();
          
          return {
            render: () =>
              `<img class="block relative max-w-40 rounded-sm" src="${thumbnail}" />`,
          };
        },
      );

      return renderSnippet(cellSnippet, {
        thumbnail: row.original.thumbnail,
      });
    },
  },
  {
    accessorKey: "info",
    header: () => {
      const amountHeaderSnippet = createRawSnippet(() => ({
        render: () => `<div class="text-left w-full"> <div class="spacer block relative" style="min-width: 0vw"></div></div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
 
      const cellSnippet = createRawSnippet<[{ info: TableModelInfo }]>(
        (getInfo) => {
          const { info } = getInfo();
          
          return {
            render: () =>
              /*html*/`<div class="w-full h-full">
                <h2 class="scroll-m-20 text-2xl font-semibold tracking-tight">${info.name}</h2>
                <div class="model-details flex align-middle pt-2 gap-4">
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-user-icon lucide-user"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                        ${info.author}
                    </span>
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-layers-icon lucide-layers"><path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12"/><path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17"/></svg>
                        ${info.base_model}
                    </span>
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-calendar-icon lucide-calendar"><path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/></svg>
                        ${info.release_Date}
                    </span>
                </div>
                <div class="model-details flex align-middle pt-4 gap-4">
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-down-to-line-icon lucide-arrow-down-to-line"><path d="M12 17V3"/><path d="m6 11 6 6 6-6"/><path d="M19 21H5"/></svg>
                        ${info.metrics.downloads}
                    </span>
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-thumbs-up-icon lucide-thumbs-up"><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"/><path d="M7 10v12"/></svg>
                        ${info.metrics.like}
                    </span>
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-library-icon lucide-library"><path d="m16 6 4 14"/><path d="M12 6v14"/><path d="M8 8v12"/><path d="M4 4v16"/></svg>
                        ${info.metrics.collected}
                    </span>
                    <span class="flex items-center gap-2 text-muted-foreground">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-zap-icon lucide-zap"><path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"/></svg>
                        ${info.metrics.donations}
                    </span>
                </div>
                <div class="model-details flex align-middle pt-8 gap-2 w-full flex-col text-muted-foreground" style="max-width: 400px">
                    <div class="flex items-center">
                        <span>Downloading</span>
                        <span class="block ml-auto" style="font-size: .85em">${info.download.download_speed} Mb/s (${info.download.current} Mb / ${info.download.total} Mb)</span>
                    </div>
                    <div class="block w-full">
                        <div data-slot="progress" class="bg-primary/20 relative h-2 overflow-hidden rounded-full w-full" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="66" data-value="66" data-state="loading" data-max="100" data-min="0" data-progress-root=""><div data-slot="progress-indicator" class="bg-primary h-full w-full flex-1 transition-all" 
                        style="transform: translateX(-${(100 - (info.download.current * 100) / info.download.total)}%);"></div></div>
                    </div>
                </div>
              </div>`,
          };
        },
      );

      return renderSnippet(cellSnippet, {
        info: row.original.info,
      });
    },
  },
  {
    accessorKey: "version",
    header: () => {
      const amountHeaderSnippet = createRawSnippet(() => ({
        render: () => `<div class="text-center"></div></div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
 
      const cellSnippet = createRawSnippet<[{ version: string }]>(
        (getVersion) => {
          const { version } = getVersion();
          
          return {
            render: () =>
              `<div class="text-center">${version}</div>`,
          };
        },
      );

      return renderSnippet(cellSnippet, {
        version: row.original.version,
      });
    },
  },
  {
    accessorKey: "size",
    header: () => {
      const amountHeaderSnippet = createRawSnippet(() => ({
        render: () => `<div class="text-center"></div></div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
 
      const cellSnippet = createRawSnippet<[{ size: string }]>(
        (getSize) => {
          const { size } = getSize();
          
          return {
            render: () =>
              `<div class="text-center">${size}</div>`,
          };
        },
      );

      return renderSnippet(cellSnippet, {
        size: row.original.size,
      });
    },
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return renderComponent(DataTableActions, { id: row.original.id });
    },
  },
];
