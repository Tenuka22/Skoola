import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { Cancel01Icon, Shield01Icon } from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
import type { PermissionEnum } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Box, HStack, Stack, Text } from '@/components/primitives'

export function RoleEditorDialog() {
  const { selectedRoleId, isRoleEditorOpen, setIsRoleEditorOpen } =
    useRBACStore()
  const queryClient = useQueryClient()

  const { data: rawPermissions } = useQuery({
    ...rbacApi.getRolePermissionsOptions(selectedRoleId || ''),
    enabled: !!selectedRoleId,
  })

  const assignedPermissions = React.useMemo(() => {
    const perms = rawPermissions?.permissions || []
    return perms.filter(isPermissionEnum)
  }, [rawPermissions])

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToRoleMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRolePermissionsOptions(selectedRoleId || '')
          .queryKey,
      })
      toast.success('Permission assigned to role')
    },
    onError: (err) => {
      toast.error(
        err instanceof Error ? err.message : 'Failed to assign permission',
      )
    },
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromRoleMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: rbacApi.getRolePermissionsOptions(selectedRoleId || '')
          .queryKey,
      })
      toast.success('Permission removed from role')
    },
    onError: (err) => {
      toast.error(
        err instanceof Error ? err.message : 'Failed to remove permission',
      )
    },
  })

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (!selectedRoleId) return

    if (checked) {
      assignPerm.mutate({
        path: { role_id: selectedRoleId },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { role_id: selectedRoleId },
        body: { permission },
      })
    }
  }

  return (
    <Dialog open={isRoleEditorOpen} onOpenChange={setIsRoleEditorOpen}>
      <DialogContent className="max-w-2xl h-[85vh] flex flex-col p-0 gap-0 overflow-hidden">
        <DialogHeader className="p-6 pb-4">
          <HStack align="start" gap={4}>
            <Box
              p={3}
              rounded="lg"
              className="bg-muted border dark:bg-zinc-900 dark:border-zinc-800"
            >
              <HugeiconsIcon
                icon={Shield01Icon}
                className="size-6 text-primary"
              />
            </Box>
            <Stack gap={1}>
              <DialogTitle>
                <HStack align="center" gap={2}>
                  <Text as="span" size="xl" className="font-bold">
                    Configure Role:
                  </Text>
                  <Text as="span" size="xl" className="font-bold text-primary">
                    {selectedRoleId}
                  </Text>
                </HStack>
              </DialogTitle>
              <Text size="sm" muted>
                Baseline permissions for all users assigned to the{' '}
                <Text as="span" className="font-semibold text-foreground">
                  {selectedRoleId}
                </Text>{' '}
                role.
              </Text>
            </Stack>
          </HStack>
        </DialogHeader>

        <Box className="flex-1 min-h-0 overflow-hidden px-6">
          <HStack align="center" justify="between" className="mb-4">
            <Text
              size="xs"
              className="font-bold uppercase tracking-wider text-muted-foreground"
            >
              Permissions Management
            </Text>
            <Badge variant="secondary" className="font-mono">
              {assignedPermissions.length} assigned
            </Badge>
          </HStack>

          <Box className="h-full pb-8">
            <PermissionList
              assignedPermissions={assignedPermissions}
              onToggle={handleTogglePermission}
            />
          </Box>
        </Box>

        <DialogFooter className="p-4 border-t">
          <HStack justify="end" gap={2}>
            <Button
              variant="outline"
              onClick={() => setIsRoleEditorOpen(false)}
            >
              <HugeiconsIcon icon={Cancel01Icon} className="size-4 mr-2" />
              Close
            </Button>
          </HStack>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
