import { useNavigate } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import { UserIcon } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import { switchUserServer } from '@/lib/auth/session'
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Spinner } from '@/components/ui/spinner'

interface AccountSwitcherProps {
  otherSessions: Array<{ user: { id: string; email: string } }>
  className?: string
  buttonVariant?:
    | 'default'
    | 'destructive'
    | 'outline'
    | 'secondary'
    | 'ghost'
    | 'link'
  align?: 'start' | 'center' | 'end'
}

export function AccountSwitcher({
  otherSessions,
  className,
  buttonVariant = 'outline',
  align = 'end',
}: AccountSwitcherProps) {
  const navigate = useNavigate()
  const { mutateAsync: handleSwitchUser, isPending } = useMutation({
    mutationFn: async (userId: string) => {
      await switchUserServer({ data: userId })
      window.location.reload()
    },
  })

  if (otherSessions.length === 0) return null

  return (
    <DropdownMenu>
      <DropdownMenuTrigger
        render={
          <Button
            variant={buttonVariant}
            disabled={isPending}
            className={className}
          >
            {isPending ? (
              <Spinner />
            ) : (
              <HugeiconsIcon icon={UserIcon} className="h-4 w-4" />
            )}
            Switch Account
          </Button>
        }
      />

      <DropdownMenuContent align={align}>
        <DropdownMenuGroup>
          <DropdownMenuLabel>Accounts</DropdownMenuLabel>
          <DropdownMenuSeparator />
          {otherSessions.map((s) => (
            <DropdownMenuItem
              key={s.user.id}
              disabled={isPending}
              onClick={() => handleSwitchUser(s.user.id)}
            >
              <div className="flex flex-col">{s.user.email}</div>
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem onClick={() => navigate({ to: '/login' })}>
            Add another account
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
