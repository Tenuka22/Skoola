import * as React from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Delete02Icon,
  HierarchyIcon,
  Search01Icon,
} from '@hugeicons/core-free-icons'
import { useRBACStore } from '../store'
import { rbacApi } from '../api'
import { RoleSetEditor } from './role-set-editor'
import { CreateRoleSetDialog } from './create-role-set-dialog'
import { Input } from '@/components/ui/input'
import { ScrollArea } from '@/components/ui/scroll-area'
import { Button } from '@/components/ui/button'
import { Box, HStack, Heading, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'

export function RoleSetsTab() {
  const [search, setSearch] = React.useState('')
  const { selectedRoleSetId, setSelectedRoleSetId } = useRBACStore()
  const queryClient = useQueryClient()

  const { data: sets = [], isLoading } = useQuery(rbacApi.getRoleSetsOptions())

  const deleteSet = useMutation({
    ...rbacApi.deleteRoleSetMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['getAllRoleSets'] })
      if (selectedRoleSetId === variables.path.role_set_id) {
        setSelectedRoleSetId(null)
      }
      toast.success('Role set deleted')
    },
    onError: (err) => toast.error(err.message),
  })

  const filteredSets =
    sets?.filter((s) => s.name.toLowerCase().includes(search.toLowerCase())) ||
    []

  const selectedSet = sets?.find((s) => s.id === selectedRoleSetId)

  return (
    <Stack
      gap={0}
      className="h-full overflow-hidden rounded-xl border border-border/60 bg-background shadow-sm"
    >
      <HStack className="h-full" align="start">
        <Stack gap={4} className="min-w-72 shrink-0 h-full border-r">
          <Stack gap={1} p={4} className="border-b">
            <HStack justify="between">
              <Heading size="h4">Role Sets</Heading>
              <CreateRoleSetDialog />
            </HStack>
            <Text size="sm" muted>
              Group roles together for easier assignment.
            </Text>
          </Stack>

          <Box px={4}>
            <Box className="relative">
              <HugeiconsIcon
                icon={Search01Icon}
                className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground"
              />
              <Input
                placeholder="Search role sets..."
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
                <Empty className="border-0 w-full justify-center">
                  <EmptyHeader>
                    <EmptyMedia variant="icon">
                      <HugeiconsIcon icon={HierarchyIcon} />
                    </EmptyMedia>
                    <EmptyTitle>No role sets found</EmptyTitle>
                  </EmptyHeader>
                </Empty>
              ) : (
                filteredSets.map((set) => (
                  <Button
                    key={set.id}
                    onClick={() => setSelectedRoleSetId(set.id)}
                    variant={
                      selectedRoleSetId === set.id ? 'secondary' : 'outline'
                    }
                    className="w-full text-left h-14 transition-colors group justify-between"
                  >
                    <Stack gap={1}>
                      <Text className="font-semibold text-sm truncate">
                        {set.name}
                      </Text>
                      <Text size="xs" muted>
                        {set.description || 'No description provided.'}
                      </Text>
                    </Stack>
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
                            path: { role_set_id: set.id },
                          })
                        }
                      }}
                    >
                      <HugeiconsIcon
                        icon={Delete02Icon}
                        className="size-3.5 text-destructive"
                      />
                    </Button>
                  </Button>
                ))
              )}
            </Stack>
          </ScrollArea>
        </Stack>

        {/* Right Panel: Role Set Editor */}
        <Box className="flex-1 h-full overflow-y-auto">
          <Box p={6}>
            {selectedSet ? (
              <RoleSetEditor set={selectedSet} key={selectedSet.id} />
            ) : (
              <Empty className="border-0 w-full h-[60vh] justify-center">
                <EmptyHeader>
                  <EmptyMedia variant="icon">
                    <HugeiconsIcon icon={HierarchyIcon} />
                  </EmptyMedia>
                  <EmptyTitle className="text-xl">
                    No Role Set Selected
                  </EmptyTitle>
                  <EmptyDescription className="max-w-xs">
                    Select a role set from the list to review its roles and
                    configure its settings.
                  </EmptyDescription>
                </EmptyHeader>
              </Empty>
            )}
          </Box>
        </Box>
      </HStack>
    </Stack>
  )
}
