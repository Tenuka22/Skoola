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
import { ThemeToggle } from '@/components/root/theme-toggle'

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
      <Sidebar collapsible="icon" className="border-r border-border/50">
        <SidebarHeader className="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex justify-center items-start h-16 border-b border-border/40 px-4">
          <HStack gap={3}>
            <Box className="bg-primary text-primary-foreground flex aspect-square size-8 items-center justify-center rounded-md shadow-sm">
              <HugeiconsIcon icon={Book01Icon} size={18} />
            </Box>
            <Stack
              gap={0}
              className="flex-1 text-left leading-tight group-has-data-[collapsible=icon]/sidebar-wrapper:hidden"
            >
              <Text className="truncate font-semibold tracking-tight">Skoola</Text>
              <Text size="xs" className="truncate text-muted-foreground font-medium">
                Admin Console
              </Text>
            </Stack>
          </HStack>
        </SidebarHeader>
        <SidebarContent className="px-2 pt-2">
          <SidebarGroup>
            <SidebarGroupLabel className="text-[11px] font-bold uppercase tracking-wider text-muted-foreground/70 px-2 mb-1">
              Platform
            </SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin">
                  <SidebarMenuButton tooltip="Dashboard" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={Home01Icon} size={18} />
                    <span className="font-medium">Dashboard</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/users">
                  <SidebarMenuButton tooltip="Users" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={UserGroupIcon} size={18} />
                    <span className="font-medium">Users</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/rbac">
                  <SidebarMenuButton tooltip="Access Control" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={Shield01Icon} size={18} />
                    <span className="font-medium">Access Control</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup className="mt-2">
            <SidebarGroupLabel className="text-[11px] font-bold uppercase tracking-wider text-muted-foreground/70 px-2 mb-1">
              Academics
            </SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin/students">
                  <SidebarMenuButton tooltip="Students" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={User02Icon} size={18} />
                    <span className="font-medium">Students</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/staff">
                  <SidebarMenuButton tooltip="Staff" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={Briefcase01Icon} size={18} />
                    <span className="font-medium">Staff</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
          <SidebarGroup className="mt-2">
            <SidebarGroupLabel className="text-[11px] font-bold uppercase tracking-wider text-muted-foreground/70 px-2 mb-1">
              Attendance
            </SidebarGroupLabel>
            <SidebarMenu>
              <SidebarMenuItem>
                <Link to="/admin/attendance/staff">
                  <SidebarMenuButton tooltip="Staff Attendance" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={Calendar01Icon} size={18} />
                    <span className="font-medium">Staff</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
              <SidebarMenuItem>
                <Link to="/admin/attendance/students">
                  <SidebarMenuButton tooltip="Student Attendance" className="hover:bg-accent/50 transition-colors">
                    <HugeiconsIcon icon={Calendar02Icon} size={18} />
                    <span className="font-medium">Students</span>
                  </SidebarMenuButton>
                </Link>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroup>
        </SidebarContent>
        <SidebarRail />
      </Sidebar>
      <SidebarInset className="bg-background">
        <HStack
          as="header"
          gap={2}
          p={0}
          className="px-6 border-b border-border/40 bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60 sticky top-0 z-10 h-16 shrink-0 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12"
        >
          <HStack gap={4} className="flex-1">
            <SidebarTrigger className="-ml-2 hover:bg-accent/50" />
            <Separator orientation="vertical" className="h-6" />
            <Box className="flex-1 px-2">
              {/* Optional breadcrumb or search could go here */}
            </Box>
            <ThemeToggle />
          </HStack>
        </HStack>
        <Stack gap={6} className="flex-1 p-10 bg-muted/20">
          <Outlet />
        </Stack>
      </SidebarInset>
    </SidebarProvider>
  )
}
