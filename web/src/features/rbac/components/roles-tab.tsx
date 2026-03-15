import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Settings01Icon, Shield01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useRBACSearchParams } from '../search-params'
import { isPermissionEnum } from '../utils/permissions'
import { getRolePermissionsQueryOptions } from '../api'
import type { RoleEnum } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'
import { Grid, HStack, Stack, Text } from '@/components/primitives'
import { cn } from '@/lib/utils'

export function RolesTab() {
  const { setSelectedRoleId, setIsRoleEditorOpen } = useRBACSearchParams()

  return (
    <Grid cols={3} gap={6}>
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
    </Grid>
  )
}

function RoleCard({
  role,
  onManage,
}: {
  role: RoleEnum
  onManage: () => void
}) {
  const { data: rawPermissions, isLoading } = useQuery(
    getRolePermissionsQueryOptions({ path: { role_id: role } }),
  )

  const permissionsFromResponse = React.useMemo(() => {
    if (!rawPermissions || typeof rawPermissions !== 'object') {
      return []
    }

    if (!('permissions' in rawPermissions)) {
      return []
    }

    const candidate = rawPermissions.permissions
    return Array.isArray(candidate)
      ? candidate.filter((p: unknown) => typeof p === 'string')
      : []
  }, [rawPermissions])

  const permissionCount = React.useMemo(() => {
    return permissionsFromResponse.filter(isPermissionEnum).length
  }, [permissionsFromResponse])

  const description =
    role === 'FullAdmin'
      ? 'Root administrative role with unrestricted access to all system features and sensitive data.'
      : role === 'Admin'
        ? 'Full administrative access for managing school operations, staff, and students.'
        : role === 'Teacher'
          ? 'Access to academic management, grading, and student progress tracking.'
          : role === 'Student'
            ? 'Access to learning resources, timetable, and personal progress.'
            : `System role defining baseline permissions and access levels for all ${role.toLowerCase()} users.`

  return (
    <Card
      className={cn(
        'p-3',
        role === 'FullAdmin' && 'bg-red-500/10',
        role === 'Admin' && 'bg-yellow-500/10',
      )}
    >
      <HStack align="start" justify="between" gap={3}>
        <Stack gap={1}>
          <HStack gap={2} align="center">
            <HugeiconsIcon icon={Shield01Icon} />
            <Text size="sm" className="font-medium">
              {role}
            </Text>
          </HStack>
          <Badge variant="secondary" className="text-[10px] px-1.5 py-0 w-fit">
            {isLoading ? 'Loading...' : `${permissionCount} Permissions`}
          </Badge>
        </Stack>

        <Button
          variant="secondary"
          size="sm"
          onClick={onManage}
          className="gap-2 self-start"
        >
          <HugeiconsIcon icon={Settings01Icon} className="size-4" />
          Configure
        </Button>
      </HStack>
      <Stack gap={4} className="mt-3">
        <Text size="sm" muted>
          {description}
        </Text>
      </Stack>
    </Card>
  )
}
