import { useQuery } from '@tanstack/react-query'
import { Badge } from '@/components/ui/badge'
import { authClient } from '@/lib/clients'
import { getPermissions9C8839E73223Cb930255A2882A4B0Db4Options as getPermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'

export function PermissionsHeader() {
  const { data: permissionsData } = useQuery(
    getPermissionsOptions({
      client: authClient,
      query: { limit: 1 },
    }),
  )

  return (
    <div className="px-8 py-6 pb-2">
      <div className="mb-1 flex items-center gap-3">
        <h1 className="text-2xl font-semibold tracking-tight">
          Permissions Management
        </h1>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {permissionsData?.total || 0}
        </Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        Define and manage granular access control policies and system
        permissions.
      </p>
    </div>
  )
}
