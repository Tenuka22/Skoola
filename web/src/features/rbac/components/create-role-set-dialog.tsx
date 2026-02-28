import * as React from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import { HugeiconsIcon } from '@hugeicons/react'
import { Add01Icon } from '@hugeicons/core-free-icons'
import { rbacApi } from '../api'
import { createRoleSetSchema } from '../schemas'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { HStack, Text } from '@/components/primitives'

export function CreateRoleSetDialog() {
  const [open, setOpen] = React.useState(false)
  const queryClient = useQueryClient()

  const createSet = useMutation({
    ...rbacApi.createRoleSetMutation(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['getAllRoleSets'] })
      setOpen(false)
      toast.success('Role set created')
    },
    onError: (err) => toast.error(err.message),
  })

  const config = defineFormConfig(createRoleSetSchema, {
    structure: [
      [
        {
          field: 'name',
          type: 'input',
          label: 'Set Name',
          placeholder: 'e.g. Administrative Group',
        },
      ],
      [
        {
          field: 'description',
          type: 'textarea',
          label: 'Description',
          placeholder: 'Describe what this role set includes...',
          rows: 3,
        },
      ],
    ],
  })

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger
        render={
          <Button size="sm">
            <HStack gap={1} align="center">
              <HugeiconsIcon icon={Add01Icon} className="size-4" />
              <Text>New Set</Text>
            </HStack>
          </Button>
        }
      />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Role Set</DialogTitle>
        </DialogHeader>
        <FormBuilder
          schema={createRoleSetSchema}
          config={config}
          onSubmit={(values) => {
            createSet.mutate({ body: values })
          }}
          isLoading={createSet.isPending}
          actions={[
            {
              label: 'Create Role Set',
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
