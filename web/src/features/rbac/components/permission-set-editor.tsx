import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Edit01Icon,
  Tick01Icon,
  UserIcon,
  Layers01Icon,
} from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { isPermissionEnum } from '../utils/permissions'
import { PermissionList } from './permission-list'
import type { UserSet, PermissionEnum } from '@/lib/api/types.gen'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { ScrollArea } from '@/components/ui/scroll-area'

interface PermissionSetEditorProps {
  set: UserSet
}

export function PermissionSetEditor({ set }: PermissionSetEditorProps) {
  const queryClient = useQueryClient()
  const [isEditingInfo, setIsEditingInfo] = React.useState(false)
  const [name, setName] = React.useState(set.name)
  const [description, setDescription] = React.useState(set.description || '')

  React.useEffect(() => {
    setName(set.name)
    setDescription(set.description || '')
    setIsEditingInfo(false)
  }, [set.id, set.name, set.description])

  const { data: rawPermissions = '' } = useQuery({
    ...rbacApi.getSetPermissionsOptions(set.id),
    enabled: !!set.id,
  })

  const assignedPermissions = React.useMemo(
    () =>
      typeof rawPermissions === 'string' && rawPermissions
        ? rawPermissions.split(',').filter(isPermissionEnum)
        : [],
    [rawPermissions],
  )

  const { data: members = [] } = useQuery(rbacApi.getSetMembersOptions(set.id))

  const updateSet = useMutation({
    ...rbacApi.updateSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      setIsEditingInfo(false)
      toast.success('Permission set updated successfully')
    },
    onError: (err) =>
      toast.error(err instanceof Error ? err.message : 'Failed to update set'),
  })

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserSetPermissions', { user_set_id: set.id }],
      })
      toast.success('Permission added to set')
    },
    onError: (err) =>
      toast.error(
        err instanceof Error ? err.message : 'Failed to add permission',
      ),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserSetPermissions', { user_set_id: set.id }],
      })
      toast.success('Permission removed from set')
    },
    onError: (err) =>
      toast.error(
        err instanceof Error ? err.message : 'Failed to remove permission',
      ),
  })

  const handleTogglePermission = (
    permission: PermissionEnum,
    checked: boolean,
  ) => {
    if (checked) {
      assignPerm.mutate({
        path: { user_set_id: set.id },
        body: { permission },
      })
    } else {
      unassignPerm.mutate({
        path: { user_set_id: set.id },
        body: { permission },
      })
    }
  }

  return (
    <div className="flex flex-col h-full gap-6 animate-in fade-in duration-500">
      <Card className="border-none shadow-none overflow-hidden">
        <CardHeader className="p-4">
          {isEditingInfo ? (
            <div className="space-y-4 max-w-2xl">
              <div className="grid gap-4">
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider ml-1">
                    Set Name
                  </label>
                  <Input
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    className="h-11"
                  />
                </div>
                <div className="space-y-2">
                  <label className="text-[10px] font-bold text-muted-foreground uppercase tracking-wider ml-1">
                    Description
                  </label>
                  <Textarea
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    rows={2}
                    className="resize-none p-3"
                  />
                </div>
              </div>
              <div className="flex gap-2 pt-2">
                <Button
                  size="sm"
                  className="px-4 h-9 gap-2"
                  onClick={() =>
                    updateSet.mutate({
                      path: { permission_set_id: set.id },
                      body: { name, description },
                    })
                  }
                  disabled={updateSet.isPending}
                >
                  <HugeiconsIcon icon={Tick01Icon} className="size-4" />
                  Save Changes
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  className="px-4 h-9"
                  onClick={() => setIsEditingInfo(false)}
                >
                  Cancel
                </Button>
              </div>
            </div>
          ) : (
            <div className="flex items-start justify-between">
              <div className="space-y-2 flex-1">
                <div className="flex items-center gap-2 group">
                  <CardTitle className="text-2xl font-bold tracking-tight">
                    {set.name}
                  </CardTitle>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="size-8 opacity-0 group-hover:opacity-100 transition-opacity"
                    onClick={() => setIsEditingInfo(true)}
                  >
                    <HugeiconsIcon
                      icon={Edit01Icon}
                      className="size-4 text-primary"
                    />
                  </Button>
                </div>
                <p className="text-muted-foreground text-[15px] leading-relaxed max-w-3xl">
                  {set.description ||
                    'No description provided for this permission set.'}
                </p>

                <div className="flex items-center gap-4 mt-4 pt-2">
                  <div className="flex items-center gap-2">
                    <div className="size-2 rounded-full bg-primary" />
                    <span className="text-xs font-semibold text-foreground/70 uppercase tracking-wide">
                      {assignedPermissions.length} Permissions
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <div className="size-2 rounded-full bg-blue-500" />
                    <span className="text-xs font-semibold text-foreground/70 uppercase tracking-wide">
                      {members.length} Users Assigned
                    </span>
                  </div>
                </div>
              </div>
            </div>
          )}
        </CardHeader>
      </Card>

      <div className="grid grid-cols-5 gap-6 flex-1 min-h-0">
        <div className="col-span-3 flex flex-col gap-4 p-4 overflow-hidden">
          <div className="flex items-center justify-between mb-2">
            <h3 className="font-bold text-[13px] uppercase tracking-wider text-foreground/60 flex items-center gap-2">
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              Manage Bundle
            </h3>
            <Badge
              variant="secondary"
              className="font-mono"
            >
              {assignedPermissions.length} ASSIGNED
            </Badge>
          </div>

          <div className="flex-1 min-h-0">
            <PermissionList
              assignedPermissions={assignedPermissions}
              onToggle={handleTogglePermission}
            />
          </div>
        </div>

        {/* Assigned Users Summary (Right 2 Columns) */}
        <div className="col-span-2 flex flex-col gap-6 overflow-hidden">
          <Card className="flex-1 overflow-hidden flex flex-col">
            <CardHeader className="p-4">
              <CardTitle className="text-[13px] font-bold uppercase tracking-wider text-foreground/60 flex items-center justify-between">
                Assigned Users
                <Badge variant="secondary" className="font-mono h-5 px-1.5">
                  {members.length}
                </Badge>
              </CardTitle>
            </CardHeader>
            <CardContent className="p-0 flex-1 overflow-hidden">
              <ScrollArea className="h-full p-6">
                {members.length === 0 ? (
                  <div className="flex flex-col items-center justify-center py-12 text-center">
                    <div className="size-10 flex items-center justify-center mb-3">
                      <HugeiconsIcon
                        icon={UserIcon}
                        className="size-6 opacity-20"
                      />
                    </div>
                    <p className="text-sm font-medium text-muted-foreground">
                      No users assigned
                    </p>
                    <p className="text-xs text-muted-foreground/60 mt-1">
                      Users can be linked via User Permissions.
                    </p>
                  </div>
                ) : (
                  <div className="grid grid-cols-1 gap-2">
                    {members.map((member) => (
                      <div
                        key={member.id}
                        className="flex items-center gap-3 p-2"
                      >
                        <div className="size-8 flex items-center justify-center">
                          <HugeiconsIcon
                            icon={UserIcon}
                            className="size-4 text-primary/70"
                          />
                        </div>
                        <div className="flex flex-col min-w-0">
                          <span className="text-[13px] font-medium truncate">
                            {member.email}
                          </span>
                          <span className="text-[10px] text-muted-foreground uppercase font-mono tracking-tighter">
                            ID: {member.id.split('-')[0]}...
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </ScrollArea>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
