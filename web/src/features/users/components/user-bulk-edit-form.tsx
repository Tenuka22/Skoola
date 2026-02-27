import * as React from 'react'
import { bulkUpdateSchema } from '../schemas'
import type * as z from 'zod'
import type { BulkUpdateValues } from '../schemas'
import type { UseFormReturn } from 'react-hook-form'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { DialogFooter } from '@/components/ui/dialog'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { zRoleEnum } from '@/lib/api/zod.gen'
import { Spinner } from '@/components/ui/spinner'
import { FormBuilder, defineFormConfig } from '@/components/form-builder'

interface UserBulkEditFormProps {
  onConfirm: (data: BulkUpdateValues) => void
  onOpenChange: (open: boolean) => void
  isSubmitting?: boolean
}

export function UserBulkEditForm({
  onConfirm,
  onOpenChange,
  isSubmitting,
}: UserBulkEditFormProps) {
  const availableRoles = {
    data: Object.values(zRoleEnum.enum).map((roleName) => ({
      id: roleName,
      name: roleName,
    })),
  }

  const preload = React.useCallback(
    (form: UseFormReturn<BulkUpdateValues, unknown, BulkUpdateValues>) => {
      form.reset()
    },
    [],
  )

  const config = defineFormConfig(bulkUpdateSchema, {
    structure: [],
    extras: {
      top: (form) => {
        const isVerified = form.watch('is_verified')
        const selectedRoles = form.watch('roles') || []
        const toggleRole = (roleName: z.infer<typeof zRoleEnum>) => {
          const current = selectedRoles || []
          if (current.includes(roleName)) {
            form.setValue(
              'roles',
              current.filter((r: z.infer<typeof zRoleEnum>) => r !== roleName),
            )
          } else {
            form.setValue('roles', [...current, roleName])
          }
        }

        return (
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-4">
              <Label>Account Status</Label>
              <div className="flex items-center justify-between py-2">
                <div className="space-y-0.5">
                  <Label className="text-sm">Verification</Label>
                  <p className="text-xs text-muted-foreground">
                    Set email verified
                  </p>
                </div>
                <Switch
                  checked={isVerified === true}
                  onCheckedChange={(checked) =>
                    form.setValue('is_verified', checked)
                  }
                />
              </div>
            </div>

            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <Label>Access Roles</Label>
                {selectedRoles.length > 0 && (
                  <Badge variant="secondary">{selectedRoles.length} Set</Badge>
                )}
              </div>
              <div className="grid grid-cols-1 gap-2 max-h-[200px] overflow-y-auto border rounded-md p-2">
                {availableRoles?.data.map((role) => (
                  <div
                    key={role.id}
                    onClick={() => toggleRole(role.name)}
                    className="flex cursor-pointer items-center gap-2 rounded-sm p-2 hover:bg-muted"
                  >
                    <Checkbox
                      checked={selectedRoles.includes(role.name)}
                      onCheckedChange={() => toggleRole(role.name)}
                    />
                    <span className="text-sm font-medium">{role.name}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )
      },
      bottom: (form) => {
        const isVerified = form.watch('is_verified')
        const selectedRoles = form.watch('roles') || []

        return (
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={
                isSubmitting ||
                (isVerified === undefined && selectedRoles.length === 0)
              }
            >
              {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
              Apply Batch Changes
            </Button>
          </DialogFooter>
        )
      },
    },
  })

  return (
    <FormBuilder
      schema={bulkUpdateSchema}
      config={config}
      defaultValues={{
        is_verified: undefined,
        lockout_until: undefined,
        roles: undefined,
      }}
      onSubmit={(values) => onConfirm(values)}
      preload={preload}
      isLoading={isSubmitting}
      showErrorSummary={false}
      toastErrors={false}
      showSuccessAlert={false}
      actions={[]}
      className="space-y-6 pt-4"
    />
  )
}
