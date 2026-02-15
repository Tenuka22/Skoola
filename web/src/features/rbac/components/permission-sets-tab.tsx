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
    <div className="flex h-[calc(100vh-200px)] gap-6 overflow-hidden">
      {/* Left Panel: Sets List (30%) */}
      <div className="flex flex-col w-[30%] gap-4 border rounded-xl bg-card p-4">
        <div className="flex items-center justify-between gap-2">
          <div className="relative flex-1">
            <HugeiconsIcon
              icon={Search01Icon}
              className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
            />
            <Input
              placeholder="Search sets..."
              className="pl-9 h-10"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <Button
            size="icon"
            onClick={() =>
              createSet.mutate({
                body: { name: 'New Set', description: 'Description here' },
              })
            }
          >
            <HugeiconsIcon icon={Add01Icon} className="size-4" />
          </Button>
        </div>

        <ScrollArea className="flex-1 -mx-2 px-2">
          <div className="space-y-1">
            {isLoading ? (
              Array.from({ length: 5 }).map((_, i) => (
                <div
                  key={i}
                  className="h-20 rounded-lg bg-muted animate-pulse mb-2"
                />
              ))
            ) : filteredSets.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
                <HugeiconsIcon
                  icon={Layers01Icon}
                  className="size-8 mb-2 opacity-20"
                />
                <p className="text-sm">No permission sets found</p>
              </div>
            ) : (
              filteredSets.map((set) => (
                <button
                  key={set.id}
                  onClick={() => setSelectedPermissionSetId(set.id)}
                  className={cn(
                    'w-full flex flex-col items-start gap-1 p-4 rounded-lg text-left transition-colors group relative',
                    selectedPermissionSetId === set.id
                      ? 'bg-primary/10 border-primary/20 border shadow-sm'
                      : 'hover:bg-muted/50 border-transparent border',
                  )}
                >
                  <div className="flex items-center justify-between w-full">
                    <span className="font-semibold text-sm truncate">
                      {set.name}
                    </span>
                    <HugeiconsIcon
                      icon={Layers01Icon}
                      className={cn(
                        'size-4',
                        selectedPermissionSetId === set.id
                          ? 'text-primary'
                          : 'text-muted-foreground',
                      )}
                    />
                  </div>
                  <p className="text-xs text-muted-foreground line-clamp-1 w-full mt-1">
                    {set.description}
                  </p>

                  <Button
                    variant="ghost"
                    size="icon"
                    className="absolute right-2 bottom-2 size-6 opacity-0 group-hover:opacity-100 hover:bg-destructive/10 hover:text-destructive transition-opacity"
                    onClick={(e) => {
                      e.stopPropagation()
                      if (
                        confirm('Are you sure you want to delete this set?')
                      ) {
                        deleteSet.mutate({
                          path: { permission_set_id: set.id },
                        })
                      }
                    }}
                  >
                    <HugeiconsIcon icon={Delete02Icon} className="size-3" />
                  </Button>
                </button>
              ))
            )}
          </div>
        </ScrollArea>
      </div>

      {/* Right Panel: Set Editor (70%) */}
      <div className="flex-1 overflow-hidden">
        {selectedSet ? (
          <PermissionSetEditor set={selectedSet} />
        ) : (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground border-2 border-dashed rounded-xl bg-muted/5">
            <HugeiconsIcon
              icon={Layers01Icon}
              className="size-12 mb-4 opacity-10"
            />
            <h3 className="text-lg font-medium">No Permission Set Selected</h3>
            <p className="text-sm max-w-[250px] text-center mt-1">
              Select a permission set from the list or create a new one to
              manage its permissions.
            </p>
          </div>
        )}
      </div>
    </div>
  )
}
