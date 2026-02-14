import { Link, Outlet, createFileRoute } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Book01Icon,
  Calendar01Icon,
  Calendar02Icon,
  Home01Icon,
  Settings01Icon,
  Shield01Icon,
  UserGroupIcon,
  User02Icon,
  Briefcase01Icon,
} from '@hugeicons/core-free-icons'
import {
  getActiveSessionServer,
  getAuthStorageServer,
} from '@/lib/auth/session'
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { Separator } from '@/components/ui/separator'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'
import { AdminRoutesAllowedRoles } from '@/features/permissions/constants'
import { AccountSwitcher } from '@/features/auth/components/account-switcher'

export const Route = createFileRoute('/admin')({
  component: AdminLayout,
  loader: async () => {
    try {
      const activeSession = await getActiveSessionServer()
      const authStorage = await getAuthStorageServer()

      const otherSessions = authStorage
        ? Object.values(authStorage.sessions).filter(
            (s) => s.user.id !== activeSession?.user.id,
          )
        : []

      return { activeSession, otherSessions }
    } catch (e) {
      console.error('Failed to load admin route data:', e)
      return { activeSession: null, otherSessions: [] }
    }
  },
})

function AdminLayout() {
  const data = Route.useLoaderData()

  const activeSession = data.activeSession ?? null
  const otherSessions = data.otherSessions ?? []

  if (
    !activeSession ||
    !activeSession.user.roles.some((role) =>
      AdminRoutesAllowedRoles.includes(role),
    )
  ) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center bg-muted/40 p-4">
        <Empty className="max-w-sm border-border bg-background">
          <EmptyHeader>
            <EmptyTitle>Admin Access Required</EmptyTitle>
            <EmptyDescription>
              You must be an administrator to view this page.
            </EmptyDescription>
          </EmptyHeader>
          <EmptyContent className="grid w-full gap-2">
            <Button
              render={<Link to="/login">Login with another account</Link>}
            />
            <AccountSwitcher
              otherSessions={otherSessions}
              className="w-full justify-start gap-2"
              buttonVariant="outline"
            />
          </EmptyContent>
        </Empty>
      </div>
    )
  }
  return (
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader className="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex justify-center items-start h-16">
          <div className="flex items-center gap-4 ">
            <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
              <HugeiconsIcon icon={Book01Icon} />
            </div>
            <div className="grid flex-1 text-left text-sm leading-tight">
              <span className="truncate font-medium">Skoola</span>{' '}
              <span className="truncate text-xs">Your edu platform.</span>
            </div>
          </div>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Platform</SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin">
                  <SidebarMenuButton tooltip="Dashboard">
                    <HugeiconsIcon icon={Home01Icon} />
                    <span>Dashboard</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/users">
                  <SidebarMenuButton tooltip="Users">
                    <HugeiconsIcon icon={UserGroupIcon} />
                    <span>Users</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/permissions">
                  <SidebarMenuButton tooltip="Permissions">
                    <HugeiconsIcon icon={Shield01Icon} />
                    <span>Permissions</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/settings">
                  <SidebarMenuButton tooltip="Settings">
                    <HugeiconsIcon icon={Settings01Icon} />
                    <span>Settings</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Academics</SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin/students">
                  <SidebarMenuButton tooltip="Students">
                    <HugeiconsIcon icon={User02Icon} />
                    <span>Students</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/staff">
                  <SidebarMenuButton tooltip="Staff">
                    <HugeiconsIcon icon={Briefcase01Icon} />
                    <span>Staff</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Attendance</SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin/attendance/staff">
                  <SidebarMenuButton tooltip="Staff Attendance">
                    <HugeiconsIcon icon={Calendar01Icon} />
                    <span>Staff</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/attendance/students">
                  <SidebarMenuButton tooltip="Student Attendance">
                    <HugeiconsIcon icon={Calendar02Icon} />
                    <span>Students</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
        </SidebarContent>
        <SidebarRail />
      </Sidebar>
      <SidebarInset>
        <header className="flex p-2 h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
          <div className="flex items-center gap-2 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2" />
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
          <Outlet />
        </div>
      </SidebarInset>
    </SidebarProvider>
  )
}
