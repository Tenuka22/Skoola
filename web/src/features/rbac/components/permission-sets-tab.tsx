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
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { Box, HStack, Stack, Text } from '@/components/primitives'
import { Skeleton } from '@/components/ui/skeleton'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
} from '@/components/ui/empty'
import {
  Card,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
} from '@/components/ui/input-group'

export function PermissionSetsTab() {
  const [search, setSearch] = React.useState('')
  const { selectedPermissionSetId, setSelectedPermissionSetId } = useRBACStore()
  const queryClient = useQueryClient()

  const { data: sets = [], isLoading } = useQuery(rbacApi.getSetsOptions())

  const deleteSet = useMutation({
    ...rbacApi.deleteSetMutation(),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      if (selectedPermissionSetId === variables.path.user_set_id) {
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
    <Card>
      <CardHeader>
        <HStack className="h-full" align="start" p={0} gap={4}>
          <Stack gap={4}>
            <Stack gap={0} p={0}>
              <CardTitle>Permission Sets</CardTitle>
              <CardDescription>
                Manage reusable permission bundles.
              </CardDescription>
            </Stack>

            <CreatePermissionSetDialog />

            <Box p={0}>
              <InputGroup>
                <InputGroupInput
                  id="inline-start-input"
                  placeholder="Search sets..."
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  className="h-9 text-xs"
                />
                <InputGroupAddon align="inline-start">
                  <HugeiconsIcon icon={Search01Icon} className="size-3.5" />
                </InputGroupAddon>
              </InputGroup>
            </Box>

            <Stack gap={1} p={0} className="min-w-72">
              <Stack gap={2} p={0}>
                {isLoading ? (
                  Array.from({ length: 5 }).map((_, i) => (
                    <Skeleton key={i} className="h-12 rounded-lg" />
                  ))
                ) : filteredSets.length === 0 ? (
                  <Empty className="border-0 w-full justify-center py-12">
                    <EmptyHeader>
                      <EmptyMedia variant="icon">
                        <HugeiconsIcon icon={Layers01Icon} />
                      </EmptyMedia>
                      <EmptyTitle className="text-sm">No sets found</EmptyTitle>
                    </EmptyHeader>
                  </Empty>
                ) : (
                  filteredSets.map((set) => (
                    <Button
                      variant={
                        selectedPermissionSetId === set.id
                          ? 'secondary'
                          : 'ghost'
                      }
                      key={set.id}
                      onClick={() => setSelectedPermissionSetId(set.id)}
                      className="w-full justify-start h-12"
                    >
                      <HStack justify="between" p={1} className="w-full">
                        <HStack>
                          <HugeiconsIcon
                            icon={Layers01Icon}
                            className={cn(
                              'size-5 transition-colors',
                              selectedPermissionSetId === set.id
                                ? 'text-primary'
                                : 'text-muted-foreground/60',
                            )}
                          />
                          <Stack gap={0} className="text-left">
                            <Text size="sm" className="font-semibold truncate">
                              {set.name}
                            </Text>
                            <Text size="xs" muted className="line-clamp-1">
                              {set.description || 'No description provided.'}
                            </Text>
                          </Stack>
                        </HStack>
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={() => {
                            deleteSet.mutate({
                              path: { user_set_id: set.id },
                            })
                          }}
                        >
                          <HugeiconsIcon
                            icon={Delete02Icon}
                            className="size-3.5 text-destructive"
                          />
                        </Button>
                      </HStack>
                    </Button>
                  ))
                )}
              </Stack>
            </Stack>
          </Stack>

          <Box className="flex-1 h-full">
            {selectedSet ? (
              <PermissionSetEditor set={selectedSet} key={selectedSet.id} />
            ) : isLoading ? (
              <Stack gap={4}>
                <Skeleton className="h-16 w-full" />
                <Skeleton className="h-64 w-full" />
              </Stack>
            ) : (
              <Empty className="border-0 w-full h-full justify-center">
                <EmptyHeader>
                  <EmptyMedia variant="icon">
                    <HugeiconsIcon
                      icon={Layers01Icon}
                      className="size-12 opacity-20"
                    />
                  </EmptyMedia>
                  <EmptyTitle className="text-lg">No Set Selected</EmptyTitle>
                  <EmptyDescription className="text-sm">
                    Select a permission set from the list to review its members
                    and configure its permission bundle.
                  </EmptyDescription>
                </EmptyHeader>
              </Empty>
            )}
          </Box>
        </HStack>
      </CardHeader>
    </Card>
  )
}
