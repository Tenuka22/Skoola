import { HugeiconsIcon } from '@hugeicons/react'
import {
  Add01Icon,
  Delete02Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import { toast } from 'sonner'
import type { z } from 'zod'
import type {
  StudentGuardianResponse,
  StudentResponse,
} from '@/lib/api/types.gen'
import type { UseFormReturn } from 'react-hook-form'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Spinner } from '@/components/ui/spinner'
import { authClient } from '@/lib/clients'
import {
  addGuardianToStudentMutation,
  getAllGuardiansForStudentOptions,
  removeGuardianFromStudentMutation,
} from '@/lib/api/@tanstack/react-query.gen'
import { zCreateStudentGuardianRequest } from '@/lib/api/zod.gen'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

const guardianFormSchema = zCreateStudentGuardianRequest.omit({
  student_id: true,
})

type GuardianFormValues = z.infer<typeof guardianFormSchema>

interface StudentGuardiansDialogProps {
  student: StudentResponse | null
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function StudentGuardiansDialog({
  student,
  open,
  onOpenChange,
}: StudentGuardiansDialogProps) {
  const queryClient = useQueryClient()

  const {
    data: guardiansData,
    isLoading,
    isError,
    error,
  } = useQuery({
    ...getAllGuardiansForStudentOptions({
      client: authClient,
      path: { student_id: student?.id ?? '' },
    }),
    enabled: !!student,
  })
  const guardians = guardiansData || []

  const addGuardian = useMutation({
    ...addGuardianToStudentMutation({ client: authClient }),
    onError: (error) => {
      toast.error(`Failed to add guardian: ${error.message || 'Unknown error'}`)
    },
  })

  const removeGuardian = useMutation({
    ...removeGuardianFromStudentMutation({ client: authClient }),
    onSuccess: () => {
      toast.success('Guardian removed successfully.')
      queryClient.invalidateQueries({
        queryKey: ['getAllGuardiansForStudent', { student_id: student?.id }],
      })
    },
    onError: (error) => {
      toast.error(
        `Failed to remove guardian: ${error.message || 'Unknown error'}`,
      )
    },
  })

  const onSubmit = (
    data: GuardianFormValues,
    form: UseFormReturn<GuardianFormValues, unknown, GuardianFormValues>,
  ) => {
    if (student) {
      addGuardian.mutate(
        {
          path: { student_id: student.id },
          body: { ...data, student_id: student.id },
        },
        {
          onSuccess: () => {
            toast.success('Guardian added successfully.')
            queryClient.invalidateQueries({
              queryKey: [
                'getAllGuardiansForStudent',
                { student_id: student?.id },
              ],
            })
            form.reset()
          },
        },
      )
    }
  }

  const config = defineFormConfig(guardianFormSchema, {
    structure: [],
    extras: {
      top: (form) => (
        <>
          <h4 className="text-sm font-semibold">Add New Guardian</h4>
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-1">
              <Label htmlFor="guardian_id">NIC / ID</Label>
              <Input
                id="guardian_id"
                {...form.register('id')}
                placeholder="Guardian ID"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="guardian_name">Full Name</Label>
              <Input
                id="guardian_name"
                {...form.register('name')}
                placeholder="Full Name"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="relationship">Relationship</Label>
              <Input
                id="relationship"
                {...form.register('relationship')}
                placeholder="e.g. Father, Mother"
              />
            </div>
            <div className="space-y-1">
              <Label htmlFor="phone">Phone</Label>
              <Input
                id="phone"
                {...form.register('phone')}
                placeholder="Phone Number"
              />
            </div>
            <div className="col-span-2 space-y-1">
              <Label htmlFor="address">Address</Label>
              <Input
                id="address"
                {...form.register('address')}
                placeholder="Residential Address"
              />
            </div>
          </div>
        </>
      ),
      bottom: (
        <Button
          type="submit"
          className="w-full"
          disabled={addGuardian.isPending}
        >
          {addGuardian.isPending ? (
            <Spinner className="mr-2" />
          ) : (
            <HugeiconsIcon icon={Add01Icon} className="size-4 mr-2" />
          )}
          Add Guardian
        </Button>
      ),
    },
  })

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl flex flex-col h-[85vh]">
        <DialogHeader>
          <DialogTitle>Guardians: {student?.name_english}</DialogTitle>
          <DialogDescription>
            Manage guardians and emergency contacts for this student.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-6 flex-1 overflow-hidden">
          {/* Current Guardians */}
          <div className="flex-1 flex flex-col min-h-0">
            <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
              <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
              Current Guardians
            </h3>
            {isLoading ? (
              <div className="grid flex-1 place-items-center">
                <Spinner />
              </div>
            ) : isError ? (
              <p className="text-xs text-destructive">Error: {error.message}</p>
            ) : guardians.length === 0 ? (
              <p className="text-xs text-muted-foreground italic text-center p-8 border rounded-lg border-dashed">
                No guardians linked to this student.
              </p>
            ) : (
              <div className="divide-y">
                {guardians.map((guardian: StudentGuardianResponse) => (
                  <div
                    key={guardian.id}
                    className="p-3 flex items-center justify-between"
                  >
                    <div className="flex flex-col">
                      <span className="font-medium text-sm">
                        {guardian.name}
                      </span>
                      <span className="text-xs text-muted-foreground">
                        {guardian.relationship} | {guardian.phone}
                      </span>
                    </div>
                    <Button
                      variant="ghost"
                      size="icon"
                      className="size-8 text-destructive hover:bg-destructive/10"
                      onClick={() =>
                        student &&
                        removeGuardian.mutate({
                          path: {
                            student_id: student.id,
                            guardian_id: guardian.id,
                          },
                        })
                      }
                    >
                      <HugeiconsIcon icon={Delete02Icon} className="size-4" />
                    </Button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Add Guardian Form */}
          <FormBuilder
            schema={guardianFormSchema}
            config={config}
            defaultValues={{
              id: '',
              name: '',
              relationship: '',
              phone: '',
              address: '',
              email: '',
            }}
            onSubmit={onSubmit}
            isLoading={addGuardian.isPending}
            showErrorSummary={false}
            toastErrors={false}
            showSuccessAlert={false}
            actions={[]}
            className="space-y-4 p-4 border rounded-xl bg-muted/30"
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}
