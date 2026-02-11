'use client'

import * as React from 'react'
import { useQuery, useQueryClient } from '@tanstack/react-query'
import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon, Delete02Icon, Loading03Icon, Shield01Icon, SparklesIcon, UserGroupIcon } from '@hugeicons/core-free-icons'
import { toast } from 'sonner'
import { fetchPermissions, unassignPermissionFromUser } from '../api'
import { PermissionManager } from './permission-manager'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

interface UserBulkPermissionsDialogProps {
  userIds: Array<string>
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function UserBulkPermissionsDialog({
  userIds,
  open,
  onOpenChange,
}: UserBulkPermissionsDialogProps) {
  const queryClient = useQueryClient()
  const [selectedAddIds, setSelectedAddIds] = React.useState<Array<number>>([])
  const [selectedRemoveIds, setSelectedRemoveIds] = React.useState<Array<number>>([])
  const [isProcessing, setIsProcessing] = React.useState(false)

  const { data: allPermissions, isLoading: isLoadingAll } = useQuery({
    queryKey: ['permissions'],
    queryFn: fetchPermissions,
    enabled: open,
  })

  const handleApply = async () => {
    setIsProcessing(true)
    try {
      // Add permissions
      for (const userId of userIds) {
        for (const permId of selectedRemoveIds) {
          await unassignPermissionFromUser(userId, permId)
        }
      }
      toast.success(`Security mesh updated for ${userIds.length} users`)
      queryClient.invalidateQueries({ queryKey: ['user-permissions'] })
      onOpenChange(false)
      setSelectedAddIds([])
      setSelectedRemoveIds([])
    } catch (e) {
      toast.error('Partial failure in mesh propagation')
    } finally {
      setIsProcessing(false)
    }
  }

  const toggleAdd = (id: number, enabled: boolean) => {
    if (enabled) {
      setSelectedAddIds([...selectedAddIds, id])
      setSelectedRemoveIds(selectedRemoveIds.filter(rid => rid !== id))
    } else {
      setSelectedAddIds(selectedAddIds.filter(aid => aid !== id))
    }
  }

  const toggleRemove = (id: number, enabled: boolean) => {
    if (enabled) {
      setSelectedRemoveIds([...selectedRemoveIds, id])
      setSelectedAddIds(selectedAddIds.filter(aid => aid !== id))
    } else {
      setSelectedRemoveIds(selectedRemoveIds.filter(rid => rid !== id))
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-5xl rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20 overflow-y-auto max-h-[90vh] custom-scrollbar">
        <DialogHeader className="mb-10 flex flex-row items-center gap-6 text-left space-y-0">
          <div className="flex size-20 shrink-0 items-center justify-center rounded-3xl bg-primary text-primary-foreground shadow-2xl shadow-primary/20">
            <HugeiconsIcon icon={SparklesIcon} className="size-10" />
          </div>
          <div className="space-y-1">
            <DialogTitle className="text-3xl font-black tracking-tight uppercase">
              Mass Mesh Propagation
            </DialogTitle>
            <DialogDescription className="text-base font-medium leading-relaxed opacity-70">
              Synchronizing capabilities across <span className="text-primary font-black">{userIds.length} identities</span>.
              Select overrides to apply or revoke in batch.
            </DialogDescription>
          </div>
        </DialogHeader>

        <Tabs defaultValue="add" className="w-full">
          <TabsList className="grid w-full grid-cols-2 h-16 rounded-2xl bg-muted/30 p-1.5 mb-10 ring-1 ring-border">
            <TabsTrigger value="add" className="rounded-xl font-black uppercase text-[11px] tracking-widest gap-2 data-[state=active]:bg-background data-[state=active]:text-primary data-[state=active]:shadow-xl transition-all">
              <HugeiconsIcon icon={Add01Icon} className="size-4" />
              Grant Capabilities ({selectedAddIds.length})
            </TabsTrigger>
            <TabsTrigger value="remove" className="rounded-xl font-black uppercase text-[11px] tracking-widest gap-2 data-[state=active]:bg-background data-[state=active]:text-destructive data-[state=active]:shadow-xl transition-all">
              <HugeiconsIcon icon={Delete02Icon} className="size-4" />
              Revoke Capabilities ({selectedRemoveIds.length})
            </TabsTrigger>
          </TabsList>

          <div className="min-h-[400px]">
            {isLoadingAll ? (
              <div className="flex flex-col items-center justify-center py-20 gap-4">
                <HugeiconsIcon icon={Loading03Icon} className="size-10 animate-spin text-primary" />
                <p className="text-xs font-black uppercase tracking-widest opacity-40">Mapping permission schema...</p>
              </div>
            ) : (
              <>
                <TabsContent value="add" className="mt-0 focus-visible:outline-none animate-in fade-in slide-in-from-left-4 duration-300">
                  <PermissionManager
                    permissions={allPermissions || []}
                    assignedPermissionIds={selectedAddIds}
                    onToggle={toggleAdd}
                  />
                </TabsContent>

                <TabsContent value="remove" className="mt-0 focus-visible:outline-none animate-in fade-in slide-in-from-right-4 duration-300">
                  <PermissionManager
                    permissions={allPermissions || []}
                    assignedPermissionIds={selectedRemoveIds}
                    onToggle={toggleRemove}
                  />
                </TabsContent>
              </>
            )}
          </div>
        </Tabs>

        <div className="mt-12 flex items-center justify-between border-t pt-10">
          <div className="flex items-center gap-2 px-4 py-2 rounded-xl bg-muted/30 ring-1 ring-border text-[10px] font-bold uppercase tracking-widest opacity-60">
            <HugeiconsIcon icon={UserGroupIcon} className="size-3" />
            Targets: {userIds.length}
          </div>
          <div className="flex gap-3">
            <Button
              variant="ghost"
              onClick={() => onOpenChange(false)}
              className="h-14 px-8 rounded-2xl font-black uppercase tracking-widest text-xs"
            >
              Cancel
            </Button>
            <Button
              disabled={isProcessing || (selectedAddIds.length === 0 && selectedRemoveIds.length === 0)}
              onClick={handleApply}
              className="h-14 px-12 rounded-2xl font-black uppercase tracking-widest text-xs shadow-2xl shadow-primary/20"
            >
              {isProcessing ? (
                <HugeiconsIcon icon={Loading03Icon} className="mr-2 h-4 w-4 animate-spin" />
              ) : (
                <HugeiconsIcon icon={Shield01Icon} className="mr-2 size-4" />
              )}
              Propagate Mesh Overrides
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  )
}
