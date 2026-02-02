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
    accessorKey: "amount",
    header: () => {
      const amountHeaderSnippet = createRawSnippet(() => ({
        render: () => `<div class="text-end">Amount</div>`,
      }));
      return renderSnippet(amountHeaderSnippet);
    },
    cell: ({ row }) => {
      const formatter = new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
      });

      const amountCellSnippet = createRawSnippet<[{ amount: number }]>(
        (getAmount) => {
          const { amount } = getAmount();
          const formatted = formatter.format(amount);
          return {
            render: () =>
              `<div class="text-end font-medium">${formatted}</div>`,
          };
        },
      );

      return renderSnippet(amountCellSnippet, {
        amount: row.original.amount,
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
