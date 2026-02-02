import type { ColumnDef } from "@tanstack/table-core";
import { createRawSnippet } from "svelte";
import { renderSnippet } from "$lib/components/ui/data-table/index.js";
import { renderComponent } from "$lib/components/ui/data-table/index.js";
import DataTableActions from "./data-table-actions.svelte";
import { Checkbox } from "$lib/components/ui/checkbox/index.js";

import { type Table } from "./data"

import DataTableEmailButton from "./data-table-email-button.svelte";

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
        render: () => `<div class="text-left">thumbnail</div>`,
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
        render: () => `<div class="text-left">Model Information</div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
 
      const cellSnippet = createRawSnippet<[{ info: object }]>(
        (getInfo) => {
          const { info } = getInfo();
          
          return {
            render: () =>
              `<div>Model info</div>`,
          };
        },
      );

      return renderSnippet(cellSnippet, {
        info: row.original.info,
      });
    },
  },
  {
    accessorKey: "status",
    header: "Status",
  },
  {
    accessorKey: "email",
    header: ({ column }) =>
      renderComponent(DataTableEmailButton, {
        onclick: column.getToggleSortingHandler(),
    }),
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return renderComponent(DataTableActions, { id: row.original.id });
    },
  },
];
