import { Link, createFileRoute, useNavigate } from '@tanstack/react-router'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon, UserIcon } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import { useServerFn } from '@tanstack/react-start'
import type { AuthStorage, Session } from '@/lib/auth/session'
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
import { logoutFn } from '@/lib/auth/actions'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'
import {
  getUsersF4D0D9F0Ef0F26C7129Bc0A687Bdd92C as getUserPermissionsApi,
} from '@/lib/api/sdk.gen' // Import the API function
import { authClient } from '@/lib/clients' // Import authClient
import { Badge } from '@/components/ui/badge' // Assuming a Badge component exists for displaying permissions

export const Route = createFileRoute('/(auth)/profile')({
  component: ProfilePage,
})

interface UserPermission {
  id: number // Corrected type to number
  name: string
  // Add other properties if available in the API response for permissions
}

// Type predicate to validate if an item is a UserPermission
function isUserPermission(item: any): item is UserPermission {
  return (
    typeof item === 'object' &&
    item !== null &&
    'id' in item &&
    typeof item.id === 'number' && // Check for number type
    'name' in item &&
    typeof item.name === 'string'
  );
}

function ProfilePage() {
  const navigate = useNavigate()
  const [session, setSession] = React.useState<Session | null>(null)
  const [storage, setStorage] = React.useState<AuthStorage | null>(null)
  const [otherSessions, setOtherSessions] = React.useState<Array<Session>>([])
  const [permissions, setPermissions] = React.useState<Array<UserPermission>>([]) // State for permissions

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

          // Fetch permissions if user is logged in
          if (activeSession.user?.id) {
            const userPermissionsResponse = await getUserPermissionsApi({
              path: { user_id: activeSession.user.id }, // Correctly pass user_id as a path parameter
              client: authClient,
            })
            // userPermissionsResponse.data is directly the array of permissions
            if (userPermissionsResponse.data && Array.isArray(userPermissionsResponse.data)) {
              // Use the type predicate to filter and narrow the type
              const validatedPermissions = userPermissionsResponse.data.filter(isUserPermission);
              setPermissions(validatedPermissions);
            } else {
              setPermissions([]);
            }
          }
        }
      } catch (e) {
        console.error('Failed to fetch session or permissions data:', e)
        setPermissions([]);
      }
    }
    fetchData()
  }, [])

  const handleLogout = async () => {
    try {
      await mutate(undefined)
    } catch (e) {
      console.error('Logout failed in component', e)
    }
  }

  const handleSwitchUser = async (userId: string) => {
    await switchUserServer({ data: userId })
    window.location.reload()
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
    )

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
            <CardTitle>{'User Profile'}</CardTitle>
            <CardDescription>{session.user.email}</CardDescription>
          </div>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="text-sm text-muted-foreground">
            <strong>ID:</strong> {session.user.id}
          </div>
          {session.user.roles && session.user.roles.length > 0 && (
            <div className="text-sm text-muted-foreground">
              <strong>Roles:</strong>{' '}
              {session.user.roles.map((role, index) => (
                <Badge key={index} variant="secondary" className="mr-1">
                  {role}
                </Badge>
              ))}
            </div>
          )}
          {permissions.length > 0 && (
            <div className="text-sm text-muted-foreground">
              <strong>Permissions:</strong>
              <div className="flex flex-wrap gap-2 mt-2">
                {permissions.map((permission) => (
                  <Badge key={permission.id} variant="outline">
                    {permission.name}
                  </Badge>
                ))}
              </div>
            </div>
          )}
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
            disabled={isPending}
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