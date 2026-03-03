import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'

import { format } from 'date-fns'
import type { AuditLogResponse } from '@/lib/api/types.gen'
import type { DataTableColumnDef } from '@/components/data-table'
import { Badge } from '@/components/ui/badge'
import { getAllAuditLogsQueryOptions } from '@/features/audit/api'
import { DataTable, DataTableColumnHeader } from '@/components/data-table'
import { Box, Stack, Text } from '@/components/primitives'

export const Route = createFileRoute('/admin/audit')({
  component: AuditLogsPage,
})

function AuditLogsPage() {
  const [search, setSearch] = React.useState('')

  const { data: auditLogs = [], isLoading } = useQuery(
    getAllAuditLogsQueryOptions(),
  )

  const columns = React.useMemo<Array<DataTableColumnDef<AuditLogResponse>>>(
    () => [
      {
        accessorKey: 'action_type',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Action" />
        ),
        cell: ({ row }) => (
          <Stack gap={1}>
            <Text size="sm" className="font-semibold">
              {row.original.action_type}
            </Text>
            <Text size="xs" muted>
              {format(new Date(row.original.timestamp), 'PPP p')}
            </Text>
          </Stack>
        ),
      },
      {
        accessorKey: 'table_name',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Table" />
        ),
        cell: ({ row }) => (
          <Badge variant="outline" className="font-mono text-[10px]">
            {row.original.table_name}
          </Badge>
        ),
      },
      {
        accessorKey: 'user_id',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="User" />
        ),
      },
      {
        accessorKey: 'record_pk',
        header: ({ column }) => (
          <DataTableColumnHeader column={column} title="Record PK" />
        ),
        cell: ({ row }) => (
          <Text size="xs" className="font-mono">
            {row.original.record_pk}
          </Text>
        ),
      },
    ],
    [],
  )

  const filteredLogs = React.useMemo(() => {
    if (!search) return auditLogs
    const s = search.toLowerCase()
    return auditLogs.filter(
      (log) =>
        log.table_name.toLowerCase().includes(s) ||
        log.action_type.toLowerCase().includes(s) ||
        log.user_id.toLowerCase().includes(s),
    )
  }, [auditLogs, search])

  return (
    <Stack gap={6} p={8} className="h-full">
      <Stack gap={1}>
        <Text size="2xl" className="font-bold tracking-tight">
          Audit Logs
        </Text>
        <Text muted>
          Track all critical actions and data changes within the system.
        </Text>
      </Stack>

      <Box className="flex-1 overflow-hidden">
        <DataTable
          columns={columns}
          data={filteredLogs}
          isLoading={isLoading}
          pageIndex={0}
          pageSize={filteredLogs.length || 10}
          pageCount={1}
          canNextPage={false}
          canPreviousPage={false}
          fetchNextPage={() => {}}
          fetchPreviousPage={() => {}}
          search={search}
          onSearchChange={setSearch}
          searchPlaceholder="Filter logs..."
          enableExpansion
          renderSubComponent={({ row }) => (
            <div className="p-4 bg-muted/30 rounded-md">
              <Text size="xs" className="font-semibold mb-2 block">
                Old Value (JSON)
              </Text>
              {row.original.old_value_json ? (
                <pre className="text-[10px] text-muted-foreground overflow-auto max-h-60 p-2 bg-background border rounded">
                  {JSON.stringify(
                    JSON.parse(row.original.old_value_json),
                    null,
                    2,
                  )}
                </pre>
              ) : (
                <Text size="xs" muted>
                  No previous value recorded.
                </Text>
              )}
            </div>
          )}
        />
      </Box>
    </Stack>
  )
}
