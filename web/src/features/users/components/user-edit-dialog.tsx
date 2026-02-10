import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon, PencilEdit01Icon } from '@hugeicons/core-free-icons'
import { RoleEnumSchema } from '@/lib/api/schemas.gen'
import { updateUserSchema, type UpdateUserValues } from '../schemas'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel,
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Checkbox } from '@/components/ui/checkbox'



import { Badge } from '@/components/ui/badge'
import type { User } from '../types'

interface UserEditDialogProps {
  user: User | null
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: UpdateUserValues) => void
  isSubmitting?: boolean
}

export function UserEditDialog({
  user,
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: UserEditDialogProps) {
  // Using RoleEnumSchema to define available roles directly.
  const availableRoles = {
    data: RoleEnumSchema.enum.map((roleName) => ({
      id: roleName, // Using role name as ID for consistency in UI
      name: roleName,
    })),
  }

  const {
    register,
    handleSubmit,
    setValue,
    watch,
    reset,
    formState: { errors },
  } = useForm<UpdateUserValues>({
    resolver: zodResolver(updateUserSchema),
  })

  React.useEffect(() => {
    if (open && user) {
      reset({
        email: user.email,
        is_verified: user.is_verified,
        is_locked: false, // We don't have current lock status in UserResponse easily, assume false for toggle
        roles: [], // We'd need to fetch current roles for a perfect pre-fill, but let's assume empty for now or fetch them
      })
    }
  }, [open, user, reset])

  const onSubmit = (data: UpdateUserValues) => {
    onConfirm(data)
  }

  const isVerified = watch('is_verified')
  const isLocked = watch('is_locked')
  const selectedRoles = watch('roles') || []

  const toggleRole = (roleName: string) => {
    const current = selectedRoles
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
            Edit User Account
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
            Modify identity parameters for {user?.email}. Changes are applied
            immediately.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className="mt-8 space-y-8">
          <FieldGroup className="space-y-6">
            <Field>
              <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
                Email Address
              </FieldLabel>
              <Input
                {...register('email')}
                className="h-14 rounded-2xl border-none bg-muted/30 px-6 font-bold focus-visible:ring-2 focus-visible:ring-primary"
              />
              <FieldError errors={[errors.email]} />
            </Field>

            <div className="grid grid-cols-2 gap-6">
              <FieldGroup className="space-y-4">
                <FieldLabel className="text-xs font-black uppercase tracking-widest opacity-50">
                  Status Controls
                </FieldLabel>
                <Field className="flex items-center justify-between rounded-2xl bg-muted/30 p-4 transition-colors hover:bg-muted/50">
                  <div className="space-y-0.5">
                    <FieldLabel className="text-sm font-bold">
                      Verification
                    </FieldLabel>
                    <p className="text-[10px] font-medium opacity-50">
                      Identity verified
                    </p>
                  </div>
                  <Switch
                    checked={isVerified}
                    onCheckedChange={(checked) =>
                      setValue('is_verified', checked)
                    }
                  />
                </Field>

                <Field className="flex items-center justify-between rounded-2xl bg-muted/30 p-4 transition-colors hover:bg-muted/50">
                  <div className="space-y-0.5">
                    <FieldLabel className="text-sm font-bold">
                      Lockout
                    </FieldLabel>
                    <p className="text-[10px] font-medium opacity-50">
                      Restrict access
                    </p>
                  </div>
                  <Switch
                    checked={isLocked}
                    onCheckedChange={(checked) =>
                      setValue('is_locked', checked)
                    }
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
                                    {selectedRoles.length} Selected
                                  </Badge>
                                )}
                              </div>
                              <div className="grid grid-cols-1 gap-2 max-h-[200px] overflow-y-auto pr-2 custom-scrollbar">
                                {availableRoles?.data.map((role) => (
                                  <div
                                    key={role.id}
                                    onClick={() => toggleRole(role.name)}
                                    className="flex cursor-pointer items-center gap-3 rounded-xl border border-transparent bg-muted/20 p-3 transition-all hover:bg-muted/40 hover:ring-1 hover:ring-primary/20"
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
                            </FieldGroup>            </div>
          </FieldGroup>

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
              disabled={isSubmitting}
              className="h-14 min-w-[240px] rounded-2xl font-black uppercase tracking-widest shadow-2xl shadow-primary/20"
            >
              {isSubmitting && (
                <HugeiconsIcon
                  icon={Loading03Icon}
                  className="mr-2 h-4 w-4 animate-spin"
                />
              )}
              Save Identity Changes
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
