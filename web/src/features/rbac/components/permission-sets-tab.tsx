import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  Layers01Icon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { PermissionSetEditor } from './permission-set-editor'
import { CreatePermissionSetDialog } from './create-permission-set-dialog'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'

export function PermissionSetsTab() {
  const [search, setSearch] = React.useState('')
  const { selectedPermissionSetId, setSelectedPermissionSetId } = useRBACStore()
  const queryClient = useQueryClient()

  const { data: sets = [], isLoading } = useQuery(rbacApi.getSetsOptions())

  const deleteSet = useMutation({
    ...rbacApi.deleteSetMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      if (selectedPermissionSetId === variables.path.permission_set_id) {
        setSelectedPermissionSetId(null)
      }
      toast.success('Permission set deleted')
    },
    onError: (err) => toast.error(err.message),
  })

  const filteredSets =
    sets?.filter((s) => s.name.toLowerCase().includes(search.toLowerCase())) ||
    []

  const selectedSet = sets?.find((s) => s.id === selectedPermissionSetId)

  return (
    <div className="h-full flex flex-col overflow-hidden rounded-xl border border-border/60 bg-background shadow-sm">
      <HStack className="h-full" align="start">
        {/* Left Panel: Sets List */}
        <Stack gap={4} className="w-[350px] shrink-0 h-full border-r">
          <Stack gap={1} p={4} className="border-b">
            <HStack justify="between">
              <Heading size="h4">Permission Sets</Heading>
              <CreatePermissionSetDialog />
            </HStack>
            <Text size="sm" muted>
              Manage reusable permission bundles.
            </Text>
          </Stack>

          <Box px={4}>
            <Box className="relative">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
              />
              <Input
                placeholder="Search sets..."
                className="pl-9"
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </Box>
          </Box>

          <ScrollArea className="flex-1">
            <Stack gap={2} p={4} className="pt-0">
              {isLoading ? (
                Array.from({ length: 5 }).map((_, i) => (
                  <Skeleton key={i} className="h-20 rounded-lg" />
                ))
              ) : filteredSets.length === 0 ? (
                <Stack
                  align="center"
                  className="justify-center py-12 text-center"
                  gap={2}
                >
                  <HugeiconsIcon
                    icon={Layers01Icon}
                    className="size-8 text-muted-foreground"
                  />
                  <Text size="sm" className="font-medium text-muted-foreground">
                    No sets found
                  </Text>
                </Stack>
              ) : (
                filteredSets.map((set) => (
                  <button
                    key={set.id}
                    onClick={() => setSelectedPermissionSetId(set.id)}
                    className={cn(
                      'w-full text-left p-3 rounded-lg transition-colors relative group',
                      selectedPermissionSetId === set.id
                        ? 'bg-muted'
                        : 'hover:bg-muted/50',
                    )}
                  >
                    <Stack gap={1}>
                      <HStack justify="between">
                        <Text className="font-semibold text-sm truncate">
                          {set.name}
                        </Text>
                        <HugeiconsIcon
                          icon={Layers01Icon}
                          className={cn(
                            'size-4 transition-colors',
                            selectedPermissionSetId === set.id
                              ? 'text-primary'
                              : 'text-muted-foreground/60',
                          )}
                        />
                      </HStack>
                      <Text size="xs" muted className="line-clamp-2">
                        {set.description || 'No description provided.'}
                      </Text>
                    </Stack>
                    <Box className="absolute right-1 bottom-1 opacity-0 group-hover:opacity-100 transition-opacity">
                      <Button
                        variant="ghost"
                        size="icon"
                        className="size-7 hover:bg-destructive/10"
                        onClick={(e) => {
                          e.stopPropagation()
                          if (
                            confirm(
                              `Are you sure you want to delete "${set.name}"? This action cannot be undone.`,
                            )
                          ) {
                            deleteSet.mutate({
                              path: { permission_set_id: set.id },
                            })
                          }
                        }}
                      >
                        <HugeiconsIcon
                          icon={Delete02Icon}
                          className="size-3.5 text-destructive"
                        />
                      </Button>
                    </Box>
                  </button>
                ))
              )}
            </Stack>
          </ScrollArea>
        </Stack>

        {/* Right Panel: Set Editor */}
        <Box className="flex-1 h-full overflow-y-auto">
          <Box p={6}>
            {selectedSet ? (
              <PermissionSetEditor set={selectedSet} key={selectedSet.id} />
            ) : (
              <Stack
                align="center"
                justify="center"
                className="h-[60vh] text-center"
                gap={2}
              >
                <HugeiconsIcon
                  icon={Layers01Icon}
                  className="size-12 text-muted-foreground/50"
                />
                <Heading size="h3">No Set Selected</Heading>
                <Text muted className="max-w-xs">
                  Select a permission set from the list to review its members
                  and configure its permission bundle.
                </Text>
              </Stack>
            )}
          </Box>
        </Box>
      </HStack>
    </div>
  )
}
