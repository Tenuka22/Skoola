import { Link, Outlet, createFileRoute } from '@tanstack/react-router'
import { HugeiconsIcon } from '@hugeicons/react'
import {
  Book01Icon,
  Briefcase01Icon,
  Calendar01Icon,
  Calendar02Icon,
  Home01Icon,
  Shield01Icon,
  User02Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import type { RoleEnum } from '@/lib/api/types.gen'
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

import { AccountSwitcher } from '@/features/auth/components/account-switcher'
import { Box, HStack, Stack, Text } from '@/components/primitives'

const ADMIN_ROLES: Array<RoleEnum> = [
  'Admin',
  'FullAdmin',
  'Principal',
  'VicePrincipal',
]

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
    !ADMIN_ROLES.some((role) => activeSession.user.roles.includes(role))
  ) {
    return (
      <Box className="flex min-h-screen w-full items-center justify-center bg-muted/40 p-4">
        <Empty className="max-w-sm border-border bg-background">
          <EmptyHeader>
            <EmptyTitle>Admin Access Required</EmptyTitle>
            <EmptyDescription>
              You must be an administrator to view this page.
            </EmptyDescription>
          </EmptyHeader>
          <EmptyContent>
            <Stack gap={2} className="w-full">
              <Button
                render={<Link to="/login">Login with another account</Link>}
              />
              <AccountSwitcher
                otherSessions={otherSessions}
                className="w-full justify-start gap-2"
                buttonVariant="outline"
              />
            </Stack>
          </EmptyContent>
        </Empty>
      </Box>
    )
  }
  return (
    <SidebarProvider>
      <Sidebar collapsible="icon">
        <SidebarHeader className="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex justify-center items-start h-16">
          <HStack gap={4}>
            <Box className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-7 items-center justify-center rounded-lg">
              <HugeiconsIcon icon={Book01Icon} />
            </Box>
            <Stack
              gap={0}
              className="flex-1  text-left leading-tight group-has-data-[collapsible=icon]/sidebar-wrapper:hidden"
            >
              <Text className="truncate font-medium">Skoola</Text>
              <Text size="xs" className="truncate">
                Your edu platform.
              </Text>
            </Stack>
          </HStack>
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
                <Link to="/admin/rbac">
                  <SidebarMenuButton tooltip="Access Control">
                    <HugeiconsIcon icon={Shield01Icon} />
                    <span>Access Control</span>
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
        <HStack
          as="header"
          gap={2}
          p={0}
          className="p-2 border-b h-16 shrink-0 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
        >
          <HStack gap={2} className="px-4">
            <SidebarTrigger className="-ml-1" />
            <Separator orientation="vertical" className="mr-2" />
          </HStack>
        </HStack>
        <Stack gap={4} className="flex-1 p-4 pt-0">
          <Outlet />
        </Stack>
      </SidebarInset>
    </SidebarProvider>
  )
}
