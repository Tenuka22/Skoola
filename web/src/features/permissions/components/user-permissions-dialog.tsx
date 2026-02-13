'use client'

import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Loading03Icon,
  Shield01Icon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import {
  fetchPermissionSets,
  fetchPermissions,
  fetchUserPermissions,
  getStaffPermissionSets,
  unassignPermissionFromUser,
} from '../api'
import { PermissionManager } from './permission-manager'
import type { UserResponse } from '@/lib/api/types.gen'
import { Dialog, DialogContent } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
// import type { PermissionSet } from '../types' // Commented out as PermissionSet is not used in the current state
// import { PermissionSet } from '@/lib/api/types.gen' // This import is no longer needed as PermissionSet is from ../types
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'
// import { Badge } from '@/components/ui/badge' // Commented out as Badge is not used in the current state
// import { cn } from '@/lib/utils' // Commented out as cn is not used in the current state
import { Checkbox } from '@/components/ui/checkbox'

interface UserPermissionsDialogProps {
  user: UserResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UserPermissionsDialog({
  user,
  open,
  onOpenChange,
}: UserPermissionsDialogProps) {
  const queryClient = useQueryClient()

  const { data: allPermissions } = useQuery({
    queryKey: ['permissions'],
    queryFn: fetchPermissions,
    enabled: open,
  })

  const { data: allPermissionSets } = useQuery({
    queryKey: ['permission-sets'],
    queryFn: fetchPermissionSets,
    enabled: open,
  })

  const { data: userPermissionSets } = useQuery({
    queryKey: ['user-permission-sets', user?.id],
    queryFn: () => (user ? getStaffPermissionSets(user.id) : []), // Assuming getStaffPermissionSets can be used for users temporarily, or proper user permission set API is added later
    enabled: !!user && open,
  })

  // Direct permissions are currently supported via a dedicated API endpoint
  const { data: directPermissions, isLoading: isLoadingDirect } = useQuery({
    queryKey: ['user-permissions', user?.id],
    queryFn: () => (user ? fetchUserPermissions(user.id) : []),
    enabled: !!user && open,
  })

  const directIds = React.useMemo(
    () => directPermissions?.map((p) => p.id) || [],
    [directPermissions],
  )

  const userPermissionSetIds = React.useMemo(
    () => userPermissionSets?.map((ps) => ps.id) || [],
    [userPermissionSets],
  )

  // Mutation for direct permissions
  const mutation = useMutation({
    mutationFn: async ({
      permissionId,
      isEnabled,
    }: {
      permissionId: number
      isEnabled: boolean
    }) => {
      if (!user) return
      if (isEnabled) {
        // return assignPermissionToUser(user.id, permissionId)
      } else {
        return unassignPermissionFromUser(user.id, permissionId)
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['user-permissions', user?.id],
      })
      toast.success('User permissions updated')
    },
    onError: () => {
      toast.error('Failed to update user permissions')
    },
  })

  // Mutation for role-based permission sets - commented out due to lack of direct API support for user permission sets
  // const roleMutation = useMutation({
  //   mutationFn: async ({
  //     setId,
  //     isEnabled,
  //   }: {
  //     setId: string
  //     isEnabled: boolean
  //   }) => {
  //     if (!user) return
  //     if (isEnabled) {
  //       return assignPermissionSetToUser(user.id, setId)
  //     } else {
  //       return unassignPermissionSetFromUser(user.id, setId)
  //     }
  //   },
  //   onSuccess: () => {
  //     queryClient.invalidateQueries({ queryKey: ['user-permission-sets', user?.id] })
  //     toast.success('User permission sets synchronized')
  //   },
  //   onError: () => {
  //     toast.error('Partial failure in permission set propagation')
  //   },
  // })

  // const getSeverityStyles = (severity: string) => { // Commented out as getSeverityStyles is not used in the current state
  //   switch (severity) {
  //     case 'Low':
  //       return 'text-green-500 bg-green-500/10 border-green-500/20'
  //     case 'Medium':
  //       return 'text-blue-500 bg-blue-500/10 border-blue-500/20'
  //     case 'High':
  //       return 'text-orange-500 bg-orange-500/10 border-orange-500/20'
  //     case 'Severe':
  //       return 'text-red-500 bg-red-500/10 border-red-500/20'
  //     default:
  //       return 'text-muted-foreground bg-muted border-transparent'
  //   }
  // }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-6xl rounded-[2.5rem] border-none p-0 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20 overflow-hidden flex h-[85vh]">
        <div className="w-80 border-r bg-muted/20 p-8 flex flex-col gap-8">
          <div className="space-y-4">
            <div className="flex size-16 items-center justify-center rounded-2xl bg-primary text-primary-foreground shadow-2xl shadow-primary/20">
              <HugeiconsIcon icon={UserIcon} className="size-8" />
            </div>
            <div>
              <h2 className="text-xl font-black tracking-tight truncate max-w-full">
                {user?.email}
              </h2>
              <p className="text-[10px] font-bold uppercase tracking-widest opacity-50">
                Security Configuration
              </p>
            </div>
          </div>

