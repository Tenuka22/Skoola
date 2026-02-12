import * as React from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon, UserAdd01Icon } from '@hugeicons/core-free-icons'
import { signUpSchema } from '../../auth/schemas'
import type { SignUpFormValues } from '../../auth/schemas'
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
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'

interface UserCreateDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onConfirm: (data: SignUpFormValues) => void
  isSubmitting?: boolean
}

export function UserCreateDialog({
  open,
  onOpenChange,
  onConfirm,
  isSubmitting,
}: UserCreateDialogProps) {
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<SignUpFormValues>({
    resolver: zodResolver(signUpSchema),
  })

  React.useEffect(() => {
    if (open) {
      reset()
    }
  }, [open, reset])

  const onSubmit = (data: SignUpFormValues) => {
    onConfirm(data)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-md rounded-[2.5rem] border-none p-10 shadow-2xl backdrop-blur-3xl ring-1 ring-white/20">
        <DialogHeader>
          <div className="mx-auto mb-4 flex size-20 items-center justify-center rounded-3xl bg-primary/10 text-primary">
            <HugeiconsIcon icon={UserAdd01Icon} className="size-10" />
          </div>
          <DialogTitle className="text-center text-3xl font-black tracking-tight">
            Create User
          </DialogTitle>
          <DialogDescription className="text-center text-base font-medium leading-relaxed opacity-70">
            Add a new user to the organization.
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className="mt-8 space-y-6">
          <FieldGroup className="space-y-4">
            <Field>
              <Label htmlFor="name" className="text-xs font-black uppercase tracking-widest opacity-50">
                Full Name
              </Label>
              <Input
                id="name"
                {...register('name')}
                placeholder="John Doe"
                className="h-12 rounded-xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary mt-1"
              />
              <FieldError errors={[errors.name]} />
            </Field>

            <Field>
              <Label htmlFor="email" className="text-xs font-black uppercase tracking-widest opacity-50">
                Email Address
              </Label>
              <Input
                id="email"
                {...register('email')}
                placeholder="john@example.com"
                className="h-12 rounded-xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary mt-1"
              />
              <FieldError errors={[errors.email]} />
            </Field>

            <Field>
              <Label htmlFor="password" className="text-xs font-black uppercase tracking-widest opacity-50">
                Password
              </Label>
              <Input
                id="password"
                type="password"
                {...register('password')}
                className="h-12 rounded-xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary mt-1"
              />
              <FieldError errors={[errors.password]} />
            </Field>

            <Field>
              <Label htmlFor="confirmPassword" className="text-xs font-black uppercase tracking-widest opacity-50">
                Confirm Password
              </Label>
              <Input
                id="confirmPassword"
                type="password"
                {...register('confirmPassword')}
                className="h-12 rounded-xl border-none bg-muted/30 px-4 font-bold focus-visible:ring-2 focus-visible:ring-primary mt-1"
              />
              <FieldError errors={[errors.confirmPassword]} />
            </Field>
          </FieldGroup>

          <DialogFooter className="mt-8 sm:justify-center gap-3 border-t pt-6">
            <Button
              type="button"
              variant="ghost"
              onClick={() => onOpenChange(false)}
              className="h-12 min-w-[100px] rounded-xl font-black uppercase tracking-widest"
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={isSubmitting}
              className="h-12 min-w-[200px] rounded-xl font-black uppercase tracking-widest shadow-xl shadow-primary/20"
            >
              {isSubmitting && (
                <HugeiconsIcon
                  icon={Loading03Icon}
                  className="mr-2 h-4 w-4 animate-spin"
                />
              )}
              Create Account
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}
