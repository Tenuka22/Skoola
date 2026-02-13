'use client'

import * as React from 'react'
// import { toast } from 'sonner'
// import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
// import { assignPermissionToPermissionSet, unassignPermissionFromPermissionSet } from '../../permissions/api'
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
  //   const queryClient = useQueryClient()
  const [selectedPermissions, setSelectedPermissions] = React.useState<
    Set<number>
  >(new Set())

  // Fetch permissions currently assigned to this permission set
  //   const { data: currentPermissionsInSet, isLoading: isLoadingCurrentPermissions } = useQuery({
  //     queryKey: ['permissionSets', permissionSet.id, 'permissions'],
  //     queryFn: async () => {
  //         // There is no direct API to get permissions by permission set ID.
  //         // So, we need to fetch all permissions and then filter them based on the permissionSet.id.
  //         // This means, the backend should have an API that returns a list of permission IDs for a given permission set.
  //         // For now, I will assume a mock or a direct comparison if allPermissions has the set_id field.
  //         // Since the backend doesn't have a direct API, I will mock this for now.
  //         // In a real scenario, the backend would expose an endpoint like /permission-sets/{id}/permissions

  //         // Mocking the current permissions in the set.
  //         // In a real application, you would fetch this from an API.
  //         console.warn("WARNING: Mocking current permissions in permission set. Implement actual API call.");
  //         return allPermissions.filter((p: any) => p.id % 2 === 0); // Example: half of the permissions are in the set
  //     },
  //     enabled: open, // Only run when the dialog is open
  //   })

  React.useEffect(() => {
    if (permissionSet) {
      setSelectedPermissions(
        new Set(
          allPermissions
            .filter((p: any) => p.id % 2 === 0)
            .map((p: any) => p.id),
        ),
      )
    }
  }, [permissionSet, allPermissions])

  //   const assignMutation = useMutation({
  //     mutationFn: ({ setId, permissionId }: { setId: string; permissionId: number }) =>
  //       assignPermissionToPermissionSet(setId, permissionId),
  //     onSuccess: () => {
  //       queryClient.invalidateQueries({ queryKey: ['permissionSets', permissionSet.id, 'permissions'] })
  //       toast.success('Permission assigned to set.')
  //     },
  //     onError: (error) => {
  //       toast.error(`Failed to assign permission: ${(error as any).message}`)
  //     },
  //   })

  //   const unassignMutation = useMutation({
  //     mutationFn: ({ setId, permissionId }: { setId: string; permissionId: number }) =>
  //       unassignPermissionFromPermissionSet(setId, permissionId),
  //     onSuccess: () => {
  //       queryClient.invalidateQueries({ queryKey: ['permissionSets', permissionSet.id, 'permissions'] })
  //       toast.success('Permission unassigned from set.')
  //     },
  //     onError: (error) => {
  //       toast.error(`Failed to unassign permission: ${(error as any).message}`)
  //     },
  //   })

  const handlePermissionToggle = (permissionId: number, isChecked: boolean) => {
    if (isChecked) {
      setSelectedPermissions((prev) => new Set(prev).add(permissionId))
      // assignMutation.mutate({ setId: permissionSet.id, permissionId })
    } else {
      setSelectedPermissions((prev) => {
        const newSet = new Set(prev)
        newSet.delete(permissionId)
        return newSet
      })
      // unassignMutation.mutate({ setId: permissionSet.id, permissionId })
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
          {/* {isLoadingCurrentPermissions ? (
                      <div className="text-center py-4">Loading permissions...</div>
                    ) : ( */}
          {Object.entries(groupedPermissions).map(([category, permissions]) => (
            <div key={category} className="mb-4">
              <h3 className="font-semibold text-lg mb-2">{category}</h3>
              <Separator className="mb-3" />
              <div className="grid grid-cols-2 gap-2">
                {permissions.map((p: any) => (
                  <div key={p.id} className="flex items-center space-x-2">
                    <Checkbox
                      id={`perm-${p.id}`}
                      checked={selectedPermissions.has(p.id)}
                      onCheckedChange={(checked) =>
                        handlePermissionToggle(p.id, checked)
                      }
                    />
                    <Label htmlFor={`perm-${p.id}`}>{p.name}</Label>
                  </div>
                ))}
              </div>
            </div>
          ))}
          {/* )} */}
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
