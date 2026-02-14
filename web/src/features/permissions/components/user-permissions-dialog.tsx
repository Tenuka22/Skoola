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
import { PermissionManager } from './permission-manager'
import type { UserResponse, Permission } from '@/lib/api/types.gen'
import { Dialog, DialogContent } from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '@/components/ui/accordion'
import { Checkbox } from '@/components/ui/checkbox'
import {
  getPermissions9C8839E73223Cb930255A2882A4B0Db4Options,
  getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44Options,
  getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92cOptions,
  getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92cQueryKey,
  postUsers069Bc83C67Aeddbeed75C9632Ba56B82Mutation,
  deleteUsers069Bc83C67Aeddbeed75C9632Ba56B82Mutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

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

  const { data: allPermissionsResponse } = useQuery({
    ...getPermissions9C8839E73223Cb930255A2882A4B0Db4Options({
      client: authClient,
      query: { limit: 1000 },
    }),
    enabled: open,
  })
  const allPermissions = allPermissionsResponse?.data || []

  const { data: allPermissionSets } = useQuery({
    ...getPermissionSets2Bd49615D055600Ba22C7Cf2Eb651B44Options({
      client: authClient,
    }),
    enabled: open,
  })

  // Placeholder for user permission sets - currently not supported by API
  const userPermissionSets: any[] = []
  // const { data: userPermissionSets } = useQuery({
  //   queryKey: ['user-permission-sets', user?.id],
  //   queryFn: () => (user ? getStaffPermissionSets(user.id) : []),
  //   enabled: !!user && open,
  // })

  const { data: directPermissions, isLoading: isLoadingDirect } = useQuery({
    ...getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92cOptions({
      client: authClient,
      path: { user_id: user?.id || '' },
    }),
    enabled: !!user && open,
  })

  const directIds = React.useMemo(() => {
    if (Array.isArray(directPermissions)) {
      return (directPermissions as Array<Permission>).map((p) => p.id)
    }
    return []
  }, [directPermissions])

  // const userPermissionSetIds = React.useMemo(
  //   () => userPermissionSets?.map((ps) => ps.id) || [],
  //   [userPermissionSets],
  // )

  const assignMutation = useMutation({
    ...postUsers069Bc83C67Aeddbeed75C9632Ba56B82Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      if (user) {
        queryClient.invalidateQueries({
          queryKey: getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92cQueryKey({
            path: { user_id: user.id },
          }),
        })
      }
      toast.success('User permissions updated')
    },
    onError: (error) => {
      toast.error(`Failed to update user permissions: ${error.message}`)
    },
  })

  const unassignMutation = useMutation({
    ...deleteUsers069Bc83C67Aeddbeed75C9632Ba56B82Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      if (user) {
        queryClient.invalidateQueries({
          queryKey: getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92cQueryKey({
            path: { user_id: user.id },
          }),
        })
      }
      toast.success('User permissions updated')
    },
    onError: (error) => {
      toast.error(`Failed to update user permissions: ${error.message}`)
    },
  })

  const handleToggle = (permissionId: number, isEnabled: boolean) => {
    if (!user) return
    if (isEnabled) {
      assignMutation.mutate({
        path: { user_id: user.id, permission_id: permissionId },
      })
    } else {
      unassignMutation.mutate({
        path: { user_id: user.id, permission_id: permissionId },
      })
    }
  }

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
                {(Array.isArray(allPermissionSets)
                  ? allPermissionSets
                  : []
                ).map((permissionSet: any) => (
                  <div
                    key={permissionSet.id}
                    className="flex items-center justify-between p-3 rounded-xl bg-background/50 ring-1 ring-border transition-all hover:ring-primary/30 cursor-not-allowed"
                  >
                    <div className="flex items-center gap-3">
                      <Checkbox
                        checked={false} // userPermissionSetIds.includes(permissionSet.id)
                        disabled={true}
                      />
                      <span className="text-[10px] font-bold">
                        {permissionSet.name}
                      </span>
                    </div>
                    {/* {userPermissionSetIds.includes(permissionSet.id) && (
                      <div className="size-1.5 rounded-full bg-primary animate-pulse" />
                    )} */}
                  </div>
                ))}
                <p className="text-[9px] text-muted-foreground text-center pt-2">
                  User roles management coming soon.
                </p>
              </AccordionContent>
            </AccordionItem>

            {/* Direct Overrides section */}
            <AccordionItem value="current" className="border-none">
              <AccordionTrigger className="hover:no-underline py-3 px-4 rounded-xl bg-muted/50 font-black text-[10px] uppercase tracking-widest">
                Active Overrides ({directPermissions?.length || 0})
              </AccordionTrigger>
              <AccordionContent className="pt-4 px-1 space-y-2 max-h-[40vh] overflow-y-auto custom-scrollbar">
                {(Array.isArray(directPermissions)
                  ? (directPermissions as Permission[])
                  : []
                ).map((permission) => (
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
                onToggle={handleToggle}
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
