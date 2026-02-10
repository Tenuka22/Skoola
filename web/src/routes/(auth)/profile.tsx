import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import * as React from 'react' // Import React
import { HugeiconsIcon } from '@hugeicons/react'
import { UserIcon, Loading03Icon } from '@hugeicons/core-free-icons' // Added Loading03Icon
import {
  getActiveSessionServer,
  getAuthStorageServer,
  switchUserServer,
} from '@/lib/auth/session'
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
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import type { Session, AuthStorage } from '@/lib/auth/session' // Import types
import { useMutation } from '@tanstack/react-query' // Add useMutation import
import { useServerFn } from '@tanstack/react-start' // New import
import { logoutFn } from '@/lib/auth/actions' // New import
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'

export const Route = createFileRoute('/(auth)/profile')({
  component: ProfilePage,
})

function ProfilePage() {
  const navigate = useNavigate()
  const [session, setSession] = React.useState<Session | null>(null)
  const [storage, setStorage] = React.useState<AuthStorage | null>(null)
  const [otherSessions, setOtherSessions] = React.useState<Array<Session>>([])

  // New: Use useServerFn for logout
  const logoutServerFn = useServerFn(logoutFn)
  const { mutate, isPending } = useMutation({
    mutationFn: logoutServerFn,
  })

  React.useEffect(() => {
    const fetchData = async () => {
      try {
        const activeSession = await getActiveSessionServer()
        setSession(activeSession)
        const authStorage = await getAuthStorageServer()
        setStorage(authStorage)
        if (authStorage && activeSession) {
          const others = Object.values(authStorage.sessions).filter(
            (s) => s.user.id !== activeSession.user.id,
          )
          setOtherSessions(others)
        }
      } catch (e) {
        console.error('Failed to fetch session data:', e)
      }
    }
    fetchData()
  }, [])

  const handleLogout = async () => {
    try {
      await mutate(undefined)
      // Redirection is handled by logoutFn
    } catch (e) {
      console.error('Logout failed in component', e)
      // Optionally display an error message to the user
    }
  }

  const handleSwitchUser = async (userId: string) => {
    // Made async
    await switchUserServer({ data: userId }) // Await the server function
    window.location.reload() // Reload to apply new session to global client/state
  }

  if (!session || !storage)
    return (
      <div className="flex min-h-screen w-full items-center justify-center p-4 bg-muted/40">
        <Empty className="max-w-sm bg-background border-border">
          <EmptyHeader>
            <EmptyTitle>User Not Found</EmptyTitle>
            <EmptyDescription>
              The page you are looking for needs you to be authenticated.
            </EmptyDescription>
          </EmptyHeader>
          <EmptyContent>
            <Button
              render={<Link to="/login">Go to Login</Link>}
              className="w-full"
            />
          </EmptyContent>
        </Empty>
      </div>
    ) // Render nothing until data is loaded

  return (
    <div className="flex min-h-screen w-full items-center justify-center p-4">
      <Card className="w-full max-w-sm">
        <CardHeader className="flex flex-row items-center gap-4">
          <Avatar className="h-16 w-16">
            <AvatarImage src={undefined} alt={session.user.email} />
            <AvatarFallback>
              {session.user.email?.substring(0, 2) || 'US'}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <CardTitle>{'User'}</CardTitle>
            <CardDescription>{session.user.email}</CardDescription>
          </div>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="text-sm text-muted-foreground">
            ID: {session.user.id}
          </div>
        </CardContent>
        <CardFooter className="flex flex-col gap-2">
          {otherSessions.length > 0 && (
            <DropdownMenu>
              <DropdownMenuTrigger
                render={
                  <Button
                    variant="outline"
                    className="w-full justify-start gap-2"
                  >
                    <HugeiconsIcon icon={UserIcon} className="h-4 w-4" />
                    Switch Account
                  </Button>
                }
              />

              <DropdownMenuContent
                align="end"
                className="w-[var(--radix-dropdown-menu-trigger-width)]"
              >
                <DropdownMenuGroup>
                  <DropdownMenuLabel>Accounts</DropdownMenuLabel>
                  <DropdownMenuSeparator />
                  {otherSessions.map((s) => (
                    <DropdownMenuItem
                      key={s.user.id}
                      onClick={() => handleSwitchUser(s.user.id)}
                    >
                      <div className="flex flex-col">
                        <span className="text-xs text-muted-foreground">
                          {s.user.email}
                        </span>
                      </div>
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
          )}

          <Button
            variant="destructive"
            className="w-full justify-start gap-2"
            onClick={handleLogout}
            disabled={isPending} // Disable button during logout
          >
            {isPending && (
              <HugeiconsIcon
                icon={Loading03Icon}
                className="mr-2 h-4 w-4 animate-spin"
              />
            )}
            Sign Out
          </Button>
        </CardFooter>
      </Card>
    </div>
  )
}
