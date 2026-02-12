import { Badge } from '@/components/ui/badge'
import { useQuery } from '@tanstack/react-query'
import { getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions } from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

export function UsersHeader() {
  const { data: stats } = useQuery(
    getUsersStatsBf304B57E4A0115F8280C4Bed2Fd9FbaOptions({
      client: authClient,
    }),
  )

  return (
    <div className="px-8 py-6 pb-2">
      <div className="mb-1 flex items-center gap-3">
        <h1 className="text-2xl font-semibold tracking-tight">
          User management
        </h1>
        <Badge
          variant="secondary"
          className="rounded-md bg-muted px-2 py-0.5 text-xs font-normal text-muted-foreground hover:bg-muted"
        >
          {stats?.total_users || 0}
        </Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        Manage your team members and their account permissions here.
      </p>
    </div>
  )
}
