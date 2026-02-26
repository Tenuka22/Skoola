import { createFileRoute } from '@tanstack/react-router'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'

import type { TermFormValues } from '@/features/academics/terms/schemas'
import { authClient } from '@/lib/clients'
import { createTermMutation } from '@/lib/api/@tanstack/react-query.gen'
import { useTermsStore } from '@/features/academics/terms/store'
import { TermsHeader } from '@/features/academics/terms/components/terms-header'
import { TermsToolbar } from '@/features/academics/terms/components/terms-toolbar'
import { TermAddDialog } from '@/features/academics/terms/components/term-add-dialog'

export const Route = createFileRoute('/admin/academics/terms')({
  component: TermsPage,
})

function TermsPage() {
  const store = useTermsStore()
  const { setIsCreateTermOpen } = store

  const queryClient = useQueryClient()
  // There is no getAllTerms, so we'll just invalidate academic years
  const invalidateQueries = () => {
    queryClient.invalidateQueries({ queryKey: ['getAllAcademicYears'] })
  }

  const createTerm = useMutation({
    ...createTermMutation({
      client: authClient,
    }),
    onSuccess: () => {
      toast.success('Term created successfully.')
      invalidateQueries()
      setIsCreateTermOpen(false)
    },
    onError: (error) => {
      toast.error(`Failed to create term: ${error.message || 'Unknown error'}`)
    },
  })

  return (
    <div className="flex h-full flex-col bg-background">
      <TermsHeader />
      <TermsToolbar />

      <TermAddDialog
        open={store.isCreateTermOpen}
        onOpenChange={setIsCreateTermOpen}
        onConfirm={(data: TermFormValues) => createTerm.mutate({ body: data })}
        isSubmitting={createTerm.isPending}
      />
    </div>
  )
}
