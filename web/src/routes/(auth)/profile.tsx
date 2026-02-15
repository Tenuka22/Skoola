import { Link, createFileRoute } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import { Loading03Icon } from '@hugeicons/core-free-icons'
import { useMutation } from '@tanstack/react-query'
import {
  getActiveSessionServer,
  getAuthStorageServer,
} from '@/lib/auth/session'
import { AccountSwitcher } from '@/features/auth/components/account-switcher'
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
import { logoutFn } from '@/lib/auth/actions'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'
import { getUserPermissions } from '@/lib/api/sdk.gen'
import { authClient } from '@/lib/clients'
import { Badge } from '@/components/ui/badge'

export const Route = createFileRoute('/(auth)/profile')({
  component: ProfilePage,
  loader: async () => {
    try {
      const activeSession = await getActiveSessionServer()
      const authStorage = await getAuthStorageServer()

      const otherSessions = authStorage
        ? Object.values(authStorage.sessions).filter(
            (s) => s.user.id !== activeSession?.user.id,
          )
        : []

      if (activeSession?.user.id) {
        try {
          const userPermissionsRes = await getUserPermissions({
            path: { user_id: activeSession?.user.id },
            client: authClient,
          })
          const processedPermissions = userPermissionsRes.data
            ? userPermissionsRes.data.split(',').filter((s) => s.trim() !== '')
            : []
          return {
            activeSession,
            otherSessions,
            authStorage,
            userPermissions: processedPermissions,
          }
        } catch (e) {
          console.error('Failed to load user permissions:', e)

          return {
            activeSession,
            otherSessions,
            authStorage,
            userPermissions: [],
          }
        }
      }
      return { activeSession, otherSessions, authStorage, userPermissions: [] }
    } catch (e) {
      console.error('Failed to load admin route data:', e)
      return {
        activeSession: null,
        authStorage: null,
        otherSessions: [],
        userPermissions: [],
      }
    }
  },
})

function ProfilePage() {
  const data = Route.useLoaderData()

  const activeSession = data.activeSession ?? null
  const otherSessions = data.otherSessions ?? []
  const authStorage = data.authStorage ?? []
  const permissions = data.userPermissions ?? []

  const { mutateAsync: handleLogoutUser, isPending } = useMutation({
    mutationFn: async () => {
      await logoutFn()
      window.location.reload()
    },
  })

  if (!activeSession || !authStorage)
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
            <AvatarImage src={undefined} alt={activeSession.user.email} />
            <AvatarFallback>
              {activeSession.user.email.substring(0, 2) || 'US'}
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <CardTitle>{'User Profile'}</CardTitle>
            <CardDescription>{activeSession.user.email}</CardDescription>
          </div>
        </CardHeader>
        <CardContent className="grid gap-2">
          <div className="text-sm text-muted-foreground">
            <strong>ID:</strong> {activeSession.user.id}
          </div>
          {activeSession.user.roles && activeSession.user.roles.length > 0 && (
            <div className="text-sm text-muted-foreground">
              <strong>Roles:</strong>{' '}
              {activeSession?.user.roles.map((role, index) => (
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
                  <Badge key={permission} variant="outline">
                    {permission}
                  </Badge>
                ))}
              </div>
            </div>
          )}
        </CardContent>
        <CardFooter className="flex flex-col gap-2">
          <AccountSwitcher
            otherSessions={otherSessions}
            className="w-full justify-start gap-2"
            buttonVariant="outline"
          />

          <Button
            variant="destructive"
            className="w-full justify-start gap-2"
            onClick={async () => await handleLogoutUser()}
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
