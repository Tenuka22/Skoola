import { createFileRoute } from '@tanstack/react-router'
import {
  Layers01Icon,
  Shield01Icon,
  UserGroupIcon,
} from '@hugeicons/core-free-icons'
import { HugeiconsIcon } from '@hugeicons/react'
import { useRBACStore } from '../../features/rbac/store'
import { RBACHeader } from '../../features/rbac/components/rbac-header'
import { UsersTab } from '../../features/rbac/components/users-tab'
import { RolesTab } from '../../features/rbac/components/roles-tab'
import { PermissionSetsTab } from '../../features/rbac/components/permission-sets-tab'
import { RoleEditorDialog } from '../../features/rbac/components/role-editor-dialog'
import { isRBACActiveTab } from '@/features/rbac/utils/permissions'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Box, HStack, Stack, Text } from '@/components/primitives'

export const Route = createFileRoute('/admin/rbac')({
  component: RBACPage,
})

function RBACPage() {
  const { activeTab, setActiveTab } = useRBACStore()

  return (
    <Stack gap={0} className="h-full bg-background">
      <RBACHeader />

      <Tabs
        value={activeTab}
        onValueChange={(val) => {
          if (typeof val === 'string' && isRBACActiveTab(val)) {
            setActiveTab(val)
          }
        }}
        className="h-full flex flex-col"
      >
        <Box px={8} className="mb-4">
          <TabsList className="w-full sm:w-fit justify-start bg-transparent p-0 border-b rounded-none h-auto gap-6">
            <TabsTrigger
              value="users"
              className="data-[state=active]:bg-transparent data-[state=active]:shadow-none data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none px-0 pb-2"
            >
              <HStack gap={2}>
                <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
                <Text>Users</Text>
              </HStack>
            </TabsTrigger>
            <TabsTrigger
              value="roles"
              className="data-[state=active]:bg-transparent data-[state=active]:shadow-none data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none px-0 pb-2"
            >
              <HStack gap={2}>
                <HugeiconsIcon icon={Shield01Icon} className="size-4" />
                <Text>Roles</Text>
              </HStack>
            </TabsTrigger>
            <TabsTrigger
              value="permission-sets"
              className="data-[state=active]:bg-transparent data-[state=active]:shadow-none data-[state=active]:border-b-2 data-[state=active]:border-primary rounded-none px-0 pb-2"
            >
              <HStack gap={2}>
                <HugeiconsIcon icon={Layers01Icon} className="size-4" />
                <Text>Permission Sets</Text>
              </HStack>
            </TabsTrigger>
          </TabsList>
        </Box>

        <Box px={8} py={4} className="flex-1 overflow-hidden">
          <TabsContent
            value="users"
            className="h-full mt-0 focus-visible:outline-none"
          >
            <UsersTab />
          </TabsContent>
          <TabsContent
            value="roles"
            className="h-full mt-0 focus-visible:outline-none"
          >
            <RolesTab />
          </TabsContent>
          <TabsContent
            value="permission-sets"
            className="h-full mt-0 focus-visible:outline-none"
          >
            <PermissionSetsTab />
          </TabsContent>
        </Box>
      </Tabs>
      <RoleEditorDialog />
    </Stack>
  )
}
