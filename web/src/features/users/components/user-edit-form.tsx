import { zodResolver } from '@hookform/resolvers/zod'
import * as React from 'react'
import { useForm } from 'react-hook-form'
import { updateUserSchema } from '../schemas'
import type { z } from 'zod'
import type { UpdateUserValues } from '../schemas'
import type { User } from '../types'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { DialogFooter } from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { zRoleEnum } from '@/lib/api/zod.gen'
import { Spinner } from '@/components/ui/spinner'

interface UserEditFormProps {
  user: User
  onConfirm: (data: UpdateUserValues) => void
  onOpenChange: (open: boolean) => void
  isSubmitting?: boolean
}

export function UserEditForm({
  user,
  onConfirm,
  onOpenChange,
  isSubmitting,
}: UserEditFormProps) {
  const availableRoles = {
    data: Object.values(zRoleEnum.enum).map((roleName) => ({
      id: roleName,
      name: roleName,
    })),
  }
  // TODO add user roles get and check them
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
    if (user) {
      reset({
        email: user.email,
        is_verified: user.is_verified ?? false,
        roles: [],
      })
    }
  }, [user, reset])

  const onSubmit = (data: UpdateUserValues) => {
    onConfirm(data)
  }

  const isVerified = watch('is_verified')
  const selectedRoles = watch('roles') || []

  const toggleRole = (roleName: z.infer<typeof zRoleEnum>) => {
    const current = selectedRoles || []
    if (current.includes(roleName)) {
      setValue(
        'roles',
        current.filter((r: z.infer<typeof zRoleEnum>) => r !== roleName),
      )
    } else {
      setValue('roles', [...current, roleName])
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 pt-4">
      <div className="space-y-2">
        <Label>Email Address</Label>
        <Input {...register('email')} placeholder="email@example.com" />
        {errors.email && (
          <p className="text-sm text-destructive">{errors.email.message}</p>
        )}
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-4">
          <Label>Status Controls</Label>
          <div className="flex items-center justify-between py-2">
            <div className="space-y-0.5">
              <Label className="text-sm">Verification</Label>
              <p className="text-xs text-muted-foreground">Identity verified</p>
            </div>
            <Switch
              checked={isVerified ?? false}
              onCheckedChange={(checked) => setValue('is_verified', checked)}
            />
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <Label>Access Roles</Label>
            {selectedRoles.length > 0 && (
              <Badge variant="secondary">{selectedRoles.length} Selected</Badge>
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

      <DialogFooter>
        <Button
          type="button"
          variant="outline"
          onClick={() => onOpenChange(false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={isSubmitting}>
          {isSubmitting && <Spinner className="mr-2 h-4 w-4" />}
          Save Changes
        </Button>
      </DialogFooter>
    </form>
  )
}
