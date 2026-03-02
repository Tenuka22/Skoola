import { createFileRoute } from '@tanstack/react-router'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { BehaviorIncidentTypeFormValues } from '@/features/behavior-management/schemas'
import { BehaviorHeader } from '@/features/behavior-management/components/behavior-header'
import { BehaviorToolbar } from '@/features/behavior-management/components/behavior-toolbar'
import { BehaviorTypesTable } from '@/features/behavior-management/components/behavior-types-table'
import { BehaviorTypeDialog } from '@/features/behavior-management/components/behavior-type-dialog'
import { useBehaviorStore } from '@/features/behavior-management/store'
import {
  createBehaviorIncidentTypeMutation,
  updateBehaviorIncidentTypeMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { authClient } from '@/lib/clients'
import { Box, Stack } from '@/components/primitives'

export const Route = createFileRoute('/admin/behavior/')({
  component: BehaviorPage,
})

function BehaviorPage() {
  const store = useBehaviorStore()
  const queryClient = useQueryClient()

  const invalidateTypes = () => {
    queryClient.invalidateQueries({
      queryKey: ['getAllBehaviorIncidentTypes'],
    })
  }

  const createMutation = useMutation({
    ...createBehaviorIncidentTypeMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Behavior type created successfully.')
      invalidateTypes()
      store.setIsCreateTypeOpen(false)
    },
    onError: (error) => {
      toast.error(
        `Failed to create behavior type: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const updateMutation = useMutation({
    ...updateBehaviorIncidentTypeMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Behavior type updated successfully.')
      invalidateTypes()
      store.setTypeToEdit(null)
    },
    onError: (error) => {
      toast.error(
        `Failed to update behavior type: ${error.message || 'Unknown error'}`,
      )
    },
  })

  return (
    <Stack gap={4} p={8} className="h-full bg-background">
      <BehaviorHeader />
      <BehaviorToolbar />
      <Box className="pt-0">
        <BehaviorTypesTable />
      </Box>

      <BehaviorTypeDialog
        open={store.isCreateTypeOpen}
        onOpenChange={store.setIsCreateTypeOpen}
        onConfirm={(data: BehaviorIncidentTypeFormValues) =>
          createMutation.mutate({ body: data })
        }
        isSubmitting={createMutation.isPending}
      />

      <BehaviorTypeDialog
        type={store.typeToEdit}
        open={!!store.typeToEdit}
        onOpenChange={() => store.setTypeToEdit(null)}
        onConfirm={(data: BehaviorIncidentTypeFormValues) =>
          store.typeToEdit &&
          updateMutation.mutate({
            path: { type_id: store.typeToEdit.id },
            body: data,
          })
        }
        isSubmitting={updateMutation.isPending}
      />
    </Stack>
  )
}
