import * as React from 'react'
import {
  
  
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable
} from '@tanstack/react-table'
import type {ColumnDef, Row, SortingState } from '@tanstack/react-table';
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { Skeleton } from '@/components/ui/skeleton'
import { Stack, Text } from '@/components/primitives'
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination'

interface StaffTableProps<TData> {
  columns: Array<ColumnDef<TData>>
  data: Array<TData>
  isLoading: boolean
  rowSelection: Record<string, boolean>
  setRowSelection: React.Dispatch<React.SetStateAction<Record<string, boolean>>>
  limit: number
  contextMenuItems?: (row: TData) => React.ReactNode
  bulkActions?: (props: { selectedRows: Array<Row<TData>> }) => React.ReactNode
}

export function StaffTable<TData>({
  columns,
  data,
  isLoading,
  rowSelection,
  setRowSelection,
  limit,
  contextMenuItems,
  bulkActions,
}: StaffTableProps<TData>) {
  const [sorting, setSorting] = React.useState<SortingState>([])

  const table = useReactTable({
    data,
    columns,
    state: {
      sorting,
      rowSelection,
    },
    onSortingChange: setSorting,
    onRowSelectionChange: setRowSelection,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    manualPagination: true,
    pageCount: Math.ceil(data.length / limit),
  })

  const selectedRows = table.getFilteredSelectedRowModel().rows

  return (
    <Stack gap={4} className="flex-1 min-h-0">
      {/* Bulk Actions Bar */}
      {selectedRows.length > 0 && bulkActions && (
        <div className="sticky top-0 z-10">{bulkActions({ selectedRows })}</div>
      )}

      {/* Table */}
      <ScrollArea className="flex-1">
        <Table>
          <TableHeader className="sticky top-0 bg-muted/50 z-10">
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead key={header.id}>
                    {flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {isLoading ? (
              Array.from({ length: limit }).map((_, i) => (
                <TableRow key={i}>
                  {columns.map((_, j) => (
                    <TableCell key={j}>
                      <Skeleton className="h-4 w-full" />
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : table.getRowModel().rows.length === 0 ? (
              <TableRow>
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  <Text muted>No results.</Text>
                </TableCell>
              </TableRow>
            ) : (
              table.getRowModel().rows.map((row) => (
                <ContextMenu key={row.id}>
                  <ContextMenuTrigger>
                    <TableRow
                      data-state={row.getIsSelected() && 'selected'}
                      className="cursor-context-menu"
                    >
                      {row.getVisibleCells().map((cell) => (
                        <TableCell key={cell.id}>
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext(),
                          )}
                        </TableCell>
                      ))}
                    </TableRow>
                  </ContextMenuTrigger>
                  {contextMenuItems && (
                    <ContextMenuContent>
                      {contextMenuItems(row.original)}
                    </ContextMenuContent>
                  )}
                </ContextMenu>
              ))
            )}
          </TableBody>
        </Table>
      </ScrollArea>

      {/* Pagination */}
      <Pagination>
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious
              href="#"
              onClick={(e) => {
                e.preventDefault()
                table.previousPage()
              }}
              className={
                !table.getCanPreviousPage() ? 'pointer-events-none opacity-50' : ''
              }
            />
          </PaginationItem>
          {table.getPageCount() > 0 &&
            Array.from(
              { length: Math.min(5, table.getPageCount()) },
              (_, pageIndex) => {
                const startPage = Math.max(0, pageIndex - 2)
                const endPage = Math.min(table.getPageCount() - 1, startPage + 4)
                const adjustedStart = Math.max(0, endPage - 4)

                return Array.from(
                  { length: endPage - adjustedStart + 1 },
                  (_, j) => adjustedStart + j,
                ).map((pageIndex) => (
                  <PaginationItem key={pageIndex}>
                    <PaginationLink
                      href="#"
                      onClick={(e) => {
                        e.preventDefault()
                        table.setPageIndex(pageIndex)
                      }}
                      isActive={
                        pageIndex === table.getState().pagination.pageIndex
                      }
                    >
                      {pageIndex + 1}
                    </PaginationLink>
                  </PaginationItem>
                ))
              },
            )}
          <PaginationItem>
            <PaginationNext
              href="#"
              onClick={(e) => {
                e.preventDefault()
                table.nextPage()
              }}
              className={
                !table.getCanNextPage() ? 'pointer-events-none opacity-50' : ''
              }
            />
          </PaginationItem>
        </PaginationContent>
      </Pagination>
    </Stack>
  )
}
