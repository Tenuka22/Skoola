import { zodResolver } from '@hookform/resolvers/zod'
import * as React from 'react'
import { useForm } from 'react-hook-form'
import { bulkUpdateSchema } from '../schemas'
import type * as z from 'zod'
import type { BulkUpdateValues } from '../schemas'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { DialogFooter } from '@/components/ui/dialog'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { zRoleEnum } from '@/lib/api/zod.gen'
import { Spinner } from '@/components/ui/spinner'

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

  const { handleSubmit, setValue, watch, reset } = useForm<BulkUpdateValues>({
    resolver: zodResolver(bulkUpdateSchema),
    defaultValues: {
      is_verified: undefined,
      lockout_until: undefined,
      roles: undefined,
    },
  })

  React.useEffect(() => {
    reset()
  }, [reset])

  const onSubmit = (data: BulkUpdateValues) => {
    onConfirm(data)
  }

  const isVerified = watch('is_verified')
  const selectedRoles = watch('roles') || []

  const toggleRole = (roleName: z.infer<typeof zRoleEnum>) => {
    const current = selectedRoles || []
    if (current.includes(roleName)) {
      setValue(
        'roles',
        current.filter((r) => r !== roleName),
      )
    } else {
      setValue('roles', [...current, roleName])
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 pt-4">
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
              onCheckedChange={(checked) => setValue('is_verified', checked)}
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
                onClick={() =>
                  toggleRole(role.name as z.infer<typeof zRoleEnum>)
                }
                className="flex cursor-pointer items-center gap-2 rounded-sm p-2 hover:bg-muted"
              >
                <Checkbox
                  checked={selectedRoles.includes(
                    role.name as z.infer<typeof zRoleEnum>,
                  )}
                  onCheckedChange={() =>
                    toggleRole(role.name as z.infer<typeof zRoleEnum>)
                  }
                />
                <span className="text-sm font-medium">{role.name}</span>
              </div>
            ))}
          </div>
        </div>
      </div>

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
    </form>
  )
}
