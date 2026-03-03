import type * as React from 'react'
import type {
  ColumnDef,
  ColumnFiltersState,
  OnChangeFn,
  Row,
  SortingState,
  Table,
  VisibilityState,
} from '@tanstack/react-table'
import type { FieldPath, FieldValues } from 'react-hook-form'
import type { ZodTypeAny, input } from 'zod'

export type FormInput<TSchema extends ZodTypeAny> = input<TSchema>

export type AutoColumnAlign = 'start' | 'center' | 'end'

export type DataTableDensity = 'compact' | 'comfortable'

export type DataTableColumnMeta<TRow = unknown> = {
  align?: AutoColumnAlign
  width?: string
  description?: string
  cellClassName?: string
  onMarkAttendance?: (record: TRow) => void
  isPinned?: 'left' | 'right'
}

export type DataTableColumnDef<TRow = unknown, TValue = unknown> = ColumnDef<
  TRow,
  TValue
> & {
  meta?: DataTableColumnMeta<TRow>
}

export type DataTableFieldConfig<TRow> = {
  field: TRow extends FieldValues ? FieldPath<TRow> : string
  header?: React.ReactNode
  description?: string
  sortable?: boolean
  visible?: boolean
  align?: AutoColumnAlign
  width?: string
  className?: string
  truncate?: boolean
  filterable?: boolean
  accessor?: (row: TRow) => unknown
  render?: (value: unknown, row: TRow, index: number) => React.ReactNode
}

export type DataTableAutoConfig<TRow> = {
  columns: Array<DataTableFieldConfig<TRow>>
  rowId?: (row: TRow) => string | number
}

export type SortingMode = 'auto' | 'client' | 'server'

export type DataTableFacetedFilterOption = {
  label: string
  value: string
  icon?: React.ComponentType<{ className?: string }>
}

export type DataTableFacetedFilter = {
  columnId: string
  title: string
  options: Array<DataTableFacetedFilterOption>
}

export type DataTableToolbarContext<TRow> = {
  table: Table<TRow>
  selectionCount: number
  selectedRows: Array<TRow>
  selectionMap: Record<string, boolean>
  data: Array<TRow>
  columnVisibility: VisibilityState
  sortingMode: SortingMode
  density: DataTableDensity
  setDensity: (density: DataTableDensity) => void
  importCsv: () => void
  importJson: () => void
  exportCsv: (options?: { allData?: boolean; selectedOnly?: boolean }) => void
  exportPdf: (options?: { allData?: boolean; selectedOnly?: boolean }) => void
  generateTemplateCsv: () => void
  pasteFromClipboard: () => void
  onAdd?: () => void
  onAddLabel?: string
}

export interface DataTableProps<TRow extends { id?: string | number }> {
  schema?: ZodTypeAny
  config?: DataTableAutoConfig<TRow>
  columns?: Array<DataTableColumnDef<TRow, unknown>>
  data: Array<TRow>
  pageIndex: number
  pageSize: number
  pageCount: number
  canNextPage: boolean
  canPreviousPage: boolean
  fetchNextPage: () => void
  fetchPreviousPage: () => void
  sorting?: SortingState
  onSortingChange?: OnChangeFn<SortingState>
  columnFilters?: ColumnFiltersState
  onColumnFiltersChange?: OnChangeFn<ColumnFiltersState>
  columnVisibility?: VisibilityState
  onColumnVisibilityChange?: OnChangeFn<VisibilityState>
  rowSelection?: Record<string, boolean>
  onRowSelectionChange?: OnChangeFn<Record<string, boolean>>
  isLoading?: boolean
  onPageSizeChange?: (pageSize: number) => void
  onPageIndexChange?: (pageIndex: number) => void
  contextMenuItems?: (row: TRow) => React.ReactNode
  rowId?: (row: TRow) => string | number
  rowActions?: (row: TRow) => React.ReactNode
  highlightRow?: (row: TRow) => 'success' | 'warning' | 'danger' | undefined
  emptyState?: React.ReactNode
  pageSizeOptions?: Array<number>
  toolbar?: (context: DataTableToolbarContext<TRow>) => React.ReactNode
  showDefaultToolbar?: boolean
  allowColumnVisibilityToggle?: boolean
  allowSortingModeSwitch?: boolean
  allowPaste?: boolean
  enableSelection?: boolean
  enablePinning?: boolean
  enableExpansion?: boolean
  renderSubComponent?: (props: { row: Row<TRow> }) => React.ReactNode
  bulkActions?: (context: {
    selectedRows: Array<TRow>
    table: Table<TRow>
  }) => React.ReactNode
  facetedFilters?: Array<DataTableFacetedFilter>
  onImportCSV?: (rows: Array<Record<string, unknown>>) => void
  onImportJSON?: (rows: Array<Record<string, unknown>>) => void
  onExportCSV?: (rows: Array<TRow>) => void
  onExportPDF?: (rows: Array<TRow>) => void
  onFetchFullData?: () => Promise<Array<TRow>>
  onAdd?: () => void
  onAddLabel?: string
  sortingMode?: SortingMode
  templateFilename?: string
  search?: string
  onSearchChange?: (value: string) => void
  searchPlaceholder?: string
  extraActions?: React.ReactNode
}
