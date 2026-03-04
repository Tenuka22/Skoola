import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import * as React from 'react'
import type { CurriculumStandardResponse } from '@/lib/api/types.gen'
import {
  getAllCurriculumStandardsQueryOptions,
  useCreateCurriculumStandard,
  useDeleteCurriculumStandard,
  useUpdateCurriculumStandard,
} from '@/features/academics/curriculum/api'
import { CurriculumHeader } from '@/features/academics/curriculum/components/curriculum-header'
import { CurriculumToolbar } from '@/features/academics/curriculum/components/curriculum-toolbar'
import { CurriculumListContainer } from '@/features/academics/curriculum/components/curriculum-list-container'
import { useCurriculumColumns } from '@/features/academics/curriculum/components/curriculum-table-columns'
import { CurriculumDialog } from '@/features/academics/curriculum/components/curriculum-dialog'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import { Stack } from '@/components/primitives'

export const Route = createFileRoute('/admin/academics/curriculum')({
  component: CurriculumPage,
})

function CurriculumPage() {
  const [search, setSearch] = React.useState('')
  const [standardToEdit, setStandardToEdit] =
    React.useState<CurriculumStandardResponse | null>(null)
  const [standardToDelete, setStandardToDelete] = React.useState<string | null>(
    null,
  )
  const [isAddOpen, setIsAddOpen] = React.useState(false)

  const curriculumQuery = useQuery(getAllCurriculumStandardsQueryOptions())
  const createMutation = useCreateCurriculumStandard()
  const updateMutation = useUpdateCurriculumStandard()
  const deleteMutation = useDeleteCurriculumStandard()

  const columns = useCurriculumColumns({
    onEdit: setStandardToEdit,
    onDelete: setStandardToDelete,
  })

  return (
    <Stack gap={4} p={0} className="h-full">
      <CurriculumHeader total={curriculumQuery.data?.length} />
      <CurriculumToolbar
        search={search}
        onSearchChange={setSearch}
        onAdd={() => setIsAddOpen(true)}
      />
      <CurriculumListContainer
        query={curriculumQuery}
        columns={columns}
        search={search}
      />

      <CurriculumDialog
        open={isAddOpen}
        onOpenChange={setIsAddOpen}
        onConfirm={(data) =>
          createMutation.mutate(
            { body: data },
            { onSuccess: () => setIsAddOpen(false) },
          )
        }
        isSubmitting={createMutation.isPending}
        title="Add Curriculum Standard"
      />

      <CurriculumDialog
        open={!!standardToEdit}
        onOpenChange={() => setStandardToEdit(null)}
        standard={standardToEdit}
        onConfirm={(data) =>
          standardToEdit &&
          updateMutation.mutate(
            {
              path: { standard_id: standardToEdit.id },
              body: data,
            },
            { onSuccess: () => setStandardToEdit(null) },
          )
        }
        isSubmitting={updateMutation.isPending}
        title="Edit Curriculum Standard"
      />

      <AlertDialog
        open={!!standardToDelete}
        onOpenChange={() => setStandardToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              curriculum standard.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={() =>
                standardToDelete &&
                deleteMutation.mutate(
                  { path: { standard_id: standardToDelete } },
                  { onSuccess: () => setStandardToDelete(null) },
                )
              }
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </Stack>
  )
}
