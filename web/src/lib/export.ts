export type Column<T> = {
  header: string
  accessor:
    | keyof T
    | ((item: T) => string | number | boolean | null | undefined)
}

export const handleExportCSV = <T extends Record<string, unknown>>(
  data: Array<T>,
  filename: string,
  columns?: Array<Column<T>>,
) => {
  if (!data || (!data.length && !columns)) return

  const cols: Array<Column<T>> =
    columns ||
    (data.length
      ? Object.keys(data[0]).map((key) => ({
          header: key,
          accessor: key,
        }))
      : [])

  const escapeCSV = (field: string | number | boolean | null | undefined) => {
    if (field === null || field === undefined) return ''
    const stringField = String(field)
    if (
      stringField.includes(',') ||
      stringField.includes('\n') ||
      stringField.includes('"')
    ) {
      return `"${stringField.replace(/"/g, '""')}"`
    }
    return stringField
  }

  const headers = cols.map((c) => c.header)
  const rows = data.map((row) =>
    cols.map((col) => {
      let value: string | number | boolean | null | undefined
      if (typeof col.accessor === 'function') {
        value = col.accessor(row)
      } else {
        const rawValue: unknown = row[col.accessor]
        if (
          rawValue === null ||
          rawValue === undefined ||
          typeof rawValue === 'string' ||
          typeof rawValue === 'number' ||
          typeof rawValue === 'boolean'
        ) {
          value = rawValue
        } else {
          value = String(rawValue)
        }
      }
      return value
    }),
  )

  const csvContent =
    'data:text/csv;charset=utf-8,' +
    [
      headers.map(escapeCSV).join(','),
      ...rows.map((row) => row.map(escapeCSV).join(',')),
    ].join('\n')

  const encodedUri = encodeURI(csvContent)
  const link = document.createElement('a')
  link.setAttribute('href', encodedUri)
  link.setAttribute('download', filename)
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}
