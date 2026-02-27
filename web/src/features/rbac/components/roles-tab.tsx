import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Settings01Icon, Shield01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useRBACStore } from '../store'
import type { RoleEnum } from '@/lib/api/types.gen'
import { authClient } from '@/lib/clients'
import { getRolePermissionsOptions } from '@/lib/api/@tanstack/react-query.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardAction,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'

export function RolesTab() {
  const { setSelectedRoleId, setIsRoleEditorOpen } = useRBACStore()

  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-col gap-1.5">
        <h2 className="text-sm font-semibold flex items-center gap-2">
          <HugeiconsIcon icon={Shield01Icon} className="size-4 text-primary" />
          System Roles
        </h2>
        <p className="text-xs text-muted-foreground">
          Baseline permissions for core user types.
        </p>
      </div>
      <div>
        {RoleEnumSchema.enum.map((role) => (
          <RoleCard
            key={role}
            role={role}
            onManage={() => {
              setSelectedRoleId(role)
              setIsRoleEditorOpen(true)
            }}
          />
        ))}
      </div>
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
  const { data: rawPermissions = '', isLoading } = useQuery({
    ...getRolePermissionsOptions({
      client: authClient,
      path: { role_id: role },
    }),
  })

  const permissionCount = React.useMemo(() => {
    if (typeof rawPermissions !== 'string' || !rawPermissions) return 0
    return rawPermissions.split(',').length
  }, [rawPermissions])

  return (
    <Card>
      <CardHeader className="flex flex-row items-start justify-between">
        <div className="flex flex-col gap-2">
          <div className="flex items-center gap-2">
            <HugeiconsIcon icon={Shield01Icon} className="size-5" />
            <CardTitle>{role}</CardTitle>
          </div>

          <Badge variant="secondary" className="w-fit">
            {isLoading ? 'Loading...' : `${permissionCount} PERMS`}
          </Badge>
        </div>

        <CardAction>
          <Button
            variant="secondary"
            size="sm"
            onClick={onManage}
            className="gap-2"
          >
            <HugeiconsIcon icon={Settings01Icon} className="size-4" />
            Configure
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent>
        {role === 'FullAdmin'
          ? 'Root administrative role with unrestricted access to all system features and sensitive data.'
          : role === 'Admin'
            ? 'Full administrative access for managing school operations, staff, and students.'
            : role === 'Teacher'
              ? 'Access to academic management, grading, and student progress tracking.'
              : role === 'Student'
                ? 'Access to learning resources, timetable, and personal progress.'
                : `System role defining baseline permissions and access levels for all ${role.toLowerCase()} users.`}
      </CardContent>
    </Card>
  )
}
