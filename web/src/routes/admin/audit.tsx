import { createFileRoute } from '@tanstack/react-router'
import {
  useQuery,
} from '@tanstack/react-query'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { ActivityIcon, AlertCircleIcon, Search01Icon } from '@hugeicons/core-free-icons'

import { format } from 'date-fns'
import type { AuditLogResponse } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import {
  getAllAuditLogsOptions,
} from '@/lib/api/@tanstack/react-query.gen'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Badge } from '@/components/ui/badge'
import { Spinner } from '@/components/ui/spinner'

export const Route = createFileRoute('/admin/audit')({
  component: AuditLogsPage,
})

function AuditLogsPage() {
  const [search, setSearch] = React.useState('')

  const { data: auditLogsData, isLoading, isError, error } = useQuery({
    ...getAllAuditLogsOptions({ client: authClient }),
  })

  const auditLogs = auditLogsData || []

  const filteredLogs = auditLogs.filter(
    (log: AuditLogResponse) =>
      log.table_name.toLowerCase().includes(search.toLowerCase()) ||
      log.action_type.toLowerCase().includes(search.toLowerCase()) ||
      log.user_id.toLowerCase().includes(search.toLowerCase()),
  )

  return (
    <div className="flex flex-col h-full gap-6 p-8">
      <div className="flex flex-col gap-1">
        <h1 className="text-3xl font-bold tracking-tight">Audit Logs</h1>
        <p className="text-muted-foreground">
          Track all critical actions and data changes within the system.
        </p>
      </div>

      <div className="relative">
        <HugeiconsIcon
          icon={Search01Icon}
          className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
        />
        <Input
          placeholder="Filter logs by table, action, or user..."
          className="pl-9 h-10 w-full max-w-md"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>

      <ScrollArea className="flex-1 border rounded-xl bg-card">
        {isLoading ? (
          <div className="grid place-items-center h-64">
            <Spinner />
          </div>
        ) : isError ? (
          <div className="grid place-items-center h-64 text-center">
            <HugeiconsIcon icon={AlertCircleIcon} className="size-12 text-destructive opacity-50" />
            <p className="mt-4 text-muted-foreground">Error: {error.message}</p>
          </div>
        ) : filteredLogs.length === 0 ? (
          <p className="text-center text-muted-foreground italic py-12">
            No audit logs found.
          </p>
        ) : (
          <div className="divide-y">
            {filteredLogs.map((log: AuditLogResponse) => (
              <div key={log.id} className="p-4 hover:bg-muted/50 transition-colors flex items-start gap-4">
                <div className="size-8 rounded-full bg-primary/10 flex items-center justify-center mt-0.5">
                  <HugeiconsIcon icon={ActivityIcon} className="size-4 text-primary" />
                </div>
                <div className="flex-1 grid grid-cols-1 md:grid-cols-4 gap-4">
                  <div className="col-span-1 space-y-1">
                    <p className="text-sm font-semibold">{log.action_type}</p>
                    <p className="text-xs text-muted-foreground">{format(new Date(log.timestamp), 'PPP p')}</p>
                  </div>
                  <div className="col-span-1 space-y-1">
                    <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">Table</span>
                    <p className="text-sm font-mono">{log.table_name}</p>
                  </div>
                  <div className="col-span-1 space-y-1">
                    <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">User</span>
                    <p className="text-sm">{log.user_id}</p>
                  </div>
                  <div className="col-span-1 flex items-center justify-end">
                    <Badge variant="outline" className="font-mono text-[10px]">
                      PK: {log.record_pk}
                    </Badge>
                  </div>
                  {log.old_value_json && (
                     <div className="col-span-full mt-2 p-3 bg-muted rounded-lg overflow-x-auto">
                        <pre className="text-[10px] text-muted-foreground">
                          {JSON.stringify(JSON.parse(log.old_value_json), null, 2)}
                        </pre>
                     </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </ScrollArea>
    </div>
  )
}
