'use client'

import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  Edit04Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { usePermissionsStore } from '../store'
import type { PermissionSet } from '../types'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'

interface PermissionSetsListProps {
  permissionSets: Array<PermissionSet>
  isLoading: boolean
  allPermissions: any // Keeping for now
}

export function PermissionSetsList({
  permissionSets,
  isLoading,
}: PermissionSetsListProps) {
  const {
    setPermissionSetToEdit,
    setPermissionSetToDelete,
    setPermissionSetToManage,
  } = usePermissionsStore()

  if (isLoading) {
    return <div className="text-center p-8">Loading permission sets...</div>
  }

  return (
    <div className="space-y-4">
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 py-4">
        {permissionSets.length === 0 ? (
          <p className="text-muted-foreground col-span-full text-center py-10">
            No permission sets found.
          </p>
        ) : (
          permissionSets.map((set) => (
            <Card key={set.id} className="shadow-sm">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-lg font-semibold">
                  <HugeiconsIcon
                    icon={UserGroupIcon}
                    className="size-5 text-primary"
                  />
                  {set.name}
                </CardTitle>
                <CardDescription>{set.description}</CardDescription>
              </CardHeader>
              <CardContent className="flex justify-end gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setPermissionSetToManage(set)}
                >
                  Permissions
                </Button>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setPermissionSetToEdit(set)}
                >
                  <HugeiconsIcon icon={Edit04Icon} className="size-4" />
                </Button>
                <Button
                  variant="destructive"
                  size="icon"
                  onClick={() => setPermissionSetToDelete(set.id)}
                >
                  <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                </Button>
              </CardContent>
            </Card>
          ))
        )}
      </div>
    </div>
  )
}
