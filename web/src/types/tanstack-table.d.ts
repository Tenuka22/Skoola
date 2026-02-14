import '@tanstack/table-core'
import { RowData } from '@tanstack/table-core'

declare module '@tanstack/table-core' {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  interface ColumnMeta<TData extends RowData> {
    onMarkAttendance?: (record: TData) => void
  }
}
