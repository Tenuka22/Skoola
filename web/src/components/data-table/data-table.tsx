'use client'

import * as React from 'react'
import { useQueryStates } from 'nuqs'
import {
  flexRender,
  getCoreRowModel,
  getExpandedRowModel,
  getFacetedRowModel,
  getFacetedUniqueValues,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table'
import { parse as parseCsv } from 'csv-parse/browser/esm/sync'
import { stringify as serializeCsv } from 'csv-stringify/browser/esm/sync'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { ArrowDown01Icon, ArrowRight01Icon } from '@hugeicons/core-free-icons'
import { dataTableParsers } from './store'
import { DataTablePagination } from './data-table-pagination'
import { DataTableToolbar } from './data-table-toolbar'
import {
  buildColumnsFromAutoConfig,
  downloadFile,
  exportToCsv,
  exportToPdf,
  getColId,
  getColumnMeta,
  isObject,
} from './utils'
import type {
  AccessorFnColumnDef,
  ColumnFiltersState,
  ColumnPinningState,
  ExpandedState,
  OnChangeFn,
  SortingState,
  VisibilityState,
} from '@tanstack/react-table'

import type {
  DataTableColumnDef,
  DataTableDensity,
  DataTableProps,
  DataTableToolbarContext,
} from './types'
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuTrigger,
} from '@/components/ui/context-menu'
import { Checkbox } from '@/components/ui/checkbox'
import { Skeleton } from '@/components/ui/skeleton'
import {
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Table as UiTable,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { Box, Stack } from '@/components/primitives'

export function DataTable<TRow extends { id?: string | number }>({
  config,
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
  columnFilters,
  onColumnFiltersChange,
  columnVisibility,
  onColumnVisibilityChange,
  rowSelection,
  onRowSelectionChange,
  isLoading,
  onPageSizeChange,
  onPageIndexChange,
  contextMenuItems,
  rowId,
  rowActions,
  highlightRow,
  emptyState,
  pageSizeOptions = [10, 20, 50, 75, 100],
  toolbar,
  showDefaultToolbar = true,
  allowColumnVisibilityToggle = true,
  allowPaste = true,
  enableSelection = false,
  enablePinning = false,
  enableExpansion = false,
  renderSubComponent,
  bulkActions,
  facetedFilters,
  onImportCSV,
  onImportJSON,
  onExportCSV,
  onExportPDF,
  onFetchFullData,
  onAdd,
  onAddLabel,
  templateFilename = 'datatable-template.csv',
  search,
  onSearchChange,
  searchPlaceholder,
  extraActions,
}: DataTableProps<TRow>) {
  const [urlState, setUrlState] = useQueryStates(dataTableParsers)

  const [density, setDensity] = React.useState<DataTableDensity>(
    urlState.density,
  )

  React.useEffect(() => {
    setUrlState({ density })
  }, [density, setUrlState])

  const [expanded, setExpanded] = React.useState<ExpandedState>({})
  const [columnPinning, setColumnPinning] = React.useState<ColumnPinningState>({
    left: [],
    right: [],
  })

  const autoColumns = React.useMemo((): Array<
    DataTableColumnDef<TRow, unknown>
  > => {
    if (config !== undefined) {
      return buildColumnsFromAutoConfig(config)
    }
    return []
  }, [config])

  const baseColumns = React.useMemo((): Array<
    DataTableColumnDef<TRow, unknown>
  > => {
    if (columns && columns.length) {
      return columns
    }
    return autoColumns
  }, [columns, autoColumns])

  const builtColumns = React.useMemo((): Array<
    DataTableColumnDef<TRow, unknown>
  > => {
    const extras: Array<DataTableColumnDef<TRow, unknown>> = baseColumns
      ? [...baseColumns]
      : []

    if (enableExpansion) {
      const expandCol: DataTableColumnDef<TRow, unknown> = {
        id: 'expand',
        header: () => null,
        cell: ({ row }) => (
          <Button
            variant="ghost"
            size="sm"
            className="h-8 w-8 p-0"
            onClick={() => row.toggleExpanded()}
          >
            <HugeiconsIcon
              icon={row.getIsExpanded() ? ArrowDown01Icon : ArrowRight01Icon}
              className="h-4 w-4"
            />
          </Button>
        ),
        enableSorting: false,
        enableHiding: false,
      }
      extras.unshift(expandCol)
    }

    if (enableSelection) {
      const selectionCol: DataTableColumnDef<TRow, unknown> = {
        id: 'select',
        header: ({ table }) => (
          <Checkbox
            checked={table.getIsAllPageRowsSelected()}
            onCheckedChange={(value) =>
              table.toggleAllPageRowsSelected(!!value)
            }
            aria-label="Select all"
          />
        ),
        cell: ({ row }) => (
          <Checkbox
            checked={row.getIsSelected()}
            onCheckedChange={(value) => row.toggleSelected(!!value)}
            aria-label="Select row"
          />
        ),
        enableSorting: false,
        enableHiding: false,
      }
      extras.unshift(selectionCol)
    }

    if (rowActions) {
      const actionsCol: DataTableColumnDef<TRow, unknown> = {
        id: 'row-actions',
        header: '',
        cell: ({ row }) => (
          <div className="flex justify-end space-x-2">
            {rowActions(row.original)}
          </div>
        ),
        meta: { align: 'end' },
      }
      extras.push(actionsCol)
    }
    return extras
  }, [baseColumns, rowActions, enableSelection, enableExpansion])

  React.useEffect(() => {
    if (enablePinning) {
      const left: Array<string> = []
      const right: Array<string> = []
      builtColumns.forEach((col) => {
        const id = getColId(col)
        if (id) {
          if (
            col.meta?.isPinned === 'left' ||
            id === 'select' ||
            id === 'expand'
          ) {
            left.push(id)
          } else if (col.meta?.isPinned === 'right' || id === 'row-actions') {
            right.push(id)
          }
        }
      })
      setColumnPinning({ left, right })
    }
  }, [builtColumns, enablePinning])

  const manualSorting = true
  const manualFiltering = onColumnFiltersChange !== undefined

  const [internalSorting, setInternalSorting] = React.useState<SortingState>(
    sorting ?? urlState.sort ?? [],
  )

  React.useEffect(() => {
    if (sorting) {
      setInternalSorting(sorting)
    }
  }, [sorting])

  const sortingState = React.useMemo(
    () => (manualSorting ? (sorting ?? []) : internalSorting),
    [manualSorting, sorting, internalSorting],
  )

  const handleSortingChange: OnChangeFn<SortingState> = (updaterOrValue) => {
    const nextSorting =
      typeof updaterOrValue === 'function'
        ? updaterOrValue(sortingState)
        : updaterOrValue

    if (!manualSorting) {
      setInternalSorting(nextSorting)
    }
    setUrlState({ sort: nextSorting })
    onSortingChange?.(nextSorting)
  }

  const [internalColumnFilters, setInternalColumnFilters] =
    React.useState<ColumnFiltersState>(columnFilters ?? [])

  React.useEffect(() => {
    if (columnFilters) {
      setInternalColumnFilters(columnFilters)
    }
  }, [columnFilters])

  const resolvedColumnFilters = columnFilters ?? internalColumnFilters

  const handleColumnFiltersChange: OnChangeFn<ColumnFiltersState> = (
    updaterOrValue,
  ) => {
    const nextFilters =
      typeof updaterOrValue === 'function'
        ? updaterOrValue(resolvedColumnFilters)
        : updaterOrValue

    if (!manualFiltering) {
      setInternalColumnFilters(nextFilters)
    }
    onColumnFiltersChange?.(nextFilters)
  }

  const [internalColumnVisibility, setInternalColumnVisibility] =
    React.useState<VisibilityState>(() => columnVisibility ?? {})

  React.useEffect(() => {
    if (columnVisibility) {
      setInternalColumnVisibility(columnVisibility)
    }
  }, [columnVisibility])

  const resolvedColumnVisibility = columnVisibility ?? internalColumnVisibility

  const handleColumnVisibilityChange: OnChangeFn<VisibilityState> = (
    updaterOrValue,
  ) => {
    const nextVisibility =
      typeof updaterOrValue === 'function'
        ? updaterOrValue(resolvedColumnVisibility)
        : updaterOrValue

    if (!columnVisibility) {
      setInternalColumnVisibility(nextVisibility)
    }
    onColumnVisibilityChange?.(nextVisibility)
  }

  const [internalRowSelection, setInternalRowSelection] = React.useState<
    Record<string, boolean>
  >(() => rowSelection ?? {})

  React.useEffect(() => {
    if (rowSelection) {
      setInternalRowSelection(rowSelection)
    }
  }, [rowSelection])

  const resolvedRowSelection = rowSelection ?? internalRowSelection

  const handleRowSelectionChange: OnChangeFn<Record<string, boolean>> = (
    updaterOrValue,
  ) => {
    const nextSelection =
      typeof updaterOrValue === 'function'
        ? updaterOrValue(resolvedRowSelection)
        : updaterOrValue

    if (!rowSelection) {
      setInternalRowSelection(nextSelection)
    }
    onRowSelectionChange?.(nextSelection)
  }

  const table = useReactTable({
    data,
    columns: builtColumns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    getFacetedRowModel: getFacetedRowModel(),
    getFacetedUniqueValues: getFacetedUniqueValues(),
    manualPagination: true,
    manualSorting,
    manualFiltering,
    enableMultiSort: true,
    getRowId: (row) => {
      const explicitId = rowId?.(row)
      if (explicitId !== undefined) return String(explicitId)
      if (
        isObject(row) &&
        'id' in row &&
        row.id !== undefined &&
        row.id !== null
      ) {
        return String(row.id)
      }
      return JSON.stringify(row)
    },
    state: {
      pagination: { pageIndex, pageSize },
      sorting: sortingState,
      columnFilters: resolvedColumnFilters,
      columnVisibility: resolvedColumnVisibility,
      rowSelection: resolvedRowSelection,
      expanded,
      columnPinning,
    },
    onSortingChange: handleSortingChange,
    onColumnFiltersChange: handleColumnFiltersChange,
    onColumnVisibilityChange: handleColumnVisibilityChange,
    onRowSelectionChange: handleRowSelectionChange,
    onExpandedChange: setExpanded,
    onColumnPinningChange: setColumnPinning,
    pageCount,
  })

  const csvInputRef = React.useRef<HTMLInputElement | null>(null)
  const jsonInputRef = React.useRef<HTMLInputElement | null>(null)

  const handleCsvFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0]
    event.target.value = ''
    if (!file) return
    if (!onImportCSV) {
      toast('CSV import is not configured for this table.')
      return
    }
    try {
      const text = await file.text()
      const parsed = parseCsv(text, {
        columns: true,
        skip_empty_lines: true,
        cast: true,
      })
      if (Array.isArray(parsed) && parsed.every(isObject)) {
        onImportCSV(parsed)
        toast.success(`Imported ${file.name}`)
      }
    } catch (err) {
      console.error(err)
      toast.error(
        err instanceof Error ? err.message : 'Failed to parse CSV file.',
      )
    }
  }

  const handleJsonFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0]
    event.target.value = ''
    if (!file) return
    if (!onImportJSON) {
      toast('JSON import is not configured for this table.')
      return
    }
    try {
      const text = await file.text()
      const parsed: unknown = JSON.parse(text)
      if (!Array.isArray(parsed) || !parsed.every(isObject)) {
        throw new Error('JSON must be an array of records.')
      }
      onImportJSON(parsed)
      toast.success(`Imported ${file.name}`)
    } catch (err) {
      console.error(err)
      toast.error(
        err instanceof Error ? err.message : 'Failed to parse JSON file.',
      )
    }
  }

  const handleGenerateTemplateCsv = React.useCallback(() => {
    const templateColumns = baseColumns.filter(
      (
        column,
      ): column is AccessorFnColumnDef<TRow, unknown> & { id: string } => {
        const id = getColId(column)
        return (
          id !== undefined &&
          id !== 'row-actions' &&
          'accessorFn' in column &&
          typeof column.accessorFn === 'function'
        )
      },
    )
    if (!templateColumns.length) {
      toast('Add at least one column before generating a template.')
      return
    }

    const exampleRow = data[0]
    const headers = templateColumns.map((column) => getColId(column) || '')
    const row = templateColumns.reduce<Record<string, unknown>>(
      (acc, column) => {
        const key = getColId(column) || ''
        let value: unknown = ''
        if (exampleRow && column.accessorFn) {
          value = column.accessorFn(exampleRow, 0)
        }
        if (value === undefined || value === null) {
          value = ''
        }
        acc[key] = typeof value === 'object' ? JSON.stringify(value) : value
        return acc
      },
      {},
    )

    const csv = serializeCsv([row], { header: true, columns: headers })
    downloadFile(csv, templateFilename, 'text/csv;charset=utf-8')
  }, [baseColumns, data, templateFilename])

  const getDataForExport = React.useCallback(
    async (options?: {
      allData?: boolean
      selectedOnly?: boolean
    }): Promise<Array<TRow>> => {
      if (options?.allData && onFetchFullData) {
        try {
          const fullData = await onFetchFullData()
          return fullData
        } catch (err) {
          console.error(err)
          toast.error('Failed to fetch full data')
          return []
        }
      }
      if (options?.selectedOnly) {
        const selected = table
          .getSelectedRowModel()
          .rows.map((row) => row.original)
        if (selected.length === 0) {
          toast.info('No rows selected for export')
        }
        return selected
      }
      return data
    },
    [data, onFetchFullData, table],
  )

  const handleExportCsv = React.useCallback(
    async (options?: { allData?: boolean; selectedOnly?: boolean }) => {
      try {
        const exportData = await getDataForExport(options)
        if (!exportData || exportData.length === 0) {
          toast.error('No data found to export')
          return
        }

        if (onExportCSV) {
          onExportCSV(exportData)
        }
        exportToCsv(exportData, builtColumns, 'export.csv')
      } catch (err: unknown) {
        console.error('Export Error:', err)
        toast.error(
          'Failed to export: ' +
            (err instanceof Error ? err.message : 'Unknown error'),
        )
      }
    },
    [builtColumns, onExportCSV, getDataForExport],
  )

  const handleExportPdf = React.useCallback(
    async (options?: { allData?: boolean; selectedOnly?: boolean }) => {
      try {
        const exportData = await getDataForExport(options)
        if (!exportData || exportData.length === 0) {
          toast.error('No data found to export')
          return
        }

        if (onExportPDF) {
          onExportPDF(exportData)
        }
        exportToPdf(exportData, builtColumns, 'export.pdf')
      } catch (err: unknown) {
        console.error('Export Error:', err)
        toast.error(
          'Failed to export: ' +
            (err instanceof Error ? err.message : 'Unknown error'),
        )
      }
    },
    [builtColumns, onExportPDF, getDataForExport],
  )

  const handlePasteFromClipboard = React.useCallback(async () => {
    if (!allowPaste) {
      toast('Paste is disabled for this table.')
      return
    }
    try {
      const text = await navigator.clipboard.readText()
      if (!text) {
        toast('Clipboard is empty.')
        return
      }
      const trimmed = text.trim()
      if (
        (trimmed.startsWith('{') || trimmed.startsWith('[')) &&
        onImportJSON
      ) {
        const parsed: unknown = JSON.parse(trimmed)
        if (!Array.isArray(parsed) || !parsed.every(isObject)) {
          throw new Error('JSON import requires an array of rows.')
        }
        onImportJSON(parsed)
        toast.success('JSON data pasted successfully.')
        return
      }
      if (onImportCSV) {
        const parsed = parseCsv(text, {
          columns: true,
          skip_empty_lines: true,
          cast: true,
        })
        if (Array.isArray(parsed) && parsed.every(isObject)) {
          onImportCSV(parsed)
          toast.success('CSV data pasted successfully.')
        }
        return
      }
      toast('No import handler registered for clipboard data.')
    } catch (err) {
      console.error(err)
      toast.error(err instanceof Error ? err.message : 'Unable to paste data.')
    }
  }, [allowPaste, onImportCSV, onImportJSON])

  const toolbarContext = React.useMemo<DataTableToolbarContext<TRow>>(() => {
    return {
      table,
      selectionCount: table.getSelectedRowModel().rows.length,
      selectedRows: table.getSelectedRowModel().rows.map((row) => row.original),
      selectionMap: resolvedRowSelection,
      data,
      columnVisibility: resolvedColumnVisibility,
      sortingMode: 'server',
      density,
      setDensity,
      importCsv: () => csvInputRef.current?.click(),
      importJson: () => jsonInputRef.current?.click(),
      exportCsv: handleExportCsv,
      exportPdf: handleExportPdf,
      generateTemplateCsv: handleGenerateTemplateCsv,
      pasteFromClipboard: handlePasteFromClipboard,
      onAdd,
      onAddLabel,
    }
  }, [
    table,
    resolvedRowSelection,
    data,
    resolvedColumnVisibility,
    density,
    handleExportCsv,
    handleExportPdf,
    handleGenerateTemplateCsv,
    handlePasteFromClipboard,
    onAdd,
    onAddLabel,
  ])

  const renderRows = () => {
    return table.getRowModel().rows.map((row) => {
      const rowState = highlightRow?.(row.original)
      const rowContent = (
        <TableRow
          key={row.id}
          data-state={rowState}
          className={cn(
            'border-b-border/30 hover:bg-muted/20 transition-colors',
            rowState === 'success' && 'bg-emerald-500/10',
            rowState === 'warning' && 'bg-amber-500/10',
            rowState === 'danger' && 'bg-destructive/10',
          )}
        >
          {row.getVisibleCells().map((cell) => {
            const meta = getColumnMeta(cell.column)
            const isPinned = cell.column.getIsPinned()
            return (
              <TableCell
                key={cell.id}
                className={cn(
                  density === 'compact' ? 'py-2' : 'py-4',
                  meta?.align === 'center' && 'text-center',
                  meta?.align === 'end' && 'text-right',
                  meta?.cellClassName,
                  isPinned &&
                    'sticky bg-card z-10 shadow-[inset_-1px_0_0_0_rgba(0,0,0,0.1)]',
                  isPinned === 'left'
                    ? 'left-0'
                    : isPinned === 'right'
                      ? 'right-0'
                      : '',
                )}
                style={{ width: meta?.width }}
              >
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </TableCell>
            )
          })}
        </TableRow>
      )

      const subComponent = renderSubComponent ? (
        <TableRow
          key={`${row.id}-expanded`}
          className="bg-muted/5 border-b-border/30"
        >
          <TableCell colSpan={row.getVisibleCells().length}>
            {renderSubComponent({ row })}
          </TableCell>
        </TableRow>
      ) : null

      const rowElement = contextMenuItems ? (
        <ContextMenu key={row.id}>
          <ContextMenuTrigger render={rowContent} />
          <ContextMenuContent className="min-w-40" alignOffset={-5}>
            {contextMenuItems(row.original)}
          </ContextMenuContent>
        </ContextMenu>
      ) : (
        rowContent
      )

      return row.getIsExpanded() ? [rowElement, subComponent] : rowElement
    })
  }

  const noData = !isLoading && table.getRowModel().rows.length === 0

  const handlePageIndexChange = (index: number) => {
    setUrlState({ page: index + 1 })
    onPageIndexChange?.(index)
  }

  return (
    <Stack gap={3} className="w-full h-full">
      {showDefaultToolbar && (
        <DataTableToolbar
          context={toolbarContext}
          allowColumnVisibilityToggle={allowColumnVisibilityToggle}
          allowPaste={allowPaste}
          onImportCSV={onImportCSV}
          onImportJSON={onImportJSON}
          onExportCSV={handleExportCsv}
          onExportPDF={handleExportPdf}
          onFetchFullData={onFetchFullData}
          baseColumns={baseColumns}
          bulkActions={bulkActions}
          facetedFilters={facetedFilters}
          search={search}
          onSearchChange={onSearchChange}
          searchPlaceholder={searchPlaceholder}
          extraActions={extraActions}
        />
      )}
      {toolbar?.(toolbarContext)}

      <Box
        rounded="lg"
        bg="bg-card"
        className="border border-border/40 shadow-sm overflow-hidden flex-1"
      >
        <div className="overflow-auto w-full h-full">
          <UiTable>
            <TableHeader className="bg-muted/30 sticky top-0 z-20">
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow
                  key={headerGroup.id}
                  className="hover:bg-transparent border-b-border/40"
                >
                  {headerGroup.headers.map((header) => {
                    const isPinned = header.column.getIsPinned()
                    return (
                      <TableHead
                        key={header.id}
                        className={cn(
                          'h-11 font-semibold text-foreground/80',
                          isPinned &&
                            'sticky bg-muted/30 z-30 shadow-[inset_-1px_0_0_0_rgba(0,0,0,0.1)]',
                          isPinned === 'left'
                            ? 'left-0'
                            : isPinned === 'right'
                              ? 'right-0'
                              : '',
                        )}
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
                  <TableRow
                    key={`skeleton-${index}`}
                    className="border-b-border/30"
                  >
                    {builtColumns.map((col, colIndex) => {
                      const meta = col.meta
                      return (
                        <TableCell
                          key={colIndex}
                          className="py-4 align-middle"
                          style={{ width: meta?.width }}
                        >
                          <Skeleton className="h-4 bg-muted/50" />
                        </TableCell>
                      )
                    })}
                  </TableRow>
                ))
              ) : noData ? (
                <TableRow>
                  <TableCell
                    colSpan={builtColumns.length || 1}
                    className="h-24 text-center text-muted-foreground"
                  >
                    {emptyState || 'No results found.'}
                  </TableCell>
                </TableRow>
              ) : (
                renderRows()
              )}
            </TableBody>
          </UiTable>
        </div>
      </Box>

      <DataTablePagination
        table={table}
        pageIndex={pageIndex}
        pageSize={pageSize}
        pageCount={pageCount}
        canNextPage={canNextPage}
        canPreviousPage={canPreviousPage}
        fetchNextPage={fetchNextPage}
        fetchPreviousPage={fetchPreviousPage}
        onPageSizeChange={(size) => {
          setUrlState({ limit: size })
          onPageSizeChange?.(size)
        }}
        onPageIndexChange={handlePageIndexChange}
        pageSizeOptions={pageSizeOptions}
        isLoading={isLoading}
        totalCount={data.length}
      />

      <input
        ref={csvInputRef}
        type="file"
        className="hidden"
        accept=".csv,text/csv"
        onChange={handleCsvFileChange}
      />
      <input
        ref={jsonInputRef}
        type="file"
        className="hidden"
        accept=".json,application/json"
        onChange={handleJsonFileChange}
      />
    </Stack>
  )
}
