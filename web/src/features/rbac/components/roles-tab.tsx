import { useQuery } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Settings01Icon, Shield01Icon } from '@hugeicons/core-free-icons'
import * as React from 'react'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import type { RoleEnum } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'
import { Grid, HStack, Stack, Text } from '@/components/primitives'

export function RolesTab() {
  const { setSelectedRoleId, setIsRoleEditorOpen } = useRBACStore()

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
  const { data: rawPermissions, isLoading } = useQuery({
    ...rbacApi.getRolePermissionsOptions(role),
  })

  const permissionCount = React.useMemo(() => {
    const perms = rawPermissions?.permissions || []
    return perms.filter(isPermissionEnum).length
  }, [rawPermissions])

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
    <Card>
      <CardHeader>
        <HStack align="start" justify="between">
          <Stack gap={2}>
            <HStack align="center" gap={3}>
              <HugeiconsIcon icon={Shield01Icon} className="size-5" />
              <CardTitle>{role}</CardTitle>
            </HStack>
            <Badge variant="secondary" className="w-fit">
              {isLoading ? 'Loading...' : `${permissionCount} Permissions`}
            </Badge>
          </Stack>
        </HStack>
      </CardHeader>
      <CardContent>
        <Stack gap={4}>
          <Text size="sm" muted>
            {description}
          </Text>
          <Button
            variant="secondary"
            size="sm"
            onClick={onManage}
            className="gap-2 self-start"
          >
            <HugeiconsIcon icon={Settings01Icon} className="size-4" />
            Configure Role
          </Button>
        </Stack>
      </CardContent>
    </Card>
  )
}
