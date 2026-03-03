import { jsPDF } from 'jspdf'
import autoTable from 'jspdf-autotable'
import { stringify as serializeCsv } from 'csv-stringify/browser/esm/sync'
import type { Column } from '@tanstack/react-table'
import type * as React from 'react'

import type {
  DataTableAutoConfig,
  DataTableColumnDef,
  DataTableColumnMeta,
  DataTableFieldConfig,
  FormInput,
} from './types'
import type { ZodTypeAny } from 'zod'

export function createDataTableConfig<TSchema extends ZodTypeAny>() {
  return (columns: Array<DataTableFieldConfig<FormInput<TSchema>>>) => columns
}

export function defineDataTableConfig<TSchema extends ZodTypeAny>(
  _schema: TSchema,
  config: DataTableAutoConfig<FormInput<TSchema>>,
) {
  return config
}

export function inferLabelFromField(field: string): string {
  const parts = field.split('.')
  const lastPart = parts[parts.length - 1] ?? field
  return lastPart
    .split('_')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ')
}

export function getValueAtPath(
  row: Record<string, unknown>,
  path: string,
): unknown {
  return path.split('.').reduce<unknown>((value, segment) => {
    if (!isObject(value)) return undefined
    return value[segment]
  }, row)
}

export function renderPrimitive(
  value: unknown,
  options?: { truncate?: boolean },
): React.ReactNode {
  if (value === null || value === undefined || value === '') {
    return 'N/A'
  }

  if (typeof value === 'boolean') {
    return value ? 'Yes' : 'No'
  }

  if (value instanceof Date) {
    return value.toLocaleString()
  }

  if (Array.isArray(value)) {
    return value.map((item, index) => (
      <span key={index}>
        {renderPrimitive(item, options)}
        {index !== value.length - 1 && ', '}
      </span>
    ))
  }

  if (isObject(value)) {
    return JSON.stringify(value)
  }

  const stringValue = String(value)
  if (options?.truncate) {
    return (
      <span className="truncate block" title={stringValue}>
        {stringValue}
      </span>
    )
  }

  return stringValue
}

export function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null
}

export function isDataTableColumnMeta<TRow>(
  meta: unknown,
): meta is DataTableColumnMeta<TRow> {
  return !!meta && typeof meta === 'object'
}

export function getColumnMeta<TRow>(
  column: Column<TRow, unknown>,
): DataTableColumnMeta<TRow> | undefined {
  const meta = column.columnDef.meta
  if (isDataTableColumnMeta<TRow>(meta)) {
    return meta
  }
  return undefined
}

export function buildColumnsFromAutoConfig<TRow>(
  config: DataTableAutoConfig<TRow>,
): Array<DataTableColumnDef<TRow, unknown>> {
  return config.columns.map((column) => {
    const col: DataTableColumnDef<TRow, unknown> = {
      id: column.field,
      accessorFn: (row: TRow) => {
        if (column.accessor) return column.accessor(row)
        if (isObject(row)) return getValueAtPath(row, column.field)
        return undefined
      },
      header:
        column.header === undefined
          ? inferLabelFromField(column.field)
          : typeof column.header === 'string'
            ? column.header
            : () => column.header,
      cell: ({ row, row: { index } }) => {
        const accessor =
          column.accessor ??
          ((r: TRow) =>
            isObject(r) ? getValueAtPath(r, column.field) : undefined)
        const value = accessor(row.original)
        if (column.render) {
          return column.render(value, row.original, index)
        }
        return renderPrimitive(value, column)
      },
      meta: {
        align: column.align ?? 'start',
        width: column.width,
        description: column.description,
        cellClassName: column.className,
      },
      enableSorting: column.sortable ?? true,
      enableHiding: column.visible !== false,
      enableColumnFilter: column.filterable ?? false,
    }
    return col
  })
}

export function jsonReplacer(_key: string, value: unknown): unknown {
  // Use the value as passed by JSON.stringify
  if (typeof value === 'bigint') {
    return value.toString()
  }
  return value
}

export function downloadFile(
  content: string,
  filename: string,
  mimeType = 'text/plain',
) {
  if (typeof document === 'undefined') return
  const blob = new Blob([content], { type: mimeType })
  const url = URL.createObjectURL(blob)
  const anchor = document.createElement('a')
  anchor.href = url
  anchor.download = filename
  document.body.appendChild(anchor)
  anchor.click()
  document.body.removeChild(anchor)
  setTimeout(() => URL.revokeObjectURL(url), 2000)
}

function getRowValue<TRow>(
  row: TRow,
  col: DataTableColumnDef<TRow, unknown>,
): unknown {
  const id = getColId(col)
  if (id && isObject(row)) {
    const val = getValueAtPath(row, id)
    if (val !== undefined) return val
  }
  if ('accessorFn' in col && typeof col.accessorFn === 'function') {
    return col.accessorFn(row, 0)
  }
  return undefined
}

export function getColId<TRow>(
  col: DataTableColumnDef<TRow, unknown>,
): string | undefined {
  if (col.id) return col.id
  if ('accessorKey' in col && typeof col.accessorKey === 'string') {
    return col.accessorKey
  }
  // Safe access for internal TanStack access
  if ('id' in col && typeof col.id === 'string') {
    return col.id
  }
  return undefined
}

export function getColHeader<TRow>(
  col: DataTableColumnDef<TRow, unknown>,
): string {
  if (typeof col.header === 'string') return col.header
  const id = getColId(col)
  if (id) return inferLabelFromField(id)
  return ''
}

export function exportToCsv<TRow>(
  data: Array<TRow>,
  columns: Array<DataTableColumnDef<TRow, unknown>>,
  filename = 'export.csv',
) {
  const exportableColumns = columns.filter((col) => {
    const id = getColId(col)
    return id && id !== 'select' && id !== 'row-actions' && id !== 'expand'
  })

  const headers = exportableColumns.map((col) => getColHeader(col))

  const rows = data.map((row) => {
    return exportableColumns.map((col) => {
      const value = getRowValue(row, col)
      if (value === null || value === undefined) return ''
      if (typeof value === 'object') {
        try {
          return JSON.stringify(value, jsonReplacer)
        } catch {
          return String(value)
        }
      }
      return String(value)
    })
  })

  const csv = serializeCsv([headers, ...rows])
  downloadFile(csv, filename, 'text/csv;charset=utf-8')
}

export function exportToPdf<TRow>(
  data: Array<TRow>,
  columns: Array<DataTableColumnDef<TRow, unknown>>,
  filename = 'export.pdf',
) {
  const doc = new jsPDF()

  const exportableColumns = columns.filter((col) => {
    const id = getColId(col)
    return id && id !== 'select' && id !== 'row-actions' && id !== 'expand'
  })

  const headers = exportableColumns.map((col) => getColHeader(col))

  const rows = data.map((row) => {
    return exportableColumns.map((col) => {
      const value = getRowValue(row, col)
      if (value === null || value === undefined) return ''
      if (typeof value === 'object') {
        try {
          return JSON.stringify(value, jsonReplacer)
        } catch {
          return String(value)
        }
      }
      return String(value)
    })
  })

  autoTable(doc, {
    head: [headers],
    body: rows,
  })

  doc.save(filename)
}
