import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Settings01Icon, Shield01Icon } from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { RoleEditorDialog } from './role-editor-dialog'
import type { RoleEnum } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import { getRolePermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

const ROLES: Array<RoleEnum> = [
  'Admin',
  'Teacher',
  'Student',
  'Guest',
  'Parent',
  'FullAdmin',
  'Principal',
  'VicePrincipal',
  'Accountant',
  'Librarian',
]

export function RolesTab() {
  const { setSelectedRoleId, setIsRoleEditorOpen } = useRBACStore()

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
      {ROLES.map((role) => (
        <RoleCard
          key={role}
          role={role}
          onManage={() => {
            setSelectedRoleId(role)
            setIsRoleEditorOpen(true)
          }}
        />
      ))}
      <RoleEditorDialog />
    </div>
  )
}

function RoleCard({
  role,
  onManage,
}: {
  role: RoleEnum
  onManage: () => void
}) {
  const { data: permissions = [], isLoading } = useQuery(
    getRolePermissionsOptions({
      client: authClient,
      path: { role_id: role },
    }),
  )

  return (
    <Card className="group hover:border-primary/50 transition-all shadow-sm">
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <div className="size-10 rounded-lg bg-primary/10 flex items-center justify-center group-hover:bg-primary group-hover:text-primary-foreground transition-colors">
            <HugeiconsIcon icon={Shield01Icon} className="size-5" />
          </div>
          <Badge variant="outline" className="font-mono">
            {isLoading ? '...' : permissions.length} perms
          </Badge>
        </div>
        <CardTitle className="text-xl mt-4">{role}</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="text-sm text-muted-foreground line-clamp-2">
          {role === 'FullAdmin'
            ? 'Root administrative role with unrestricted access to all system features and data.'
            : `System role defining baseline permissions for all ${role.toLowerCase()} users.`}
        </p>
      </CardContent>
      <CardFooter>
        <Button
          variant="secondary"
          className="w-full group-hover:bg-primary group-hover:text-primary-foreground"
          onClick={onManage}
        >
          <HugeiconsIcon icon={Settings01Icon} className="size-4 mr-2" />
          Manage Permissions
        </Button>
      </CardFooter>
    </Card>
  )
}
