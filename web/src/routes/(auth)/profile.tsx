import { createFileRoute, redirect, useNavigate } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import { LogoutIcon, UserIcon } from '@hugeicons/core-free-icons'
import {
  getActiveSession,
  getAuthStorage,
  removeSession,
  switchUser,
} from '@/lib/auth/session'
import { postAuthLogout5D5C18E2301F7F66A8222C30Cd9230A0 as logoutApi } from '@/lib/api/sdk.gen'
import { authClient } from '@/lib/clients'
import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

export const Route = createFileRoute('/(auth)/profile')({
  beforeLoad: ({ location }) => {
    const session = getActiveSession()
    if (!session) {
      throw redirect({
        to: '/login',
        search: {
          redirect: location.href,
        },
      })
    }
  },
  component: ProfilePage,
})

function ProfilePage() {
  const navigate = useNavigate()
  const session = getActiveSession()
  const storage = getAuthStorage()
  const otherSessions = Object.values(storage.sessions).filter(
    (s) => s.user.id !== session?.user.id,
  )

  const handleLogout = async () => {
    try {
      await logoutApi({
        client: authClient,
        body: {
          refresh_token: session?.refreshToken || '',
        },
      })
    } catch (e) {
      console.error('Logout failed', e)
    } finally {
      removeSession()
      navigate({ to: '/login' })
    }
  }

  const handleSwitchUser = (userId: string) => {
    switchUser(userId)
    window.location.reload() // Reload to apply new session to global client/state
  }

  if (!session) return null

  return (
    <div className="flex min-h-screen w-full items-center justify-center p-4">
      <Card className="w-full max-w-sm">
        <CardHeader className="flex flex-row items-center gap-4">
          <Avatar className="h-16 w-16">
            <AvatarImage src={session.user.photo_url} alt={session.user.name} />
            <AvatarFallback>
              {session.user.name?.charAt(0) || 'U'}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <CardTitle>{session.user.name || 'User'}</CardTitle>
            <CardDescription>{session.user.email}</CardDescription>
          </div>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="text-sm text-muted-foreground">
            Role: {session.user.role || 'User'}
          </div>
          <div className="text-sm text-muted-foreground">
            ID: {session.user.id}
          </div>
        </CardContent>
        <CardFooter className="flex flex-col gap-2">
          {otherSessions.length > 0 && (
            <DropdownMenu>
              <DropdownMenuTrigger render={<Button
                variant="outline"
                className="w-full justify-start gap-2"
              >
                <HugeiconsIcon icon={UserIcon} className="h-4 w-4" />
                Switch Account
              </Button>} />

              <DropdownMenuContent
                align="end"
                className="w-[var(--radix-dropdown-menu-trigger-width)]"
              >
                <DropdownMenuLabel>Accounts</DropdownMenuLabel>
                <DropdownMenuSeparator />
                {otherSessions.map((s) => (
                  <DropdownMenuItem
                    key={s.user.id}
                    onClick={() => handleSwitchUser(s.user.id)}
                  >
                    <div className="flex flex-col">
                      <span className="font-medium">{s.user.name}</span>
                      <span className="text-xs text-muted-foreground">
                        {s.user.email}
                      </span>
                    </div>
                  </DropdownMenuItem>
                ))}
                <DropdownMenuSeparator />
                <DropdownMenuItem onClick={() => navigate({ to: '/login' })}>
                  Add another account
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          )}

          <Button
            variant="destructive"
            className="w-full justify-start gap-2"
            onClick={handleLogout}
          >
            <HugeiconsIcon icon={LogoutIcon} className="h-4 w-4" />
            Sign Out
          </Button>
        </CardFooter>
      </Card>
    </div>
  )
}
