import * as React from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon } from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { createPermissionSetSchema } from '../schemas'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'

export function CreatePermissionSetDialog() {
  const [open, setOpen] = React.useState(false)
  const queryClient = useQueryClient()

  const createSet = useMutation({
    ...rbacApi.createSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllPermissionSets'] })
      setOpen(false)
      toast.success('Permission set created')
    },
    onError: (err) => toast.error(err.message),
  })

  const config = defineFormConfig(createPermissionSetSchema, {
    structure: [
      [
        {
          field: 'name',
          type: 'input',
          label: 'Set Name',
          placeholder: 'e.g. Academic Manager',
        },
      ],
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          placeholder: 'Describe what this set allows...',
          rows: 3,
        },
      ],
    ],
  })

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger
        render={
          <Button className="gap-2 w-full">
            <HugeiconsIcon icon={Add01Icon} className="size-4" />
            New Set
          </Button>
        }
      />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Permission Set</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={createPermissionSetSchema}
          config={config}
          onSubmit={(values) => {
            createSet.mutate({ body: values })
          }}
          isLoading={createSet.isPending}
          actions={[
            {
              label: 'Create Set',
              type: 'submit',
              variant: 'default',
            },
            {
              label: 'Cancel',
              type: 'button',
              variant: 'outline',
              onClick: () => setOpen(false),
            },
          ]}
          showSuccessAlert={false}
          showErrorSummary={false}
        />
      </DialogContent>
    </Dialog>
  )
}