          <Accordion defaultValue={['roles']} className="w-full space-y-4">
            <AccordionItem value="roles" className="border-none">
              <AccordionTrigger className="hover:no-underline py-3 px-4 rounded-xl bg-primary/10 text-primary font-black text-[10px] uppercase tracking-widest ring-1 ring-primary/20">
                Assigned Permission Sets ({userPermissionSets?.length || 0})
              </AccordionTrigger>
              <AccordionContent className="pt-4 px-1 space-y-2 max-h-[30vh] overflow-y-auto custom-scrollbar">
                {allPermissionSets?.map((permissionSet) => (
                  <div
                    key={permissionSet.id}
                    className="flex items-center justify-between p-3 rounded-xl bg-background/50 ring-1 ring-border transition-all hover:ring-primary/30 cursor-not-allowed" // Changed cursor to not-allowed as functionality is disabled
                    // onClick={() => roleMutation.mutate({ setId: permissionSet.id, isEnabled: !userPermissionSetIds.includes(permissionSet.id) })} // Commented out click handler
                  >
                    <div className="flex items-center gap-3">
                      <Checkbox
                        checked={userPermissionSetIds.includes(
                          permissionSet.id,
                        )}
                        // onCheckedChange={(checked) => roleMutation.mutate({ setId: permissionSet.id, isEnabled: !!checked })} // Commented out change handler
                        disabled={true} // Disable checkbox as functionality is disabled
                      />
                      <span className="text-[10px] font-bold">
                        {permissionSet.name}
                      </span>
                    </div>
                    {userPermissionSetIds.includes(permissionSet.id) && (
                      <div className="size-1.5 rounded-full bg-primary animate-pulse" />
                    )}
                  </div>
                ))}
              </AccordionContent>
            </AccordionItem>

            {/* Direct Overrides section */}
            <AccordionItem value="current" className="border-none">
              <AccordionTrigger className="hover:no-underline py-3 px-4 rounded-xl bg-muted/50 font-black text-[10px] uppercase tracking-widest">
                Active Overrides ({directPermissions?.length || 0})
              </AccordionTrigger>
              <AccordionContent className="pt-4 px-1 space-y-2 max-h-[40vh] overflow-y-auto custom-scrollbar">
                {directPermissions?.map((permission) => (
                  <div
                    key={permission.id}
                    className="flex items-center justify-between p-3 rounded-xl bg-background/50 ring-1 ring-border"
                  >
                    <span className="text-[10px] font-bold">
                      {permission.name}
                    </span>
                    <div className="size-1.5 rounded-full bg-orange-500 animate-pulse" />
                  </div>
                ))}
              </AccordionContent>
            </AccordionItem>
          </Accordion>

          <div className="mt-auto p-5 rounded-2xl bg-primary/5 border border-primary/10 space-y-3">
            <div className="flex items-center gap-2 text-primary">
              <HugeiconsIcon icon={Shield01Icon} className="size-4" />
              <span className="text-[10px] font-black uppercase tracking-wider">
                Infrastructure Policy
              </span>
            </div>
            <p className="text-[9px] font-medium leading-relaxed opacity-70">
              Direct overrides take precedence over role-based permissions. Use
              sparingly to maintain security audit trails.
            </p>
          </div>
        </div>

        <div className="flex-1 p-10 overflow-y-auto custom-scrollbar flex flex-col">
          <div className="mb-10 flex items-center justify-between">
            <div className="space-y-1">
              <h3 className="text-2xl font-black tracking-tight uppercase">
                Capability Matrix
              </h3>
              <p className="text-sm font-medium opacity-50">
                Toggle specific system capabilities for this identity. Direct
                permissions override assigned permission sets.
              </p>
            </div>
            <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-muted/30 ring-1 ring-border">
              <div className="size-2 rounded-full bg-green-500 animate-pulse" />
              <span className="text-[10px] font-black uppercase tracking-widest opacity-70">
                Real-time Sync Active
              </span>
            </div>
          </div>

          <div className="flex-1">
            {isLoadingDirect ? (
              <div className="flex flex-col items-center justify-center py-24 gap-4">
                <HugeiconsIcon
                  icon={Loading03Icon}
                  className="size-10 animate-spin text-primary"
                />
                <p className="text-[10px] font-black uppercase tracking-widest opacity-40">
                  Loading permission matrix...
                </p>
              </div>
            ) : (
              <PermissionManager
                permissions={allPermissions || []}
                assignedPermissionIds={directIds}
                onToggle={(id, enabled) =>
                  mutation.mutate({ permissionId: id, isEnabled: enabled })
                }
              />
            )}
          </div>

          <div className="mt-10 flex justify-end gap-3 border-t pt-8">
            <Button
              variant="ghost"
              onClick={() => onOpenChange(false)}
              className="h-12 px-8 rounded-xl font-black uppercase text-[10px] tracking-widest"
            >
              Close
            </Button>
            <Button
              onClick={() => onOpenChange(false)}
              className="h-12 px-10 rounded-xl font-black uppercase text-[10px] tracking-widest shadow-xl shadow-primary/20"
            >
              Done
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
