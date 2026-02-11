import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import {  bulkUpdateSchema } from '../schemas'
import type {BulkUpdateValues} from '../schemas';
import { zRoleEnum } from '@/lib/api/zod.gen'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Field, FieldGroup, FieldLabel } from '@/components/ui/field'
import { Switch } from '@/components/ui/switch'
import { Checkbox } from '@/components/ui/checkbox'



import { Badge } from '@/components/ui/badge'

interface UserBulkEditDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: BulkUpdateValues) => void
  selectedCount: number
  isSubmitting?: boolean
}

export function UserBulkEditDialog({
  open,
  onOpenChange,
  onConfirm,
  selectedCount,
  isSubmitting,
}: UserBulkEditDialogProps) {
  const availableRoles = {
    data: Object.values(zRoleEnum.enum).map((roleName) => ({
      id: roleName as z.infer<typeof zRoleEnum>,
      name: roleName as z.infer<typeof zRoleEnum>,
    })),
  }

  const {
    handleSubmit,
    setValue,
    watch,
    reset,
  } = useForm<BulkUpdateValues>({
    resolver: zodResolver(bulkUpdateSchema),
    defaultValues: {
      is_verified: undefined,
      is_locked: undefined,
      roles: undefined,
    },
  })

  React.useEffect(() => {
    if (open) {
      reset()
    }
  }, [open, reset])

  const onSubmit = (data: BulkUpdateValues) => {
    onConfirm(data)
  }

  const isVerified = watch('is_verified')
  const isLocked = watch('is_locked')
  const selectedRoles = watch('roles') || []


  const toggleRole = (roleName: z.infer<typeof zRoleEnum>) => {
    const current = selectedRoles as z.infer<typeof zRoleEnum>[]
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
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
            <HugeiconsIcon icon={PencilEdit01Icon} className="size-10" />
          </div>
          <DialogTitle className="text-center text-3xl font-black tracking-tight">
            Bulk Edit Users
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
            Updating {selectedCount} selected user identities. Fields left
            unchanged will remain as they are.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className="mt-8 space-y-8">
          <div className="grid grid-cols-2 gap-6">
            <FieldGroup className="space-y-4">
              <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
                Account Status
              </FieldLabel>
              <Field className="flex items-center justify-between rounded-2xl bg-muted/30 p-4 transition-colors hover:bg-muted/50">
                <div className="space-y-0.5">
                  <FieldLabel className="text-sm font-bold">
                    Verification
                  </FieldLabel>
                  <p className="text-[10px] font-medium opacity-50">
                    Set email verified
                  </p>
                </div>
                <Switch
                  checked={isVerified === true}
                  onCheckedChange={(checked) =>
                    setValue('is_verified', checked)
                  }
                />
              </Field>

              <Field className="flex items-center justify-between rounded-2xl bg-muted/30 p-4 transition-colors hover:bg-muted/50">
                <div className="space-y-0.5">
                  <FieldLabel className="text-sm font-bold">Lockout</FieldLabel>
                  <p className="text-[10px] font-medium opacity-50">
                    Restrict access
                  </p>
                </div>
                <Switch
                  checked={isLocked === true}
                  onCheckedChange={(checked) => setValue('is_locked', checked)}
                />
              </Field>
            </FieldGroup>

                        <FieldGroup className="space-y-4">
                          <div className="flex items-center justify-between">
                            <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
                              Access Roles
                            </FieldLabel>
                            {selectedRoles.length > 0 && (
                              <Badge
                                variant="secondary"
                                className="rounded-md px-1.5 py-0 text-[9px] font-black uppercase tracking-tighter"
                              >
                                {selectedRoles.length} Set
                              </Badge>
                            )}
                          </div>
                          <div className="grid grid-cols-1 gap-2 max-h-[200px] overflow-y-auto pr-2 custom-scrollbar">
                            {availableRoles?.data.map((role) => (
                              <div
                                key={role.id}
                                onClick={() => toggleRole(role.name)}
                                className="flex cursor-pointer items-center gap-3 rounded-xl border border-transparent bg-muted/20 p-3 transition-colors hover:bg-muted/40 hover:ring-1 hover:ring-primary/20"
                              >
                                <Checkbox
                                  checked={selectedRoles.includes(role.name)}
                                  onCheckedChange={() => toggleRole(role.name)}
                                  className="rounded-md"
                                />
                                <span className="text-xs font-bold tracking-tight">
                                  {role.name}
                                </span>
                              </div>
                            ))}
                          </div>
                        </FieldGroup>          </div>

          <DialogFooter className="mt-10 sm:justify-center gap-3 border-t pt-8">
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
              className="h-14 min-w-[120px] rounded-2xl font-black uppercase tracking-widest"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={
                isSubmitting ||
                (isVerified === undefined &&
                  isLocked === undefined &&
                  selectedRoles.length === 0)
              }
              className="h-14 min-w-[240px] rounded-2xl font-black uppercase tracking-widest shadow-2xl shadow-primary/20"
            >
              {isSubmitting && (
                <HugeiconsIcon
                  icon={Loading03Icon}
                  className="mr-2 h-4 w-4 animate-spin"
                />
              )}
              Apply Batch Changes
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
