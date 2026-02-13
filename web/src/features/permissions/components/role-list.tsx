'use client'

import * as React from 'react'
import { useQuery } from '@tanstack/react-query'
// import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  HierarchyIcon,
  Loading03Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
// import { toast } from 'sonner'
// import { createPermissionSet, deletePermissionSet, fetchPermissionSets } from '../api'
import { RoleCard } from './role-card'
import { RolePermissionsDialog } from './role-permissions-dialog'
import type { PermissionSet } from '../types'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

export function RoleList() {
  const [selectedPermissionSet, setSelectedPermissionSet] =
    React.useState<PermissionSet | null>(null)
  const [isPermissionsOpen, setIsPermissionsOpen] = React.useState(false)
  const [isCreateOpen, setIsCreateOpen] = React.useState(false)
  const [newRoleName, setNewRoleName] = React.useState('')
  const [newRoleDescription, setNewRoleDescription] = React.useState('')

  //   const queryClient = useQueryClient()

  const { data: permissionSets = [], isLoading } = useQuery({
    queryKey: ['permission-sets'],
    queryFn: async () => [], // Mocking as fetchPermissionSets is commented out
  })

  //   const createMutation = useMutation({
  //     mutationFn: () => createPermissionSet(newRoleName, newRoleDescription),
  //     onSuccess: () => {
  //       queryClient.invalidateQueries({ queryKey: ['permission-sets'] })
  //       setIsCreateOpen(false)
  //       setNewRoleName('')
  //       setNewRoleDescription('')
  //       toast.success('New permission set integrated into mesh')
  //     },
  //     onError: () => toast.error('Failed to create permission set'),
  //   })

  //   const deleteMutation = useMutation({
  //     mutationFn: (id: string) => deletePermissionSet(id),
  //     onSuccess: () => {
  //       queryClient.invalidateQueries({ queryKey: ['permission-sets'] })
  //       toast.success('Permission set purged from infrastructure')
  //     },
  //     onError: () => toast.error('Failed to delete permission set'),
  //   })

  const handleManage = (permissionSet: PermissionSet) => {
    setSelectedPermissionSet(permissionSet)
    setIsPermissionsOpen(true)
  }

  const renderPermissionSetList = () => {
    if (!permissionSets || permissionSets.length === 0) return null

    return (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        {permissionSets.map((permissionSet: any) => (
          <div key={permissionSet.id} className="space-y-6">
            <RoleCard
              permissionSet={permissionSet}
              userCount={0}
              permissionCount={0}
              onManage={handleManage}
              onDelete={() => {
                /* deleteMutation.mutate(permissionSet.id) */
              }}
            />
          </div>
        ))}
      </div>
    )
  }

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center py-24 gap-4">
        <HugeiconsIcon
          icon={Loading03Icon}
          className="size-12 animate-spin text-primary"
        />
        <p className="text-xs font-black uppercase tracking-widest opacity-50 italic">
          Initializing Security Mesh...
        </p>
      </div>
    )
  }

  return (
    <div className="space-y-12">
      <div className="flex items-center justify-between bg-muted/20 p-6 rounded-[2rem] ring-1 ring-border shadow-inner">
        <div className="flex items-center gap-4">
          <div className="p-4 rounded-2xl bg-primary text-primary-foreground shadow-2xl shadow-primary/20">
            <HugeiconsIcon icon={UserGroupIcon} className="size-8" />
          </div>
          <div>
            <h2 className="text-3xl font-black tracking-tighter uppercase">
              Permission Set Management
            </h2>
            <p className="text-sm font-medium opacity-50">
              Define and manage granular access control policies.
            </p>
          </div>
        </div>

        <Dialog open={isCreateOpen} onOpenChange={setIsCreateOpen}>
          <DialogTrigger
            render={
              <Button className="h-14 px-8 rounded-2xl font-black uppercase tracking-widest shadow-2xl shadow-primary/20 transition-all hover:scale-[1.02] active:scale-[0.98]">
                <HugeiconsIcon icon={Add01Icon} className="mr-2 size-5" />
                New Identity Role
              </Button>
            }
          />
          <DialogContent className="rounded-[2rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
            <DialogHeader>
              <div className="mx-auto mb-4 flex size-16 items-center justify-center rounded-2xl bg-primary/10 text-primary">
                <HugeiconsIcon icon={HierarchyIcon} className="size-8" />
              </div>
              <DialogTitle className="text-center text-2xl font-black uppercase tracking-tight">
                Create Permission Set
              </DialogTitle>
              <DialogDescription className="text-center">
                Define a new set of permissions within the access control
                infrastructure.
              </DialogDescription>
            </DialogHeader>
            <div className="space-y-6 py-6">
              <div className="space-y-2">
                <Label className="text-[10px] font-black uppercase tracking-widest opacity-50">
                  Permission Set Name
                </Label>
                <Input
                  value={newRoleName}
                  onChange={(e) => setNewRoleName(e.target.value)}
                  placeholder="e.g. Department Head"
                  className="h-12 rounded-xl bg-muted/30 border-none px-4 font-bold"
                />
              </div>
              <div className="space-y-2">
                <Label className="text-[10px] font-black uppercase tracking-widest opacity-50">
                  Description
                </Label>
                <textarea
                  value={newRoleDescription}
                  onChange={(e) => setNewRoleDescription(e.target.value)}
                  placeholder="Briefly describe the purpose of this permission set."
                  className="min-h-[80px] w-full rounded-xl bg-muted/30 border-none px-4 py-3 font-bold text-sm focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent resize-y"
                />
              </div>
            </div>
            <DialogFooter>
              <Button
                variant="ghost"
                onClick={() => setIsCreateOpen(false)}
                className="h-12 rounded-xl font-black uppercase tracking-widest text-[10px]"
              >
                Abort
              </Button>
              <Button
                //   disabled={!newRoleName || !newRoleDescription || createMutation.isPending}
                //   onClick={() => createMutation.mutate()}
                className="h-12 px-8 rounded-xl font-black uppercase tracking-widest text-[10px] shadow-xl shadow-primary/20"
              >
                Create Permission Set
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>

      <div className="px-2">{renderPermissionSetList()}</div>

      <RolePermissionsDialog
        permissionSet={selectedPermissionSet}
        open={isPermissionsOpen}
        onOpenChange={setIsPermissionsOpen}
      />
    </div>
  )
}
