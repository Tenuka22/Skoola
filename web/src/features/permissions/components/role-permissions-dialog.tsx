'use client'

import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Loading03Icon,
  SecurityIcon,
  Shield01Icon,
} from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import { PermissionManager } from './permission-manager'
import type { PermissionSet } from '../types'
import type { Permission } from '@/lib/api/types.gen'

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import {
  getPermissions9C8839E73223Cb930255A2882A4B0Db4Options,
  getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0Options,
  getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey,
  postPermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation,
  deletePermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'

interface RolePermissionsDialogProps {
  permissionSet: PermissionSet | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function RolePermissionsDialog({
  permissionSet,
  open,
  onOpenChange,
}: RolePermissionsDialogProps) {
  const queryClient = useQueryClient()

  const { data: permissionsResponse, isLoading: isLoadingAll } = useQuery({
    ...getPermissions9C8839E73223Cb930255A2882A4B0Db4Options({
      client: authClient,
      query: { limit: 1000 }, // Fetch all permissions
    }),
    enabled: open,
  })

  const permissions = permissionsResponse?.data || []

  const { data: assignedPermissions, isLoading: isLoadingAssigned } = useQuery({
    ...getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0Options({
      client: authClient,
      path: { permission_set_id: permissionSet?.id || '' },
    }),
    enabled: !!permissionSet && open,
  })

  const assignedIds = React.useMemo(() => {
    if (Array.isArray(assignedPermissions)) {
      return (assignedPermissions as Array<Permission>).map((p) => p.id)
    }
    return []
  }, [assignedPermissions])

  const assignMutation = useMutation({
    ...postPermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      if (permissionSet) {
        queryClient.invalidateQueries({
          queryKey: getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey({
            path: { permission_set_id: permissionSet.id },
          }),
        })
      }
      toast.success('Security policy synchronized')
    },
    onError: () => {
      toast.error('Failed to update mesh parameters')
    },
  })

  const unassignMutation = useMutation({
    ...deletePermissionSetsE88249A62Acbe1Edff95479F9E23B8F3Mutation({
      client: authClient,
    }),
    onSuccess: () => {
      if (permissionSet) {
        queryClient.invalidateQueries({
          queryKey: getPermissionSets3134991Ad907142C0B9D153Ceaf59Bc0QueryKey({
            path: { permission_set_id: permissionSet.id },
          }),
        })
      }
      toast.success('Security policy synchronized')
    },
    onError: () => {
      toast.error('Failed to update mesh parameters')
    },
  })

  const handleToggle = (permissionId: number, isEnabled: boolean) => {
    if (!permissionSet) return
    if (isEnabled) {
      assignMutation.mutate({
        path: {
          permission_set_id: permissionSet.id,
          permission_id: permissionId,
        },
      })
    } else {
      unassignMutation.mutate({
        path: {
          permission_set_id: permissionSet.id,
          permission_id: permissionId,
        },
      })
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-5xl rounded-[2.5rem] border-none p-12 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20 overflow-y-auto max-h-[90vh] custom-scrollbar">
        <DialogHeader className="mb-12 flex flex-row items-center gap-8 text-left space-y-0 border-b pb-12 bg-muted/5 -mx-12 -mt-12 p-12">
          <div className="flex size-24 shrink-0 items-center justify-center rounded-[2rem] bg-primary text-primary-foreground shadow-2xl shadow-primary/20 ring-4 ring-primary/10">
            <HugeiconsIcon icon={SecurityIcon} className="size-12" />
          </div>
          <div className="space-y-1.5">
            <div className="flex items-center gap-3">
              <DialogTitle className="text-4xl font-black tracking-tighter uppercase">
                {permissionSet?.name}
              </DialogTitle>
              <div className="px-3 py-1 rounded-full bg-primary/10 text-primary text-[10px] font-black uppercase tracking-widest ring-1 ring-primary/20">
                Core Policy
              </div>
            </div>
            <DialogDescription className="text-lg font-medium leading-relaxed opacity-70">
              Configure baseline capabilities for the{' '}
              <span className="text-foreground font-bold">RBAC Mesh</span>.
              Changes propagate to all identities inheriting this role.
            </DialogDescription>
          </div>
        </DialogHeader>

        <div className="min-h-[400px]">
          {isLoadingAssigned || isLoadingAll ? (
            <div className="flex flex-col items-center justify-center py-24 gap-4">
              <HugeiconsIcon
                icon={Loading03Icon}
                className="size-12 animate-spin text-primary"
              />
              <p className="text-xs font-black uppercase tracking-widest opacity-40 italic">
                Querying capability matrix...
              </p>
            </div>
          ) : (
            <PermissionManager
              permissions={permissions || []}
              assignedPermissionIds={assignedIds}
              onToggle={handleToggle}
            />
          )}
        </div>

        <div className="mt-12 flex items-center justify-between border-t pt-10">
          <div className="flex items-center gap-2 px-5 py-2.5 rounded-2xl bg-muted/30 ring-1 ring-border text-[10px] font-black uppercase tracking-widest opacity-60">
            <HugeiconsIcon icon={Shield01Icon} className="size-3.5" />
            Policy Integrity Verified
          </div>
          <Button
            onClick={() => onOpenChange(false)}
            className="h-14 min-w-[220px] rounded-2xl font-black uppercase tracking-widest text-xs shadow-2xl shadow-primary/20 transition-all hover:scale-[1.02] active:scale-[0.98]"
          >
            Finalize Mesh Policy
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}
