import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Delete02Icon,
  Layers01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { PermissionSetEditor } from './permission-set-editor'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

export function PermissionSetsTab() {
  const [search, setSearch] = React.useState('')
  const { selectedPermissionSetId, setSelectedPermissionSetId } = useRBACStore()
  const queryClient = useQueryClient()

  const { data: sets = [], isLoading } = useQuery(rbacApi.getSetsOptions())

  const createSet = useMutation({
    ...rbacApi.createSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      toast.success('Permission set created')
    },
    onError: (err) => toast.error(err.message),
  })

  const deleteSet = useMutation({
    ...rbacApi.deleteSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      setSelectedPermissionSetId(null)
      toast.success('Permission set deleted')
    },
    onError: (err) => toast.error(err.message),
  })

  const filteredSets = sets.filter((s) =>
    s.name.toLowerCase().includes(search.toLowerCase()),
  )

  const selectedSet = sets.find((s) => s.id === selectedPermissionSetId)

  return (
    <div className="flex h-full">
      {/* Left Panel: Sets List (400px fixed) */}
      <div className="flex flex-col w-[400px] shrink-0 gap-4 p-2">
        <div className="flex items-center justify-between">
          <div className="flex flex-col gap-1.5">
            <h2 className="text-sm font-semibold flex items-center gap-2">
              <HugeiconsIcon
                icon={Layers01Icon}
                className="size-4 text-primary"
              />
              Permission Sets
            </h2>
            <p className="text-xs text-muted-foreground">
              Manage reusable permission groups.
            </p>
          </div>
          <Button
            size="sm"
            className="h-8 gap-1.5 px-3"
            onClick={() =>
              createSet.mutate({
                body: {
                  name: 'New Permission Set',
                  description: "Briefly describe this set's purpose",
                },
              })
            }
          >
            <HugeiconsIcon icon={Add01Icon} className="size-4" />
            New Set
          </Button>
        </div>

        <div className="relative">
          <HugeiconsIcon
            icon={Search01Icon}
            className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
          />
          <Input
            placeholder="Search sets..."
            className="pl-9 h-11"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <ScrollArea className="flex-1 -mx-2 px-2">
          <div className="space-y-2 pb-4">
            {isLoading ? (
              Array.from({ length: 4 }).map((_, i) => (
                <div
                  key={i}
                  className="h-24 rounded-xl bg-muted animate-pulse mb-2"
                />
              ))
            ) : filteredSets.length === 0 ? (
              <div className="flex flex-col items-center justify-center py-12 text-muted-foreground">
                <HugeiconsIcon
                  icon={Layers01Icon}
                  className="size-10 mb-3 opacity-20"
                />
                <p className="text-sm font-medium">No permission sets found</p>
                <p className="text-xs opacity-60">
                  Try creating a new one above
                </p>
              </div>
            ) : (
              filteredSets.map((set) => (
                <button
                  key={set.id}
                  onClick={() => setSelectedPermissionSetId(set.id)}
                  className={cn(
                    'w-full flex flex-col items-start gap-1 p-2 text-left transition-all relative overflow-hidden group',
                    selectedPermissionSetId === set.id ? 'bg-primary/5' : '',
                  )}
                >
                  {selectedPermissionSetId === set.id && (
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-primary" />
                  )}
                  <div className="flex items-center justify-between w-full">
                    <span className="font-semibold text-[14px] truncate">
                      {set.name}
                    </span>
                    <div className="size-8 flex items-center justify-center">
                      <HugeiconsIcon
                        icon={Layers01Icon}
                        className={cn(
                          'size-4 transition-colors',
                          selectedPermissionSetId === set.id
                            ? 'text-primary'
                            : 'text-muted-foreground group-hover:text-primary',
                        )}
                      />
                    </div>
                  </div>
                  <p className="text-[13px] text-muted-foreground line-clamp-2 w-full mt-1 leading-snug">
                    {set.description || 'No description provided.'}
                  </p>

                  <div className="absolute right-2 bottom-2 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                    <Button
                      variant="ghost"
                      size="icon"
                      className="size-7"
                      onClick={(e) => {
                        e.stopPropagation()
                        if (
                          confirm(
                            'Are you sure you want to delete this permission set? This action cannot be undone.',
                          )
                        ) {
                          deleteSet.mutate({
                            path: { permission_set_id: set.id },
                          })
                        }
                      }}
                    >
                      <HugeiconsIcon icon={Delete02Icon} className="size-3.5" />
                    </Button>
                  </div>
                </button>
              ))
            )}
          </div>
        </ScrollArea>
      </div>

      {/* Right Panel: Set Editor (Remaining) */}
      <div className="flex-1 overflow-hidden h-full">
        {selectedSet ? (
          <PermissionSetEditor set={selectedSet} />
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <div className="size-16 flex items-center justify-center mb-6">
              <HugeiconsIcon
                icon={Layers01Icon}
                className="size-10 opacity-20"
              />
            </div>
            <h3 className="text-xl font-semibold text-foreground">
              No Set Selected
            </h3>
            <p className="text-sm max-w-[280px] text-center mt-2 leading-relaxed opacity-70">
              Select a permission set from the list on the left to review its
              member users and configure its specific permission bundle.
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
