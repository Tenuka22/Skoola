import type { RowData } from '@tanstack/table-core'

declare module '@tanstack/table-core' {
  interface ColumnMeta<TData extends RowData> {
    onMarkAttendance?: (record: TData) => void
  }
}
