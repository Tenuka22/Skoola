import { createFileRoute } from '@tanstack/react-router'
import * as React from 'react'
import type { BehaviorIncidentTypeFormValues } from '@/features/behavior-management/schemas'
import type { BehaviorIncidentTypeResponse } from '@/lib/api/types.gen'
import { BehaviorHeader } from '@/features/behavior-management/components/behavior-header'
import { BehaviorToolbar } from '@/features/behavior-management/components/behavior-toolbar'
import { BehaviorTypesTable } from '@/features/behavior-management/components/behavior-types-table'
import { BehaviorTypeDialog } from '@/features/behavior-management/components/behavior-type-dialog'
import {
  useCreateBehaviorIncidentType,
  useUpdateBehaviorIncidentType,
} from '@/features/behavior-management/api'
import { Box, Stack } from '@/components/primitives'

export const Route = createFileRoute('/admin/behavior/')({
  component: BehaviorPage,
})

function BehaviorPage() {
  const [isCreateTypeOpen, setIsCreateTypeOpen] = React.useState(false)
  const [typeToEdit, setTypeToEdit] =
    React.useState<BehaviorIncidentTypeResponse | null>(null)

  const createMutation = useCreateBehaviorIncidentType()

  const updateMutation = useUpdateBehaviorIncidentType()

  return (
    <Stack gap={4} p={8} className="h-full bg-background">
      <BehaviorHeader />
      <BehaviorToolbar setIsCreateTypeOpen={setIsCreateTypeOpen} />
      <Box className="pt-0">
        <BehaviorTypesTable setTypeToEdit={setTypeToEdit} />
      </Box>

      <BehaviorTypeDialog
        open={isCreateTypeOpen}
        onOpenChange={setIsCreateTypeOpen}
        onConfirm={(data: BehaviorIncidentTypeFormValues) =>
          createMutation.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateTypeOpen(false)
              },
            },
          )
        }
        isSubmitting={createMutation.isPending}
      />

      <BehaviorTypeDialog
        type={typeToEdit}
        open={!!typeToEdit}
        onOpenChange={() => setTypeToEdit(null)}
        onConfirm={(data: BehaviorIncidentTypeFormValues) =>
          typeToEdit &&
          updateMutation.mutate(
            {
              path: { type_id: typeToEdit.id },
              body: data,
            },
            {
              onSuccess: () => {
                setTypeToEdit(null)
              },
            },
          )
        }
        isSubmitting={updateMutation.isPending}
      />
    </Stack>
  )
}
