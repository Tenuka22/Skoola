import {
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table'
import type {
  ColumnDef,
  OnChangeFn,
  SortingState,
  VisibilityState,
} from '@tanstack/react-table'

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Skeleton } from '@/components/ui/skeleton'
import { cn } from '@/lib/utils'
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from '@/components/ui/pagination'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'

interface DataTableProps<TData extends { id: string | number }, TValue> {
  columns: Array<ColumnDef<TData, TValue>>
  data: Array<TData>
  pageIndex: number
  pageSize: number
  pageCount: number
  canNextPage: boolean
  canPreviousPage: boolean
  fetchNextPage: () => void
  fetchPreviousPage: () => void
  sorting?: SortingState
  onSortingChange?: OnChangeFn<SortingState>
  columnVisibility?: VisibilityState
  onColumnVisibilityChange?: OnChangeFn<VisibilityState>
  rowSelection?: Record<string, boolean>
  onRowSelectionChange?: OnChangeFn<Record<string, boolean>>
  isLoading?: boolean
  onPageSizeChange?: (pageSize: number) => void
  onPageIndexChange?: (pageIndex: number) => void
  contextMenuItems?: (row: TData) => React.ReactNode
}

export function DataTable<TData extends { id: string | number }, TValue>({
  columns,
  data,
  pageIndex,
  pageSize,
  pageCount,
  canNextPage,
  canPreviousPage,
  fetchNextPage,
  fetchPreviousPage,
  sorting,
  onSortingChange,
  columnVisibility,
  onColumnVisibilityChange,
  rowSelection,
  onRowSelectionChange,
  isLoading,
  onPageSizeChange,
  onPageIndexChange,
  contextMenuItems,
}: DataTableProps<TData, TValue>) {
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    manualPagination: true,
    manualSorting: true,
    getRowId: (row: TData) => String(row.id),
    state: {
      pagination: {
        pageIndex,
        pageSize,
      },
      sorting,
      columnVisibility,
      rowSelection: rowSelection ?? {},
    },
    onSortingChange,
    onColumnVisibilityChange,
    onRowSelectionChange,
    pageCount,
  })

  return (
    <div className="w-full">
      <div className="rounded-lg border border-border/40 bg-card shadow-sm overflow-hidden">
        <Table>
          <TableHeader className="bg-muted/30">
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow
                key={headerGroup.id}
                className="hover:bg-transparent border-b-border/40"
              >
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead
                      key={header.id}
                      className="h-11 font-semibold text-foreground/80"
                    >
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                            header.column.columnDef.header,
                            header.getContext(),
                          )}
                    </TableHead>
                  )
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {isLoading ? (
              Array.from({ length: pageSize || 10 }).map((_, index) => (
                <TableRow key={index} className="border-b-border/30">
                  {columns.map((_, colIndex) => {
                    // Create some visual variation in skeleton widths
                    // but keep it deterministic based on row/col index
                    const widthClasses = [
                      'w-1/3',
                      'w-1/2',
                      'w-3/4',
                      'w-full',
                      'w-2/3',
                      'w-4/5',
                    ]
                    let w1 =
                      widthClasses[(index + colIndex) % widthClasses.length]
                    let w2 =
                      widthClasses[(index * colIndex + 1) % widthClasses.length]

                    return (
                      <TableCell key={colIndex} className="py-4 align-middle">
                        {colIndex === 0 ? (
                          <div className="flex items-center gap-3">
                            <Skeleton className="size-8 rounded-full bg-muted/50 shrink-0" />
                            <div className="flex flex-col gap-1.5 w-full">
                              <Skeleton className={`h-4 ${w1} bg-muted/50`} />
                              <Skeleton className={`h-3 ${w2} bg-muted/40`} />
                            </div>
                          </div>
                        ) : colIndex === columns.length - 1 ? (
                          <div className="flex justify-end pr-2">
                            <Skeleton className="h-8 w-8 rounded-md bg-muted/50" />
                          </div>
                        ) : (
                          <Skeleton className={`h-5 ${w1} bg-muted/50`} />
                        )}
                      </TableCell>
                    )
                  })}
                </TableRow>
              ))
            ) : table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => {
                const rowContent = (
                  <TableRow
                    key={row.id}
                    data-state={row.getIsSelected() && 'selected'}
                    className="border-b-border/30 hover:bg-muted/20 transition-colors"
                  >
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id} className="py-3">
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext(),
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                )

                if (contextMenuItems) {
                  return (
                    <ContextMenu key={row.id}>
                      <ContextMenuTrigger render={rowContent} />
                      <ContextMenuContent className="min-w-40" alignOffset={-5}>
                        {contextMenuItems(row.original)}
                      </ContextMenuContent>
                    </ContextMenu>
                  )
                }
                return rowContent
              })
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center text-muted-foreground"
                >
                  No results found.
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      <div className="flex items-center justify-between py-4 px-2">
        <div className="flex items-center space-x-6 lg:space-x-8">
          <div className="text-xs font-medium text-muted-foreground whitespace-nowrap hidden sm:block">
            Showing{' '}
            <span className="text-foreground">{pageIndex * pageSize + 1}</span>{' '}
            to{' '}
            <span className="text-foreground">
              {Math.min((pageIndex + 1) * pageSize, data.length)}
            </span>{' '}
            of <span className="text-foreground">{data.length}</span> results
          </div>
          <div className="flex items-center space-x-2">
            <p className="text-xs font-medium text-muted-foreground whitespace-nowrap hidden sm:block">
              Rows per page
            </p>
            <Select
              value={`${pageSize}`}
              onValueChange={(value) => {
                if (onPageSizeChange) {
                  onPageSizeChange(Number(value))
                }
              }}
            >
              <SelectTrigger className="h-8 w-[70px]">
                <SelectValue placeholder={pageSize} />
              </SelectTrigger>
              <SelectContent side="top">
                {[10, 20, 30, 40, 50].map((pageSizeOption) => (
                  <SelectItem key={pageSizeOption} value={`${pageSizeOption}`}>
                    {pageSizeOption}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>

        <Pagination className="mx-0 w-auto justify-end">
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                className={cn(
                  !canPreviousPage || isLoading
                    ? 'pointer-events-none opacity-50'
                    : 'cursor-pointer',
                )}
                onClick={
                  canPreviousPage && !isLoading ? fetchPreviousPage : undefined
                }
              />
            </PaginationItem>

            {/* Numbered Pagination */}
            <div className="hidden sm:flex items-center">
              {(() => {
                const currentPage = pageIndex + 1
                const maxVisiblePages = 5

                if (pageCount <= maxVisiblePages) {
                  return Array.from({ length: pageCount }).map((_, i) => (
                    <PaginationItem key={i}>
                      <PaginationLink
                        isActive={currentPage === i + 1}
                        onClick={() =>
                          onPageIndexChange
                            ? onPageIndexChange(i)
                            : table.setPageIndex(i)
                        }
                        className="cursor-pointer"
                      >
                        {i + 1}
                      </PaginationLink>
                    </PaginationItem>
                  ))
                }

                const pages = []

                // Always show first page
                pages.push(
                  <PaginationItem key={0}>
                    <PaginationLink
                      isActive={currentPage === 1}
                      onClick={() =>
                        onPageIndexChange
                          ? onPageIndexChange(0)
                          : table.setPageIndex(0)
                      }
                      className="cursor-pointer"
                    >
                      1
                    </PaginationLink>
                  </PaginationItem>,
                )

                if (currentPage > 3) {
                  pages.push(
                    <PaginationItem key="ellipsis-start">
                      <PaginationEllipsis />
                    </PaginationItem>,
                  )
                }

                // Middle pages
                const startPage = Math.max(2, currentPage - 1)
                const endPage = Math.min(pageCount - 1, currentPage + 1)

                for (let i = startPage; i <= endPage; i++) {
                  pages.push(
                    <PaginationItem key={i - 1}>
                      <PaginationLink
                        isActive={currentPage === i}
                        onClick={() =>
                          onPageIndexChange
                            ? onPageIndexChange(i - 1)
                            : table.setPageIndex(i - 1)
                        }
                        className="cursor-pointer"
                      >
                        {i}
                      </PaginationLink>
                    </PaginationItem>,
                  )
                }

                if (currentPage < pageCount - 2) {
                  pages.push(
                    <PaginationItem key="ellipsis-end">
                      <PaginationEllipsis />
                    </PaginationItem>,
                  )
                }

                // Always show last page
                pages.push(
                  <PaginationItem key={pageCount - 1}>
                    <PaginationLink
                      isActive={currentPage === pageCount}
                      onClick={() =>
                        onPageIndexChange
                          ? onPageIndexChange(pageCount - 1)
                          : table.setPageIndex(pageCount - 1)
                      }
                      className="cursor-pointer"
                    >
                      {pageCount}
                    </PaginationLink>
                  </PaginationItem>,
                )

                return pages
              })()}
            </div>
            {/* Mobile fallback indicator */}
            <PaginationItem className="sm:hidden">
              <span className="text-sm px-4">
                Page {pageIndex + 1} of {pageCount}
              </span>
            </PaginationItem>

            <PaginationItem>
              <PaginationNext
                className={cn(
                  !canNextPage || isLoading
                    ? 'pointer-events-none opacity-50'
                    : 'cursor-pointer',
                )}
                onClick={canNextPage && !isLoading ? fetchNextPage : undefined}
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      </div>
    </div>
  )
}
