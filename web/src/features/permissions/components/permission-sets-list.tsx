'use client'

import * as React from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import type { Permission } from '@/lib/api/types.gen'
import type { PermissionSet } from '../types'
import { Button } from '@/components/ui/button'
import { HugeiconsIcon } from '@hugeicons/react'
import { PlusSignIcon, Edit04Icon, Delete02Icon, UserGroupIcon } from '@hugeicons/core-free-icons'
import { CreatePermissionSetDialog } from './create-permission-set-dialog'
import { EditPermissionSetDialog } from './edit-permission-set-dialog'
import { ManagePermissionSetPermissionsDialog } from './manage-permission-set-permissions-dialog'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { deletePermissionSet } from '../../permissions/api'
import { toast } from 'sonner'

interface PermissionSetsListProps {
  permissionSets: PermissionSet[]
  isLoading: boolean
  allPermissions: Permission[]
}

export function PermissionSetsList({
  permissionSets,
  isLoading,
  allPermissions,
}: PermissionSetsListProps) {
  const queryClient = useQueryClient()
  const [isCreateDialogOpen, setIsCreateDialogOpen] = React.useState(false)
  const [editPermissionSet, setEditPermissionSet] = React.useState<PermissionSet | null>(null)
  const [managePermissionsForSet, setManagePermissionsForSet] = React.useState<PermissionSet | null>(null)

  const deleteMutation = useMutation({
    mutationFn: deletePermissionSet,
    onSuccess: () => {
      toast.success('Permission set deleted successfully.')
      queryClient.invalidateQueries({ queryKey: ['permissionSets'] })
    },
    onError: (error) => {
      toast.error(`Failed to delete permission set: ${(error as any).message}`)
    },
  })

  if (isLoading) {
    return <div className="text-center p-8">Loading permission sets...</div>
  }

  return (
    <div className="space-y-4">
      <div className="flex justify-end p-4">
        <Button onClick={() => setIsCreateDialogOpen(true)} size="sm" className="rounded-xl">
          <HugeiconsIcon icon={PlusSignIcon} className="mr-2 size-4" />
          New Permission Set
        </Button>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4">
        {permissionSets.length === 0 ? (
          <p className="text-muted-foreground col-span-full text-center">No permission sets found.</p>
        ) : (
          permissionSets.map((set) => (
            <Card key={set.id} className="shadow-sm">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-lg">
                  <HugeiconsIcon icon={UserGroupIcon} className="size-5 text-primary" />
                  {set.name}
                </CardTitle>
                <CardDescription>{set.description}</CardDescription>
              </CardHeader>
              <CardContent className="flex justify-end gap-2">
                <Button variant="outline" size="sm" onClick={() => setManagePermissionsForSet(set)}>
                  Manage Permissions
                </Button>
                <Button variant="outline" size="icon" onClick={() => setEditPermissionSet(set)}>
                  <HugeiconsIcon icon={Edit04Icon} className="size-4" />
                </Button>
                <Button variant="destructive" size="icon" onClick={() => deleteMutation.mutate(set.id)}>
                  <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                </Button>
              </CardContent>
            </Card>
          ))
        )}
      </div>

      <CreatePermissionSetDialog
        open={isCreateDialogOpen}
        onOpenChange={setIsCreateDialogOpen}
      />

      {editPermissionSet && (
        <EditPermissionSetDialog
          open={!!editPermissionSet}
          onOpenChange={() => setEditPermissionSet(null)}
          permissionSet={editPermissionSet}
        />
      )}

      {managePermissionsForSet && (
        <ManagePermissionSetPermissionsDialog
          open={!!managePermissionsForSet}
          onOpenChange={() => setManagePermissionsForSet(null)}
          permissionSet={managePermissionsForSet}
          allPermissions={allPermissions}
        />
      )}
    </div>
  )
}

