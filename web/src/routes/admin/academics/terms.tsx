import { createFileRoute } from '@tanstack/react-router'
import * as React from 'react'

import type { TermFormValues } from '@/features/academics/terms/schemas'
import { TermsHeader } from '@/features/academics/terms/components/terms-header'
import { TermsToolbar } from '@/features/academics/terms/components/terms-toolbar'
import { TermAddDialog } from '@/features/academics/terms/components/term-add-dialog'
import { useCreateTerm } from '@/features/academics/terms/api'

export const Route = createFileRoute('/admin/academics/terms')({
  component: TermsPage,
})

function TermsPage() {
  const [isCreateTermOpen, setIsCreateTermOpen] = React.useState(false)

  const createTerm = useCreateTerm()

  return (
    <div className="flex h-full flex-col bg-background">
      <TermsHeader />
      <TermsToolbar setIsCreateTermOpen={setIsCreateTermOpen} />

      <TermAddDialog
        open={isCreateTermOpen}
        onOpenChange={setIsCreateTermOpen}
        onConfirm={(data: TermFormValues) =>
          createTerm.mutate(
            { body: data },
            {
              onSuccess: () => {
                setIsCreateTermOpen(false)
              },
            },
          )
        }
        isSubmitting={createTerm.isPending}
      />
    </div>
  )
}
