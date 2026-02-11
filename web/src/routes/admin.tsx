import { Link, Outlet, createFileRoute } from '@tanstack/react-router'
import * as React from 'react'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Calendar01Icon,
  Calendar02Icon,
  Home01Icon,
  Settings01Icon,
  Shield01Icon,
  UserGroupIcon,
  UserIcon,
} from '@hugeicons/core-free-icons'

import type { Session } from '@/lib/auth/session'
import {
  getActiveSessionServer,
  getAuthStorageServer,
  switchUserServer,
} from '@/lib/auth/session'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
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
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
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
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from '@/components/ui/empty'

export const Route = createFileRoute('/admin')({
  component: AdminLayout,
})

function AdminLayout() {
  const [session, setSession] = React.useState<Session | null>(null)
  const [otherSessions, setOtherSessions] = React.useState<Array<Session>>([])
  const [isLoading, setIsLoading] = React.useState(true)

  React.useEffect(() => {
    const fetchData = async () => {
      try {
        const activeSession = await getActiveSessionServer()
        setSession(activeSession)
        const authStorage = await getAuthStorageServer()
        if (authStorage && activeSession) {
          const others = Object.values(authStorage.sessions).filter(
            (s) => s.user.id !== activeSession.user.id,
          )
          setOtherSessions(others)
        }
      } catch (e) {
        console.error('Failed to fetch session data:', e)
      } finally {
        setIsLoading(false)
      }
    }
    fetchData()
  }, [])

  const handleSwitchUser = async (userId: string) => {
    await switchUserServer({ data: userId })
    window.location.reload()
  }

  if (isLoading) {
    return (
      <div className="flex min-h-screen w-full items-center justify-center p-4">
        Loading...
      </div>
    )
  }

  if (!session || !session.user.roles.includes('FullAdmin')) {
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
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </EmptyContent>
        </Empty>
      </div>
    )
  }
  return (
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader>
          <div className="flex items-center gap-4 px-2">
            <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
              <span className="text-lg font-bold">S</span>
            </div>
            <div className="flex flex-col gap-0.5 leading-none">
              <span className="font-semibold">Skoola</span>
              <span className="text-xs text-muted-foreground">v1.0.0</span>
            </div>
          </div>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Platform</SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Dashboard"
                  render={<Link to="/admin" />}
                >
                  <HugeiconsIcon icon={Home01Icon} />
                  <span>Dashboard</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Users"
                  render={<Link to="/admin/users" />}
                >
                  <HugeiconsIcon icon={UserGroupIcon} />
                  <span>Users</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Permissions"
                  render={<Link to="/admin/permissions" />}
                >
                  <HugeiconsIcon icon={Shield01Icon} />
                  <span>Permissions</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Settings"
                  render={<Link to="/admin/settings" />}
                >
                  <HugeiconsIcon icon={Settings01Icon} />
                  <span>Settings</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Attendance</SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Staff Attendance"
                  render={<Link to="/admin/attendance/staff" />}
                >
                  <HugeiconsIcon icon={Calendar01Icon} />
                  <span>Staff</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <SidebarMenuButton
                  tooltip="Student Attendance"
                  render={<Link to="/admin/attendance/students" />}
                >
                  <HugeiconsIcon icon={Calendar02Icon} />
                  <span>Students</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>{/* User menu placeholder */}</SidebarFooter>
        <SidebarRail />
      </Sidebar>
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
          <div className="flex items-center gap-4 px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2 h-4" />
            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem className="hidden md:block">
                  <BreadcrumbLink render={<Link to="/admin" />}>
                    Admin
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator className="hidden md:block" />
                <BreadcrumbItem>
                  <BreadcrumbPage>Dashboard</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </div>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4 pt-0">
          <Outlet />
        </div>
      </SidebarInset>
    </SidebarProvider>
  )
}
