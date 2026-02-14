'use client'

import * as React from 'react'
import { toast } from 'sonner'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import type { Permission } from '@/lib/api/types.gen'
import type { PermissionSet } from '../types'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Separator } from '@/components/ui/separator'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import {
  getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0Options,
  getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey,
  postPermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation,
  deletePermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

interface ManagePermissionSetPermissionsDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  permissionSet: PermissionSet
  allPermissions: Array<Permission>
}

export function ManagePermissionSetPermissionsDialog({
  open,
  onOpenChange,
  permissionSet,
  allPermissions,
}: ManagePermissionSetPermissionsDialogProps) {
  const queryClient = useQueryClient()
  const [selectedPermissions, setSelectedPermissions] = React.useState<
    Set<number>
  >(new Set())

  // Fetch permissions currently assigned to this permission set
  const {
    data: currentPermissionsInSet,
    isLoading: isLoadingCurrentPermissions,
  } = useQuery({
    ...getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0Options({
      client: authClient,
      path: { permission_set_id: permissionSet.id },
    }),
    enabled: open && !!permissionSet.id,
  })

  React.useEffect(() => {
    if (currentPermissionsInSet && Array.isArray(currentPermissionsInSet)) {
      setSelectedPermissions(
        new Set(
          (currentPermissionsInSet as Array<Permission>).map((p) => p.id),
        ),
      )
    }
  }, [currentPermissionsInSet])

  const assignMutation = useMutation({
    ...postPermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey({
          path: { permission_set_id: permissionSet.id },
        }),
      })
      toast.success('Permission assigned to set.')
    },
    onError: (error) => {
      toast.error(`Failed to assign permission: ${error.message}`)
    },
  })

  const unassignMutation = useMutation({
    ...deletePermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey({
          path: { permission_set_id: permissionSet.id },
        }),
      })
      toast.success('Permission unassigned from set.')
    },
    onError: (error) => {
      toast.error(`Failed to unassign permission: ${error.message}`)
    },
  })

  const handlePermissionToggle = (permissionId: number, isChecked: boolean) => {
    if (isChecked) {
      setSelectedPermissions((prev) => new Set(prev).add(permissionId))
      assignMutation.mutate({
        path: {
          permission_set_id: permissionSet.id,
          permission_id: permissionId,
        },
      })
    } else {
      setSelectedPermissions((prev) => {
        const newSet = new Set(prev)
        newSet.delete(permissionId)
        return newSet
      })
      unassignMutation.mutate({
        path: {
          permission_set_id: permissionSet.id,
          permission_id: permissionId,
        },
      })
    }
  }

  const groupedPermissions = React.useMemo(() => {
    const groups: Record<string, Array<Permission>> = {}
    allPermissions.forEach((p) => {
      const category = getPermissionCategory(p.name)
      if (!groups[category]) {
        groups[category] = []
      }
      groups[category].push(p)
    })
    return groups
  }, [allPermissions])

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle>Manage Permissions for {permissionSet.name}</DialogTitle>
        </DialogHeader>
        <ScrollArea className="h-[400px] pr-4">
          {isLoadingCurrentPermissions ? (
            <div className="text-center py-4">Loading permissions...</div>
          ) : (
            Object.entries(groupedPermissions).map(([category, permissions]) => (
              <div key={category} className="mb-4">
                <h3 className="font-semibold text-lg mb-2">{category}</h3>
                <Separator className="mb-3" />
                <div className="grid grid-cols-2 gap-2">
                  {permissions.map((p: Permission) => (
                    <div key={p.id} className="flex items-center space-x-2">
                      <Checkbox
                        id={`perm-${p.id}`}
                        checked={selectedPermissions.has(p.id)}
                        onCheckedChange={(checked) =>
                          handlePermissionToggle(p.id, checked as boolean)
                        }
                      />
                      <Label htmlFor={`perm-${p.id}`}>{p.name}</Label>
                    </div>
                  ))}
                </div>
              </div>
            ))
          )}
        </ScrollArea>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Close
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}

// Helper to get category for grouping, consistent with permissions-table-columns.tsx
const getPermissionCategory = (permissionName: string) => {
  if (permissionName.startsWith('User')) return 'User Permissions'
  if (permissionName.startsWith('Role')) return 'Role Permissions'
  if (permissionName.startsWith('PermissionSet'))
    return 'Permission Set Management'
  if (permissionName.startsWith('Permission')) return 'Permission Management'
  if (permissionName.startsWith('Staff')) return 'Staff Permissions'
  if (permissionName.startsWith('Student')) return 'Student Permissions'
  if (permissionName.startsWith('AcademicYear'))
    return 'Academic Year Permissions'
  if (permissionName.startsWith('Term')) return 'Term Permissions'
  if (permissionName.startsWith('GradeLevel')) return 'Grade Level Permissions'
  if (permissionName.startsWith('Class')) return 'Class Permissions'
  if (permissionName.startsWith('Subject')) return 'Subject Permissions'
  if (permissionName.startsWith('ClassSubjectTeacher'))
    return 'Class Subject Teacher Permissions'
  if (permissionName.startsWith('Timetable')) return 'Timetable Permissions'
  if (permissionName.startsWith('ExamType')) return 'Exam Type Permissions'
  if (permissionName.startsWith('ExamSubject'))
    return 'Exam Subject Permissions'
  if (permissionName.startsWith('Exam')) return 'Exam Permissions'
  if (permissionName.startsWith('GradingScheme'))
    return 'Grading Scheme Permissions'
  if (permissionName.startsWith('GradingCriterion'))
    return 'Grading Criterion Permissions'
  if (permissionName.startsWith('Library')) return 'Library Permissions'
  return 'Other Permissions'
}
