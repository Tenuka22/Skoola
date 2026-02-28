import { createFileRoute } from '@tanstack/react-router'
import {
  HierarchyIcon,
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
import { RoleSetsTab } from '../../features/rbac/components/role-sets-tab'
import { RoleEditorDialog } from '../../features/rbac/components/role-editor-dialog'
import { isRBACActiveTab } from '@/features/rbac/utils/permissions'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { HStack, Stack, Text } from '@/components/primitives'

export const Route = createFileRoute('/admin/rbac')({
  component: RBACPage,
})

function RBACPage() {
  const { activeTab, setActiveTab } = useRBACStore()

  return (
    <Stack gap={4} p={8}>
      <RBACHeader />

      <Tabs
        value={activeTab}
        onValueChange={(val) => {
          if (typeof val === 'string' && isRBACActiveTab(val)) {
            setActiveTab(val)
          }
        }}
        className="gap-4"
      >
        <TabsList>
          <TabsTrigger value="users">
            <HStack gap={1}>
              <HugeiconsIcon icon={UserGroupIcon} className="size-4" />
              <Text>Users</Text>
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="roles">
            <HStack gap={1}>
              <HugeiconsIcon icon={Shield01Icon} className="size-4" />
              <Text>Roles</Text>
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="permission-sets">
            <HStack gap={1}>
              <HugeiconsIcon icon={Layers01Icon} className="size-4" />
              <Text>Permission Sets</Text>
            </HStack>
          </TabsTrigger>
          <TabsTrigger value="role-sets">
            <HStack gap={1}>
              <HugeiconsIcon icon={HierarchyIcon} className="size-4" />
              <Text>Role Sets</Text>
            </HStack>
          </TabsTrigger>
        </TabsList>

        <TabsContent value="users">
          <UsersTab />
        </TabsContent>
        <TabsContent value="roles">
          <RolesTab />
        </TabsContent>
        <TabsContent value="permission-sets">
          <PermissionSetsTab />
        </TabsContent>
        <TabsContent value="role-sets">
          <RoleSetsTab />
        </TabsContent>
      </Tabs>
      <RoleEditorDialog />
    </Stack>
  )
}
