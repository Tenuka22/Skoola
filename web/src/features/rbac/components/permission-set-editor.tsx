import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Alert01Icon,
  Delete02Icon,
  Edit01Icon,
  Tick01Icon,
  UserIcon,
} from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { PermissionPalette } from './permission-palette'
import type { PermissionEnum, UserSet } from '@/lib/api/types.gen'
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

  const { data: rawPermissions = [] } = useQuery(
    rbacApi.getSetPermissionsOptions(set.id),
  )

  const assignedPermissions = React.useMemo(
    () =>
      Array.isArray(rawPermissions) ? (rawPermissions as Array<PermissionEnum>) : [],
    [rawPermissions],
  )

  const { data: members = [] } = useQuery(rbacApi.getSetMembersOptions(set.id))

  const updateSet = useMutation({
    ...rbacApi.updateSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      setIsEditingInfo(false)
      toast.success('Set info updated')
    },
    onError: (err) => toast.error(err.message),
  })

  const assignPerm = useMutation({
    ...rbacApi.assignPermissionToSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserSetPermissions', { user_set_id: set.id }],
      })
      toast.success('Permission assigned to set')
    },
    onError: (err) => toast.error(err.message),
  })

  const unassignPerm = useMutation({
    ...rbacApi.unassignPermissionFromSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ['getUserSetPermissions', { user_set_id: set.id }],
      })
      toast.success('Permission removed from set')
    },
    onError: (err) => toast.error(err.message),
  })

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    const permission = e.dataTransfer.getData('permission') as PermissionEnum
    if (permission && !assignedPermissions.includes(permission)) {
      assignPerm.mutate({
        path: { user_set_id: set.id },
        body: { permission },
      })
    }
  }

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
  }

  return (
    <div className="flex flex-col h-full gap-6">
      <Card className="border shadow-sm">
        <CardHeader className="pb-4">
          {isEditingInfo ? (
            <div className="space-y-4">
              <div className="space-y-2">
                <label className="text-xs font-semibold text-muted-foreground uppercase">
                  Set Name
                </label>
                <Input value={name} onChange={(e) => setName(e.target.value)} />
              </div>
              <div className="space-y-2">
                <label className="text-xs font-semibold text-muted-foreground uppercase">
                  Description
                </label>
                <Textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  rows={2}
                />
              </div>
              <div className="flex gap-2">
                <Button
                  size="sm"
                  onClick={() =>
                    updateSet.mutate({
                      path: { permission_set_id: set.id },
                      body: { name, description },
                    })
                  }
                >
                  <HugeiconsIcon icon={Tick01Icon} className="size-4 mr-2" />
                  Save Changes
                </Button>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={() => setIsEditingInfo(false)}
                >
                  Cancel
                </Button>
              </div>
            </div>
          ) : (
            <div className="flex items-start justify-between">
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <CardTitle className="text-2xl">{set.name}</CardTitle>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="size-8"
                    onClick={() => setIsEditingInfo(true)}
                  >
                    <HugeiconsIcon
                      icon={Edit01Icon}
                      className="size-4 text-muted-foreground"
                    />
                  </Button>
                </div>
                <p className="text-muted-foreground">{set.description}</p>
              </div>
              <Badge variant="secondary" className="font-mono">
                {assignedPermissions.length} permissions
              </Badge>
            </div>
          )}
        </CardHeader>
      </Card>

      <div className="grid grid-cols-2 gap-6 flex-1 min-h-0">
        <div className="flex flex-col gap-6 overflow-hidden">
          <div
            className="flex flex-col gap-4 border-2 border-dashed rounded-xl bg-muted/5 p-6 overflow-hidden flex-[2]"
            onDrop={handleDrop}
            onDragOver={handleDragOver}
          >
            <div className="flex items-center justify-between">
              <h3 className="font-semibold text-sm flex items-center gap-2">
                Permissions in this Set
                <Badge variant="outline">{assignedPermissions.length}</Badge>
              </h3>
            </div>

            <ScrollArea className="flex-1 pr-4">
              {assignedPermissions.length === 0 ? (
                <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
                  <HugeiconsIcon
                    icon={Alert01Icon}
                    className="size-8 mb-2 opacity-20"
                  />
                  <p className="text-sm">No permissions assigned to this set</p>
                  <p className="text-xs">Drag from the palette to add</p>
                </div>
              ) : (
                <div className="flex flex-wrap gap-2 pt-1">
                  {assignedPermissions.map((perm) => (
                    <Badge
                      key={perm}
                      className="flex items-center gap-1 pl-2 pr-1 py-1 h-8"
                    >
                      {perm}
                      <Button
                        variant="ghost"
                        size="icon"
                        className="size-5 p-0 hover:bg-destructive/20 hover:text-destructive transition-colors"
                        onClick={() =>
                          unassignPerm.mutate({
                            path: { user_set_id: set.id },
                            body: { permission: perm },
                          })
                        }
                      >
                        <HugeiconsIcon icon={Delete02Icon} className="size-3" />
                      </Button>
                    </Badge>
                  ))}
                </div>
              )}
            </ScrollArea>
          </div>

          <Card className="flex-1 border-none shadow-none bg-muted/30">
            <CardHeader className="py-3 px-6">
              <CardTitle className="text-sm flex items-center justify-between">
                Assigned Users
                <Badge variant="outline" className="font-mono">
                  {members.length}
                </Badge>
              </CardTitle>
            </CardHeader>
            <CardContent className="px-6 pb-4">
              <ScrollArea className="h-24">
                {members.length === 0 ? (
                  <p className="text-xs text-muted-foreground italic">
                    No users explicitly assigned to this set yet.
                  </p>
                ) : (
                  <div className="flex flex-wrap gap-2">
                    {members.map((member) => (
                      <Badge
                        key={member.id}
                        variant="secondary"
                        className="gap-1 px-2 py-1"
                      >
                        <HugeiconsIcon icon={UserIcon} className="size-3" />
                        {member.email}
                      </Badge>
                    ))}
                  </div>
                )}
              </ScrollArea>
            </CardContent>
          </Card>
        </div>

        <div className="flex flex-col gap-4 overflow-hidden border rounded-xl bg-card p-6">
          <h3 className="font-semibold text-sm">
            Available Permissions Palette
          </h3>
          <PermissionPalette />
        </div>
      </div>
    </div>
  )
}
